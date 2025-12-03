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

    async fn get_page_source_at(&self, path: &str) -> Result<String> {
        let url = format!("{}{}", self.base_url, path);
        self.driver.goto(&url).await?;
        self.driver.source().await.context("getting page source")
    }

    async fn get_css_content(&self) -> Result<String> {
        let css_url = format!("{}/pkg/styles.css", self.base_url);
        self.driver.goto(&css_url).await?;
        self.driver.source().await.context("getting CSS content")
    }

    async fn quit(self) -> Result<()> {
        self.driver.quit().await.context("quitting WebDriver")
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

    assert!(
        body.contains("Food Log"),
        "Main page should contain 'Food Log' heading"
    );

    Ok(())
}

/// Test: Navigation links are present
async fn test_navigation_links_present(runner: &TestRunner) -> Result<()> {
    let body = runner.get_page_source().await?;

    let required_links = [
        ("Food Log", "Main page link"),
        ("Ingredients", "Ingredients page link"),
        ("Recipes", "Recipes page link"),
        ("Settings", "Settings page link"),
    ];

    let mut missing = Vec::new();
    for (link_text, context) in &required_links {
        if !body.contains(link_text) {
            missing.push(format!("{} ({})", link_text, context));
        }
    }

    assert!(
        missing.is_empty(),
        "Navigation should contain all required links. Missing: {:?}",
        missing
    );

    Ok(())
}

/// Test: Ingredients page is accessible and contains expected content
async fn test_ingredients_page_accessible(runner: &TestRunner) -> Result<()> {
    let body = runner.get_page_source_at("/ingredients").await?;

    assert!(
        body.contains("Ingredient List"),
        "Ingredients page should contain 'Ingredient List' heading"
    );

    assert!(
        body.contains("Chicken Breast") || body.contains("chicken breast"),
        "Ingredients page should contain sample ingredient data"
    );

    assert!(
        body.contains("Protein") && body.contains("Carbs"),
        "Ingredients page should contain nutritional columns"
    );

    Ok(())
}

/// Test: Recipes page is accessible and contains expected content
async fn test_recipes_page_accessible(runner: &TestRunner) -> Result<()> {
    let body = runner.get_page_source_at("/recipes").await?;

    assert!(
        body.contains("Recipes"),
        "Recipes page should contain 'Recipes' heading"
    );

    assert!(
        body.contains("Ingredients:") && body.contains("Instructions:"),
        "Recipes page should contain recipe structure"
    );

    Ok(())
}

/// Test: Settings page is accessible and contains expected content
async fn test_settings_page_accessible(runner: &TestRunner) -> Result<()> {
    let body = runner.get_page_source_at("/settings").await?;

    assert!(
        body.contains("Settings"),
        "Settings page should contain 'Settings' heading"
    );

    assert!(
        body.contains("Target Calories") || body.contains("Macro Distribution"),
        "Settings page should contain settings options"
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

    let result = run_tests!(&runner;
        "Main page is reachable" => test_main_page_reachable,
        "Navigation links present" => test_navigation_links_present,
        "Ingredients page accessible" => test_ingredients_page_accessible,
        "Recipes page accessible" => test_recipes_page_accessible,
        "Settings page accessible" => test_settings_page_accessible,
        "CSS link present in HTML" => test_css_link_present,
        "CSS file is accessible" => test_css_file_accessible,
        "CSS contains Tailwind classes" => test_css_contains_tailwind_classes,
        "CSS is valid Tailwind output" => test_css_is_valid_tailwind,
    );

    // Explicitly quit WebDriver to avoid Tokio runtime shutdown panic
    runner.quit().await?;

    result
}
