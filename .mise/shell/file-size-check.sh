#!/usr/bin/env bash
set -euo pipefail

echo "Checking for Rust source files over 200 lines..."
echo ""

found_issues=0

while IFS= read -r file; do
    lines=$(wc -l < "$file" | tr -d ' ')
    if [ "$lines" -gt 200 ]; then
        echo "⚠️  $file ($lines lines) - consider refactoring into smaller modules"
        found_issues=1
    fi
done < <(find . -name "*.rs" -not -path "./target/*" -not -path "./.git/*")

if [ $found_issues -eq 0 ]; then
    echo "✅ No files exceed 200 lines"
    exit 0
else
    echo ""
    echo "⚠️  Found files over 200 lines. Consider breaking them into smaller, focused modules."
    exit 0  # Non-fatal warning
fi

