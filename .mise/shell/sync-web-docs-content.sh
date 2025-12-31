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
  
  # Process each file found
  while IFS= read -r file; do
    [ -z "$file" ] && continue
    [ ! -f "$file" ] && continue
    
    local filename=$(basename "$file")
    if ! cp "$file" "$dest_dir/$filename"; then
      echo "❌ Failed to copy $file to $dest_dir/$filename"
      rm -f "$temp_file"
      return 1
    fi
    ((file_count++))
  done < "$temp_file"
  
  rm -f "$temp_file"

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

