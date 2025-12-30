#!/usr/bin/env bash
set -euo pipefail

echo "Running npm audit for icons..."
cd icons && npm audit --production || {
    echo "⚠️  npm audit found issues in icons/"
    cd ..
    exit 1
}
cd ..

echo "Running npm audit for tailwind..."
cd tailwind && npm audit --production || {
    echo "⚠️  npm audit found issues in tailwind/"
    cd ..
    exit 1
}
cd ..

echo "✅ All npm audits passed"

