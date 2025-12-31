#!/usr/bin/env bash

# Get the script directory and project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
WEB_DOCS_DIR="$PROJECT_ROOT/web-docs"
BACKLOG_DOCS_DIR="$PROJECT_ROOT/backlog/docs"
BACKLOG_DECISIONS_DIR="$PROJECT_ROOT/backlog/decisions"
CONTENT_ROOT_DIR="$WEB_DOCS_DIR/content"
CONTENT_DOCS_DIR="$WEB_DOCS_DIR/content/docs"
CONTENT_DECISIONS_DIR="$WEB_DOCS_DIR/content/decisions"
GUI_ASSETS_DIR="$PROJECT_ROOT/gui/assets"
PUBLIC_DIR="$WEB_DOCS_DIR/public"
HOME_MDX_SOURCE="$WEB_DOCS_DIR/home.mdx"

echo "📄 Syncing markdown content to web-docs..."

# Create content directories if they don't exist
mkdir -p "$CONTENT_ROOT_DIR"
mkdir -p "$CONTENT_DOCS_DIR"
mkdir -p "$CONTENT_DECISIONS_DIR"

# Function to sanitize filename for URL-friendly format
sanitize_filename() {
  local filename="$1"
  local basename=$(basename "$filename")
  
  # Remove .md extension temporarily
  local name="${basename%.md}"
  
  # Remove "doc-" or "decision-" prefix (case-insensitive)
  name=$(echo "$name" | sed -E 's/^(doc|decision)-//i')
  
  # Replace spaces with hyphens
  name=$(echo "$name" | tr ' ' '-')
  
  # Remove all characters that are not alphanumeric or hyphens
  name=$(echo "$name" | sed 's/[^a-zA-Z0-9-]//g')
  
  # Collapse multiple consecutive hyphens into a single hyphen (repeat until no more changes)
  local prev_name=""
  while [ "$name" != "$prev_name" ]; do
    prev_name="$name"
    name=$(echo "$name" | sed 's/--/-/g')
  done
  
  # Remove leading and trailing hyphens
  name=$(echo "$name" | sed 's/^-*//; s/-*$//')
  
  # Ensure we have a number prefix pattern (e.g., 0001-) at the start
  # Extract the number prefix if it exists
  local number_prefix=""
  if [[ "$name" =~ ^([0-9]+)- ]]; then
    number_prefix="${BASH_REMATCH[1]}-"
    name="${name#${BASH_REMATCH[1]}-}"
  fi
  
  # Limit to 20 characters (base name only, excluding number prefix)
  local max_length=20
  local prefix_length=${#number_prefix}
  local available_length=$((max_length - prefix_length))
  
  if [ ${#name} -gt $available_length ]; then
    name="${name:0:$available_length}"
    # Remove trailing hyphen if truncation created one
    name=$(echo "$name" | sed 's/-*$//')
  fi
  
  # Reconstruct with number prefix
  if [ -n "$number_prefix" ]; then
    name="${number_prefix}${name}"
  fi
  
  # Re-add .md extension
  echo "${name}.md"
}

# Function to copy markdown files recursively and flatten them
copy_and_flatten_md() {
  local source_dir="$1"
  local dest_dir="$2"
  local file_count=0

  if [ ! -d "$source_dir" ]; then
    echo "⚠️  Source directory does not exist: $source_dir"
    return 0
  fi

  # Ensure destination directory exists
  mkdir -p "$dest_dir" || {
    echo "❌ Failed to create destination directory: $dest_dir"
    return 1
  }

  # Find all .md files recursively and copy them to the destination (flattened)
  # Use a temporary approach that's more compatible with strict error handling
  local temp_file
  temp_file=$(mktemp) || {
    echo "❌ Failed to create temporary file"
    return 1
  }
  
  # Find files and write to temp file, then process
  if ! find "$source_dir" -type f -name "*.md" > "$temp_file" 2>&1; then
    echo "❌ Error running find on $source_dir"
    cat "$temp_file"
    rm -f "$temp_file"
    return 1
  fi
  
  # Track used filenames to handle conflicts (using temp file for bash 3.2 compatibility)
  local used_filenames_file
  used_filenames_file=$(mktemp) || {
    echo "❌ Failed to create temporary file for tracking filenames"
    rm -f "$temp_file"
    return 1
  }
  
  # Process each file found
  while IFS= read -r file; do
    [ -z "$file" ] && continue
    [ ! -f "$file" ] && continue
    
    local original_filename=$(basename "$file")
    local sanitized_filename=$(sanitize_filename "$original_filename")
    
    # Handle filename conflicts
    local final_filename="$sanitized_filename"
    local counter=1
    while grep -q "^${final_filename}$" "$used_filenames_file" 2>/dev/null; do
      # If conflict, add a counter suffix before .md extension
      local base_name="${sanitized_filename%.md}"
      final_filename="${base_name}-${counter}.md"
      counter=$((counter + 1))
    done
    
    # Mark this filename as used
    echo "$final_filename" >> "$used_filenames_file"
    
    if ! cp "$file" "$dest_dir/$final_filename"; then
      echo "❌ Failed to copy $file to $dest_dir/$final_filename"
      rm -f "$temp_file" "$used_filenames_file"
      return 1
    fi
    file_count=$((file_count + 1))
  done < "$temp_file"
  
  rm -f "$temp_file" "$used_filenames_file"

  echo "✅ Copied $file_count markdown files from $source_dir to $dest_dir"
  return 0
}

# Function to copy assets recursively while preserving directory structure
copy_assets_preserve_structure() {
  local source_dir="$1"
  local dest_dir="$2"
  local file_count=0

  if [ ! -d "$source_dir" ]; then
    echo "⚠️  Source directory does not exist: $source_dir"
    return 0
  fi

  # Create destination directory if it doesn't exist
  mkdir -p "$dest_dir"

  # Count files before copying
  file_count=$(find "$source_dir" -type f | wc -l | tr -d ' ')

  # Copy recursively while preserving structure
  cp -r "$source_dir"/* "$dest_dir/" 2>/dev/null || {
    # Handle case where source_dir might be empty or have permission issues
    if [ -d "$source_dir" ] && [ "$(ls -A "$source_dir" 2>/dev/null)" ]; then
      cp -r "$source_dir"/* "$dest_dir/"
    else
      echo "⚠️  No files to copy from $source_dir"
      return 0
    fi
  }

  echo "✅ Copied $file_count asset files from $source_dir to $dest_dir (structure preserved)"
  return 0
}

# Copy home.mdx to content root as index.mdx
echo "🏠 Copying home.mdx to content root as index.mdx..."
if [ -f "$HOME_MDX_SOURCE" ]; then
  cp "$HOME_MDX_SOURCE" "$CONTENT_ROOT_DIR/index.mdx"
  echo "✅ Copied home.mdx to $CONTENT_ROOT_DIR/index.mdx"
else
  echo "⚠️  home.mdx not found at $HOME_MDX_SOURCE"
fi

# Copy docs (flattened)
echo "📚 Copying docs from backlog/docs..."
copy_and_flatten_md "$BACKLOG_DOCS_DIR" "$CONTENT_DOCS_DIR"

# Copy decisions (flattened)
echo "📋 Copying decisions from backlog/decisions..."
copy_and_flatten_md "$BACKLOG_DECISIONS_DIR" "$CONTENT_DECISIONS_DIR"

# Copy assets (preserving structure)
echo "🖼️  Copying assets from gui/assets..."
copy_assets_preserve_structure "$GUI_ASSETS_DIR" "$PUBLIC_DIR"

echo "✅ Content sync complete!"

