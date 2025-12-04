#!/usr/bin/env bash

set -euo pipefail

# PR Snapshot Script
# 
# This script captures before/after screenshots for PR comparisons.
# Usage: ./pr-snapshots.sh [PR_NUMBER]
#
# Environment variables:
#   BASE_URL - Base URL to capture (default: http://localhost:8787)
#   OUTPUT_DIR - Directory to save screenshots (default: ./screenshots)

PR_NUMBER="${1:-}"
BASE_URL="${BASE_URL:-http://localhost:8787}"
OUTPUT_DIR="${OUTPUT_DIR:-./screenshots}"
TIMESTAMP=$(date +%Y%m%d-%H%M%S)

if [ -z "$PR_NUMBER" ]; then
    echo "Usage: $0 <PR_NUMBER>"
    echo "Environment variables:"
    echo "  BASE_URL (default: $BASE_URL)"
    echo "  OUTPUT_DIR (default: $OUTPUT_DIR)"
    exit 1
fi

# Create output directories
mkdir -p "$OUTPUT_DIR/before"
mkdir -p "$OUTPUT_DIR/after"

echo "📸 Capturing screenshots for PR #$PR_NUMBER"
echo "Base URL: $BASE_URL"
echo "Output directory: $OUTPUT_DIR"

# Function to capture screenshots
capture_screenshots() {
    local prefix="$1"
    local url="$2"
    
    echo "Capturing $prefix screenshots..."
    
    # Homepage
    nix run .#capture-screenshot -- "$url" \
        -o "$OUTPUT_DIR/$prefix/homepage.png" \
        --width 1200 --height 800 \
        --wait "section"
    
    # Full page homepage
    nix run .#capture-screenshot -- "$url" \
        -o "$OUTPUT_DIR/$prefix/homepage-full.png" \
        --width 1200 --height 800 \
        --full-page \
        --wait "section"
    
    # Mobile view
    nix run .#capture-screenshot -- "$url" \
        -o "$OUTPUT_DIR/$prefix/homepage-mobile.png" \
        --width 375 --height 667 \
        --wait "section"
    
    # Tablet view
    nix run .#capture-screenshot -- "$url" \
        -o "$OUTPUT_DIR/$prefix/homepage-tablet.png" \
        --width 768 --height 1024 \
        --wait "section"
    
    echo "✅ $prefix screenshots completed"
}

# Check if we're capturing "before" or "after"
if [ "$PR_NUMBER" = "before" ]; then
    capture_screenshots "before" "$BASE_URL"
elif [ "$PR_NUMBER" = "after" ]; then
    capture_screenshots "after" "$BASE_URL"
else
    # For actual PRs, capture "after" screenshots
    capture_screenshots "after" "$BASE_URL"
    
    # Generate comparison summary
    cat > "$OUTPUT_DIR/comparison.md" << EOF
# PR #$PR_NUMBER - Visual Comparison

Generated: $(date)

## Screenshots

### Desktop (1200x800)
- [Before](./before/homepage.png) ← [After](./after/homepage.png)
- [Before Full Page](./before/homepage-full.png) ← [After Full Page](./after/homepage-full.png)

### Mobile (375x667)
- [Before Mobile](./before/homepage-mobile.png) ← [After Mobile](./after/homepage-mobile.png)

### Tablet (768x1024)
- [Before Tablet](./before/homepage-tablet.png) ← [After Tablet](./after/homepage-tablet.png)

## Usage in PR

Add this to your PR description:

\`\`\`markdown
## 📸 Visual Changes

| View | Before | After |
|------|--------|--------|
| Desktop | ![Before](./before/homepage.png) | ![After](./after/homepage.png) |
| Full Page | ![Before](./before/homepage-full.png) | ![After](./after/homepage-full.png) |
| Mobile | ![Before](./before/homepage-mobile.png) | ![After](./after/homepage-mobile.png) |
| Tablet | ![Before](./before/homepage-tablet.png) | ![After](./after/homepage-tablet.png) |
\`\`\`

EOF
    
    echo "📋 Comparison summary generated: $OUTPUT_DIR/comparison.md"
fi

echo "🎉 Screenshot capture completed!"
echo "📁 Screenshots saved in: $OUTPUT_DIR"