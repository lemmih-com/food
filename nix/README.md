# Screenshot Tool

A cross-platform headless screenshot tool using Playwright and Nix.

## Features

- ✅ Cross-platform (Linux, macOS, Windows)
- ✅ Works in Nix sandboxes (including macOS)
- ✅ Multiple viewport sizes (desktop, mobile, tablet)
- ✅ Full page screenshots
- ✅ Multiple image formats (PNG, JPEG)
- ✅ Custom wait selectors
- ✅ Configurable timeouts

## Usage

### Basic Screenshot

```bash
# Capture a website
nix run .#capture-screenshot -- https://example.com

# Custom output file and size
nix run .#capture-screenshot -- https://example.com -o example.png -w 1920 -h 1080

# Full page screenshot
nix run .#capture-screenshot -- https://example.com --full-page

# Wait for specific element
nix run .#capture-screenshot -- https://example.com --wait ".content"

# JPEG format with quality
nix run .#capture-screenshot -- https://example.com -o photo.jpeg --format jpeg --quality 90
```

### PR Snapshots

```bash
# Capture "before" screenshots (baseline)
nix run .#pr-snapshots -- before

# Capture "after" screenshots (PR changes)
nix run .#pr-snapshots -- 123

# Custom base URL and output directory
BASE_URL=https://staging.example.com OUTPUT_DIR=./my-screenshots nix run .#pr-snapshots -- 123
```

## Command Line Options

### capture-screenshot

```
Usage: capture-screenshot <url> [options]

Arguments:
  url                    URL to capture screenshot of

Options:
  -o, --output FILE      Output file path (default: screenshot.png)
  -w, --width PIXELS     Viewport width (default: 1200)
  -h, --height PIXELS    Viewport height (default: 800)
  -f, --full-page        Capture full page screenshot
  --wait SELECTOR        Wait for selector before taking screenshot
  --timeout MS           Timeout in milliseconds (default: 30000)
  --format FORMAT        Image format: png|jpeg (default: png)
  --quality PERCENT      JPEG quality 1-100 (default: 80)
  --help                 Show this help message
```

### pr-snapshots

```
Usage: ./pr-snapshots.sh <PR_NUMBER>

Environment variables:
   BASE_URL - Base URL to capture (default: http://localhost:8787)
   OUTPUT_DIR - Directory to save screenshots (default: ./screenshots)
```

## Integration with CI/CD

### GitHub Actions

```yaml
name: Visual Regression Tests

on:
  pull_request:
    types: [opened, synchronize]

jobs:
  screenshots:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Nix
        uses: cachix/install-nix-action@v22
        
      - name: Setup development environment
        run: |
          nix develop --accept-flake-config
          
      - name: Start development server
        run: |
          nix run .#website &
          sleep 10
          
      - name: Capture screenshots
        run: |
          nix run .#pr-snapshots -- ${{ github.event.number }}
          
      - name: Upload screenshots
        uses: actions/upload-artifact@v3
        with:
          name: pr-screenshots-${{ github.event.number }}
          path: screenshots/
```

### Local Development

```bash
# Start your development server
wrangler dev --port 8787

# In another terminal, capture baseline screenshots
nix run .#pr-snapshots -- before

# Make your changes...

# Capture after screenshots
nix run .#pr-snapshots -- after

# Compare screenshots manually or use image diff tools
```

## How It Works

1. **Nix Integration**: Uses `playwright-driver` from Nixpkgs which includes browser binaries
2. **Cross-Platform**: Works on macOS where Chromium isn't available in Nixpkgs
3. **Headless**: Runs browsers without GUI, perfect for CI/CD
4. **Flexible**: Supports multiple viewports, formats, and wait conditions

## Technical Details

- **Browser**: Chromium (via Playwright)
- **Node.js**: Version 20+ 
- **Nix**: Uses `playwright-driver` package
- **Formats**: PNG (default), JPEG
- **Viewports**: Customizable, includes presets for mobile/tablet

## Troubleshooting

### Connection Refused
Ensure your development server is running:
```bash
wrangler dev --port 8787
```

### Permission Issues
The tool creates temporary files in the current directory. Ensure write permissions.

### Browser Not Found
Check that `playwright-driver` is available in your Nix environment.

## Examples

### E-commerce Site
```bash
# Capture product page with wait
nix run .#capture-screenshot -- \
  https://example.com/product/123 \
  --wait ".product-price" \
  --full-page \
  -o product-screenshot.png
```

### Dashboard
```bash
# Mobile dashboard screenshot
nix run .#capture-screenshot -- \
  https://dashboard.example.com \
  --width 375 --height 667 \
  --wait ".dashboard-content" \
  -o dashboard-mobile.png
```

### Full Website Audit
```bash
#!/bin/bash
URLS=(
  "https://example.com"
  "https://example.com/about"
  "https://example.com/contact"
)

for url in "${URLS[@]}"; do
  filename=$(echo "$url" | sed 's|https://||g; s|/|-|g')
  nix run .#capture-screenshot -- "$url" \
    -o "screenshots/${filename}.png" \
    --full-page \
    --wait "main"
done
```