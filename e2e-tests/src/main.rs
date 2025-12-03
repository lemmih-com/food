use anyhow::{Context, Result};
use std::time::Duration;
use thirtyfour::prelude::*;

struct TestRunner {
    driver: WebDriver,
    base_url: String,
}

impl TestRunner {
    async fn new() -> Result<Self> {
        let base_url = std::env::var("FOOD_APP_BASE_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:8787".to_string());

        let webdriver_port = std::env::var("WEBDRIVER_PORT").unwrap_or_else(|_| "4444".to_string());

        let mut caps = DesiredCapabilities::firefox();
        caps.set_headless()?;

        let driver = WebDriver::new(&format!("http://localhost:{}", webdriver_port), caps)
            .await
            .context("creating WebDriver connection")?;

        driver
            .set_implicit_wait_timeout(Duration::from_secs(10))
            .await?;

        Ok(Self { driver, base_url })
    }

    async fn get_page_source(&self) -> Result<String> {
        self.driver.goto(&self.base_url).await?;
        self.driver.source().await.context("getting page source")
    }

    async fn get_css_content(&self) -> Result<String> {
        let css_url = format!("{}/pkg/styles.css", self.base_url);
        self.driver.goto(&css_url).await?;
        self.driver.source().await.context("getting CSS content")
    }
}

impl Drop for TestRunner {
    fn drop(&mut self) {
        // WebDriver will be automatically closed when dropped
    }
}

// ============================================================================
// Test Definitions
// ============================================================================

/// Test: Main page is reachable and contains expected content
async fn test_main_page_reachable(runner: &TestRunner) -> Result<()> {
    let body = runner.get_page_source().await?;

    assert!(
        body.contains("food.lemmih.com"),
        "HTML should contain 'food.lemmih.com'"
    );

    Ok(())
}

/// Test: CSS stylesheet link is present in HTML head
async fn test_css_link_present(runner: &TestRunner) -> Result<()> {
    let body = runner.get_page_source().await?;

    assert!(
        body.contains(r#"href="/pkg/styles.css""#) && body.contains("stylesheet"),
        "HTML should contain CSS link tag with /pkg/styles.css"
    );

    Ok(())
}

/// Test: CSS file is accessible and not empty
async fn test_css_file_accessible(runner: &TestRunner) -> Result<()> {
    let css_content = runner.get_css_content().await?;

    assert!(!css_content.is_empty(), "CSS file should not be empty");
    assert!(
        css_content.len() >= 100,
        "CSS file should have sufficient content (at least 100 bytes, got {})",
        css_content.len()
    );

    Ok(())
}

/// Test: CSS contains required Tailwind utility classes
async fn test_css_contains_tailwind_classes(runner: &TestRunner) -> Result<()> {
    let css_content = runner.get_css_content().await?;

    // Classes that should be present based on the HTML structure
    let required_classes = [
        (".mx-auto", "section element"),
        (".flex", "div container"),
        (".items-center", "flex container"),
        (".justify-center", "flex container"),
        (".text-center", "section element"),
        (".min-h-screen", "main element"),
    ];

    let mut missing = Vec::new();
    for (class, context) in &required_classes {
        if !css_content.contains(class) {
            missing.push(format!("{} (used in {})", class, context));
        }
    }

    assert!(
        missing.is_empty(),
        "CSS should contain all required Tailwind classes. Missing: {:?}",
        missing
    );

    Ok(())
}

/// Test: CSS is valid Tailwind CSS output
async fn test_css_is_valid_tailwind(runner: &TestRunner) -> Result<()> {
    let css_content = runner.get_css_content().await?;

    assert!(
        css_content.contains("tailwindcss"),
        "CSS should contain Tailwind CSS identifier"
    );

    Ok(())
}

// ============================================================================
// Test Runner
// ============================================================================

macro_rules! run_tests {
    ($runner:expr; $( $name:literal => $test:ident ),* $(,)? ) => {{
        let test_names: &[&str] = &[$($name),*];
        let total = test_names.len();
        println!("Running {} tests...\n", total);

        let mut idx = 0;
        $(
            idx += 1;
            print!("[{}/{}] {} ... ", idx, total, $name);
            match $test($runner).await {
                Ok(()) => println!("✅"),
                Err(e) => {
                    println!("❌");
                    anyhow::bail!("Test '{}' failed: {}", $name, e);
                }
            }
        )*

        println!("\n✅ All {} tests passed!", total);
        Ok::<(), anyhow::Error>(())
    }};
}

#[tokio::main]
async fn main() -> Result<()> {
    let runner = TestRunner::new().await?;

    run_tests!(&runner;
        "Main page is reachable" => test_main_page_reachable,
        "CSS link present in HTML" => test_css_link_present,
        "CSS file is accessible" => test_css_file_accessible,
        "CSS contains Tailwind classes" => test_css_contains_tailwind_classes,
        "CSS is valid Tailwind output" => test_css_is_valid_tailwind,
    )
}
