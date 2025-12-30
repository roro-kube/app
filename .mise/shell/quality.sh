#!/usr/bin/env bash
set -euo pipefail

echo "🔍 Running quality checks..."
echo ""

echo "📝 Formatting code..."
cargo fmt || {
    echo "❌ Formatting failed."
    exit 1
}

echo "✅ Code formatted"
echo ""

echo "📝 Verifying formatting..."
cargo fmt -- --check || {
    echo "❌ Formatting verification failed."
    exit 1
}

echo "✅ Formatting verified"
echo ""

echo "🔧 Running clippy linter..."
cargo clippy --all-targets --all-features -- -D warnings || {
    echo "❌ Clippy check failed. Fix warnings above."
    exit 1
}

echo "✅ Clippy check passed"
echo ""

echo "🧪 Running tests..."
cargo test || {
    echo "❌ Tests failed."
    exit 1
}

echo "✅ Tests passed"
echo ""

echo "📦 Running npm audits..."
bash .mise/shell/npm-audit.sh || {
    echo "❌ NPM audit check failed."
    exit 1
}

echo "✅ NPM audits passed"
echo ""

echo "📏 Checking file sizes..."
bash .mise/shell/file-size-check.sh || {
    echo "⚠️  File size check found issues (non-fatal)"
}

echo ""
echo "✅ All quality checks passed!"

