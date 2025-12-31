#!/usr/bin/env bash
set -euo pipefail

# Get the script directory and project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
WEB_DOCS_DIR="$PROJECT_ROOT/web-docs"
BACKLOG_DOCS_DIR="$PROJECT_ROOT/backlog/docs"
BACKLOG_DECISIONS_DIR="$PROJECT_ROOT/backlog/decisions"
CONTENT_DOCS_DIR="$WEB_DOCS_DIR/content/docs"
CONTENT_DECISIONS_DIR="$WEB_DOCS_DIR/content/decisions"
GUI_ASSETS_DIR="$PROJECT_ROOT/gui/assets"
PUBLIC_DIR="$WEB_DOCS_DIR/public"

echo "📄 Syncing markdown content to web-docs..."

# Create content directories if they don't exist
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

  # Find all .md files recursively and copy them to the destination (flattened)
  while IFS= read -r -d '' file; do
    local filename=$(basename "$file")
    cp "$file" "$dest_dir/$filename"
    ((file_count++))
  done < <(find "$source_dir" -type f -name "*.md" -print0)

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

