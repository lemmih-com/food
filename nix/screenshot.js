#!/usr/bin/env node

const { chromium } = require('playwright');
const path = require('path');
const fs = require('fs');

function printUsage() {
  console.error(`
Usage: screenshot.js <url> [options]

Arguments:
  url                    URL to capture screenshot of

Options:
  -o, --output FILE      Output file path (default: screenshot.png)
  -w, --width PIXELS     Viewport width (default: 1200)
  -h, --height PIXELS    Viewport height (default: 800)
  -f, --full-page        Capture full page screenshot
  -w, --wait SELECTOR    Wait for selector before taking screenshot
  --timeout MS           Timeout in milliseconds (default: 30000)
  --format FORMAT        Image format: png|jpeg (default: png)
  --quality PERCENT      JPEG quality 1-100 (default: 80)
  --help                 Show this help message

Examples:
  screenshot.js https://example.com
  screenshot.js https://example.com -o example.png -w 1920 -h 1080
  screenshot.js https://example.com --full-page --wait ".content"
  screenshot.js http://localhost:8787 -o local-screenshot.jpeg --format jpeg
`);
}

function parseArgs() {
  const args = process.argv.slice(2);
  if (args.length === 0 || args.includes('--help') || args.includes('-h')) {
    printUsage();
    process.exit(0);
  }

  const options = {
    url: '',
    output: 'screenshot.png',
    width: 1200,
    height: 800,
    fullPage: false,
    waitSelector: null,
    timeout: 30000,
    format: 'png',
    quality: 80
  };

  let i = 0;
  while (i < args.length) {
    const arg = args[i];
    
    switch (arg) {
      case '-o':
      case '--output':
        options.output = args[++i];
        break;
      case '-w':
      case '--width':
        options.width = parseInt(args[++i]);
        break;
      case '-h':
      case '--height':
        options.height = parseInt(args[++i]);
        break;
      case '-f':
      case '--full-page':
        options.fullPage = true;
        break;
      case '--wait':
        options.waitSelector = args[++i];
        break;
      case '--timeout':
        options.timeout = parseInt(args[++i]);
        break;
      case '--format':
        options.format = args[++i];
        break;
      case '--quality':
        options.quality = parseInt(args[++i]);
        break;
      default:
        if (!arg.startsWith('-') && !options.url) {
          options.url = arg;
        } else {
          console.error(`Unknown argument: ${arg}`);
          printUsage();
          process.exit(1);
        }
    }
    i++;
  }

  if (!options.url) {
    console.error('Error: URL is required');
    printUsage();
    process.exit(1);
  }

  return options;
}

async function captureScreenshot(options) {
  let browser;
  
  try {
    console.log(`Launching browser...`);
    browser = await chromium.launch({
      headless: true,
      args: ['--no-sandbox', '--disable-setuid-sandbox']
    });

    const context = await browser.newContext({
      viewport: { width: options.width, height: options.height }
    });

    const page = await context.newPage();

    console.log(`Navigating to: ${options.url}`);
    
    // Set timeout
    page.setDefaultTimeout(options.timeout);

    await page.goto(options.url, { 
      waitUntil: 'networkidle',
      timeout: options.timeout 
    });

    // Wait for specific selector if provided
    if (options.waitSelector) {
      console.log(`Waiting for selector: ${options.waitSelector}`);
      await page.waitForSelector(options.waitSelector, { timeout: options.timeout });
    }

    // Ensure output directory exists
    const outputDir = path.dirname(options.output);
    if (outputDir && !fs.existsSync(outputDir)) {
      fs.mkdirSync(outputDir, { recursive: true });
    }

    // Configure screenshot options
    const screenshotOptions = {
      path: options.output,
      fullPage: options.fullPage,
      type: options.format
    };

    // Add quality for JPEG
    if (options.format === 'jpeg') {
      screenshotOptions.quality = options.quality;
    }

    console.log(`Capturing screenshot: ${options.output}`);
    await page.screenshot(screenshotOptions);

    console.log(`✅ Screenshot saved to: ${options.output}`);

  } catch (error) {
    console.error(`❌ Error capturing screenshot: ${error.message}`);
    process.exit(1);
  } finally {
    if (browser) {
      await browser.close();
    }
  }
}

// Main execution
const options = parseArgs();
captureScreenshot(options);