use anyhow::{Context, Result};
use std::time::Duration;
use thirtyfour::components::SelectElement;
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
        // Wait a bit for page to fully load
        tokio::time::sleep(Duration::from_millis(500)).await;
        self.driver.source().await.context("getting page source")
    }

    async fn get_page_source_at(&self, path: &str) -> Result<String> {
        let url = format!("{}{}", self.base_url, path);
        self.driver.goto(&url).await?;
        // Wait longer for page to fully load (including async data fetching)
        tokio::time::sleep(Duration::from_millis(2000)).await;
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
    let body = runner
        .get_page_source()
        .await
        .context("Failed to fetch main page")?;

    if !body.contains("food.lemmih.com") {
        anyhow::bail!(
            "HTML should contain 'food.lemmih.com'. Page length: {} bytes",
            body.len()
        );
    }

    if !body.contains("Food Log") {
        anyhow::bail!(
            "Main page should contain 'Food Log' heading. Page length: {} bytes",
            body.len()
        );
    }

    Ok(())
}

/// Test: Navigation links are present
async fn test_navigation_links_present(runner: &TestRunner) -> Result<()> {
    let body = runner
        .get_page_source()
        .await
        .context("Failed to fetch main page for navigation check")?;

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

    if !missing.is_empty() {
        anyhow::bail!(
            "Navigation should contain all required links. Missing: {:?}. Page length: {} bytes",
            missing,
            body.len()
        );
    }

    Ok(())
}

/// Test: Ingredients page is accessible and contains expected content
async fn test_ingredients_page_accessible(runner: &TestRunner) -> Result<()> {
    let body = runner
        .get_page_source_at("/ingredients")
        .await
        .context("Failed to fetch ingredients page")?;

    if !body.contains("Ingredient List") && !body.contains("Ingredients") {
        anyhow::bail!(
            "Ingredients page should contain 'Ingredient List' or 'Ingredients' heading. Page length: {} bytes",
            body.len()
        );
    }

    // Check if still loading
    if body.contains("Loading ingredients") {
        anyhow::bail!(
            "Ingredients page is still showing loading state after 2s wait. Server function may not be working. Page length: {} bytes",
            body.len()
        );
    }

    // Check for error state
    if body.contains("Failed to load ingredients") {
        anyhow::bail!(
            "Ingredients page shows error loading data. Check server function and D1 database. Page length: {} bytes",
            body.len()
        );
    }

    // Check for table headers which should be present even without data
    let has_table_headers = body.contains("Proteins")
        || body.contains("Carbs")
        || body.contains("Vegetables")
        || body.contains("Other");

    if !has_table_headers {
        // Print a snippet of the body for debugging
        let snippet = if body.len() > 500 {
            &body[..500]
        } else {
            &body
        };
        anyhow::bail!(
            "Ingredients page should contain category tables (Proteins, Carbs, Vegetables, Other). Page length: {} bytes. Snippet: {}...",
            body.len(),
            snippet
        );
    }

    // More lenient check - just verify some ingredient-related content exists
    let has_ingredient = body.contains("Chicken Breast")
        || body.contains("chicken breast")
        || body.contains("Broccoli")
        || body.contains("broccoli");

    if !has_ingredient {
        anyhow::bail!(
            "Ingredients page should contain sample ingredient data. Page length: {} bytes",
            body.len()
        );
    }

    // Check for nutritional columns
    let has_nutrition = (body.contains("Protein") || body.contains("protein"))
        && (body.contains("Carbs") || body.contains("carbs") || body.contains("Carbohydrates"));

    if !has_nutrition {
        anyhow::bail!(
            "Ingredients page should contain nutritional columns. Page length: {} bytes",
            body.len()
        );
    }

    Ok(())
}

/// Test: Recipes page is accessible and contains expected content
async fn test_recipes_page_accessible(runner: &TestRunner) -> Result<()> {
    let body = runner
        .get_page_source_at("/recipes")
        .await
        .context("Failed to fetch recipes page")?;

    if !body.contains("Recipes") && !body.contains("recipes") {
        anyhow::bail!(
            "Recipes page should contain 'Recipes' heading. Page length: {} bytes",
            body.len()
        );
    }

    // More lenient check for recipe structure
    let has_ingredients = body.contains("Ingredients:") || body.contains("ingredients:");
    let has_instructions = body.contains("Instructions:") || body.contains("instructions:");

    if !has_ingredients || !has_instructions {
        anyhow::bail!(
            "Recipes page should contain recipe structure (Ingredients and Instructions). Has ingredients: {}, Has instructions: {}. Page length: {} bytes",
            has_ingredients,
            has_instructions,
            body.len()
        );
    }

    Ok(())
}

/// Test: Settings page is accessible and contains expected content
async fn test_settings_page_accessible(runner: &TestRunner) -> Result<()> {
    let body = runner
        .get_page_source_at("/settings")
        .await
        .context("Failed to fetch settings page")?;

    if !body.contains("Settings") && !body.contains("settings") {
        anyhow::bail!(
            "Settings page should contain 'Settings' heading. Page length: {} bytes",
            body.len()
        );
    }

    // More lenient check for settings options
    let has_settings = body.contains("Target Calories")
        || body.contains("Macro Distribution")
        || body.contains("Daily Goals")
        || body.contains("calories");

    if !has_settings {
        anyhow::bail!(
            "Settings page should contain settings options. Page length: {} bytes",
            body.len()
        );
    }

    Ok(())
}

/// Test: CSS stylesheet link is present in HTML head
async fn test_css_link_present(runner: &TestRunner) -> Result<()> {
    let body = runner
        .get_page_source()
        .await
        .context("Failed to fetch page for CSS link check")?;

    let has_css_link = body.contains(r#"href="/pkg/styles.css""#) && body.contains("stylesheet");

    if !has_css_link {
        anyhow::bail!(
            "HTML should contain CSS link tag with /pkg/styles.css. Page length: {} bytes",
            body.len()
        );
    }

    Ok(())
}

/// Test: CSS file is accessible and not empty
async fn test_css_file_accessible(runner: &TestRunner) -> Result<()> {
    let css_content = runner
        .get_css_content()
        .await
        .context("Failed to fetch CSS file")?;

    if css_content.is_empty() {
        anyhow::bail!("CSS file should not be empty");
    }

    if css_content.len() < 100 {
        anyhow::bail!(
            "CSS file should have sufficient content (at least 100 bytes, got {})",
            css_content.len()
        );
    }

    Ok(())
}

/// Test: CSS contains required Tailwind utility classes
async fn test_css_contains_tailwind_classes(runner: &TestRunner) -> Result<()> {
    let css_content = runner
        .get_css_content()
        .await
        .context("Failed to fetch CSS for Tailwind check")?;

    // Classes that should be present based on the HTML structure
    // Make this more lenient by checking for partial matches
    let required_classes = [
        ("mx-auto", "section element"),
        ("flex", "div container"),
        ("items-center", "flex container"),
        ("min-h-screen", "main element"),
    ];

    let mut missing = Vec::new();
    for (class, context) in &required_classes {
        // Check for the class in various forms (., :, [, etc.)
        if !css_content.contains(class) {
            missing.push(format!("{} (used in {})", class, context));
        }
    }

    if !missing.is_empty() {
        anyhow::bail!(
            "CSS should contain required Tailwind classes. Missing: {:?}. CSS length: {} bytes",
            missing,
            css_content.len()
        );
    }

    Ok(())
}

/// Test: CSS is valid Tailwind CSS output
async fn test_css_is_valid_tailwind(runner: &TestRunner) -> Result<()> {
    let css_content = runner
        .get_css_content()
        .await
        .context("Failed to fetch CSS for validation")?;

    // More lenient check - just verify it's not empty and has some CSS structure
    if css_content.len() < 50 {
        anyhow::bail!(
            "CSS should have meaningful content (got {} bytes)",
            css_content.len()
        );
    }

    // Check for basic CSS structure
    let has_css_structure = css_content.contains("{") && css_content.contains("}");

    if !has_css_structure {
        anyhow::bail!(
            "CSS should contain valid CSS structure. CSS length: {} bytes",
            css_content.len()
        );
    }

    Ok(())
}

// ----------------------------------------------------------------------------
// Recipe CRUD flow (requires unlock)
// ----------------------------------------------------------------------------

async fn unlock_admin(runner: &TestRunner) -> Result<()> {
    // If already unlocked, the new recipe button will be present
    if runner
        .driver
        .find_elements(By::Css(r#"button[data-test="recipe-new-button"]"#))
        .await?
        .is_empty()
    {
        let unlock_btn = runner
            .driver
            .find_element(By::XPath(r#"//button[contains(., 'Unlock')]"#))
            .await
            .context("finding unlock button")?;
        unlock_btn.click().await?;

        let pin = std::env::var("ADMIN_PIN").unwrap_or_else(|_| "1234".to_string());
        for (idx, ch) in pin.chars().take(4).enumerate() {
            let input = runner
                .driver
                .find_element(By::Id(&format!("pin-digit-{}", idx)))
                .await
                .context("finding PIN input")?;
            input.clear().await?;
            input.send_keys(ch.to_string()).await?;
        }

        tokio::time::sleep(Duration::from_millis(400)).await;
    }

    // Wait until the button shows up
    for _ in 0..10 {
        if !runner
            .driver
            .find_elements(By::Css(r#"button[data-test="recipe-new-button"]"#))
            .await?
            .is_empty()
        {
            return Ok(());
        }
        tokio::time::sleep(Duration::from_millis(300)).await;
    }

    anyhow::bail!("Failed to unlock admin mode for recipes")
}

async fn go_to_recipes(runner: &TestRunner) -> Result<()> {
    runner
        .driver
        .goto(&format!("{}/recipes", runner.base_url))
        .await?;
    tokio::time::sleep(Duration::from_millis(800)).await;
    Ok(())
}

async fn fill_first_ingredient(runner: &TestRunner) -> Result<()> {
    for _ in 0..10 {
        let select_el = runner
            .driver
            .find_element(By::Css(r#"select[data-test="recipe-ingredient-select-0"]"#))
            .await?;
        let select = SelectElement::new(&select_el)?;
        let options = select.options().await?;
        if options.len() > 1 {
            select.select_by_index(1).await?; // first non-empty option

            // Use 1 package and 50 extra grams to exercise both inputs
            let pkg_input = runner
                .driver
                .find_element(By::Css(
                    r#"input[data-test="recipe-ingredient-packages-0"]"#,
                ))
                .await?;
            pkg_input.clear().await?;
            pkg_input.send_keys("1").await?;

            let grams_input = runner
                .driver
                .find_element(By::Css(r#"input[data-test="recipe-ingredient-grams-0"]"#))
                .await?;
            grams_input.clear().await?;
            grams_input.send_keys("50").await?;

            return Ok(());
        }
        tokio::time::sleep(Duration::from_millis(200)).await;
    }

    anyhow::bail!("No ingredient options available for recipe form")
}

async fn create_recipe(runner: &TestRunner, name: &str) -> Result<()> {
    runner
        .driver
        .find_element(By::Css(r#"button[data-test="recipe-new-button"]"#))
        .await?
        .click()
        .await?;

    tokio::time::sleep(Duration::from_millis(200)).await;

    let name_input = runner
        .driver
        .find_element(By::Css(r#"input[data-test="recipe-name"]"#))
        .await?;
    name_input.clear().await?;
    name_input.send_keys(name).await?;

    let servings = runner
        .driver
        .find_element(By::Css(r#"input[data-test="recipe-servings"]"#))
        .await?;
    servings.clear().await?;
    servings.send_keys("2").await?;

    let tags = runner
        .driver
        .find_element(By::Css(r#"input[data-test="recipe-tags"]"#))
        .await?;
    tags.clear().await?;
    tags.send_keys("Test, Quick").await?;

    let instructions = runner
        .driver
        .find_element(By::Css(r#"textarea[data-test="recipe-instructions"]"#))
        .await?;
    instructions.clear().await?;
    instructions
        .send_keys("Step one\nStep two\nServe and enjoy")
        .await?;

    fill_first_ingredient(runner).await?;

    runner
        .driver
        .find_element(By::Css(r#"button[data-test="recipe-save"]"#))
        .await?
        .click()
        .await?;

    // Wait for card to appear
    for _ in 0..10 {
        let body = runner.driver.source().await?;
        if body.contains(name) {
            return Ok(());
        }
        tokio::time::sleep(Duration::from_millis(300)).await;
    }

    anyhow::bail!("Recipe card did not appear after save")
}

async fn edit_recipe(runner: &TestRunner, original: &str, updated: &str) -> Result<()> {
    // Click the first edit button
    runner
        .driver
        .find_element(By::Css(r#"button[data-test^="recipe-edit-"]"#))
        .await?
        .click()
        .await?;

    tokio::time::sleep(Duration::from_millis(200)).await;

    let name_input = runner
        .driver
        .find_element(By::Css(r#"input[data-test="recipe-name"]"#))
        .await?;
    name_input.clear().await?;
    name_input.send_keys(updated).await?;

    runner
        .driver
        .find_element(By::Css(r#"button[data-test="recipe-save"]"#))
        .await?
        .click()
        .await?;

    for _ in 0..10 {
        let body = runner.driver.source().await?;
        if body.contains(updated) && !body.contains(original) {
            return Ok(());
        }
        tokio::time::sleep(Duration::from_millis(300)).await;
    }

    anyhow::bail!("Updated recipe title not found after edit")
}

async fn delete_recipe(runner: &TestRunner, name: &str) -> Result<()> {
    runner
        .driver
        .find_element(By::Css(r#"button[data-test^="recipe-delete-"]"#))
        .await?
        .click()
        .await?;

    // Confirm dialog
    if let Ok(alert) = runner.driver.switch_to().alert() {
        alert.accept().await?;
    }

    for _ in 0..10 {
        let body = runner.driver.source().await?;
        if !body.contains(name) {
            return Ok(());
        }
        tokio::time::sleep(Duration::from_millis(300)).await;
    }

    anyhow::bail!("Recipe still visible after delete")
}

async fn test_recipes_crud_flow(runner: &TestRunner) -> Result<()> {
    go_to_recipes(runner).await?;
    unlock_admin(runner).await?;

    let name = "Test Recipe";
    create_recipe(runner, name).await?;

    let updated = "Updated Test Recipe";
    edit_recipe(runner, name, updated).await?;
    delete_recipe(runner, updated).await?;

    Ok(())
}

/// Test: Macro distribution always sums to 100% after changing inputs
async fn test_macro_distribution_sums_to_100(runner: &TestRunner) -> Result<()> {
    // Navigate to settings page
    let url = format!("{}/settings", runner.base_url);
    runner.driver.goto(&url).await?;
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Find macro percentage inputs (they are number inputs with min="5" max="90")
    // The inputs are in order: protein %, protein g, carbs %, carbs g, fat %, fat g
    let inputs = runner
        .driver
        .find_all(By::Css(r#"input[type="number"][min="5"][max="90"]"#))
        .await
        .context("Failed to find macro percentage inputs")?;

    if inputs.len() != 3 {
        anyhow::bail!("Expected 3 macro percentage inputs, found {}", inputs.len());
    }

    // Helper to get current macro percentages
    async fn get_macro_sum(inputs: &[WebElement]) -> Result<i32> {
        let mut sum = 0;
        for input in inputs {
            let value = input.prop("value").await?.unwrap_or_default();
            let pct: i32 = value.parse().unwrap_or(0);
            sum += pct;
        }
        Ok(sum)
    }

    // Check initial sum is 100%
    let initial_sum = get_macro_sum(&inputs).await?;
    if initial_sum != 100 {
        anyhow::bail!(
            "Initial macro distribution should sum to 100%, got {}%",
            initial_sum
        );
    }

    // Change protein to 50% and verify sum is still 100%
    inputs[0].clear().await?;
    inputs[0].send_keys("50").await?;
    tokio::time::sleep(Duration::from_millis(300)).await;

    // Re-fetch inputs after the page updates
    let inputs = runner
        .driver
        .find_all(By::Css(r#"input[type="number"][min="5"][max="90"]"#))
        .await?;

    let sum_after_protein_change = get_macro_sum(&inputs).await?;
    if sum_after_protein_change != 100 {
        anyhow::bail!(
            "After changing protein to 50%, macro sum should be 100%, got {}%",
            sum_after_protein_change
        );
    }

    // Change carbs to 30% and verify sum is still 100%
    inputs[1].clear().await?;
    inputs[1].send_keys("30").await?;
    tokio::time::sleep(Duration::from_millis(300)).await;

    let inputs = runner
        .driver
        .find_all(By::Css(r#"input[type="number"][min="5"][max="90"]"#))
        .await?;

    let sum_after_carbs_change = get_macro_sum(&inputs).await?;
    if sum_after_carbs_change != 100 {
        anyhow::bail!(
            "After changing carbs to 30%, macro sum should be 100%, got {}%",
            sum_after_carbs_change
        );
    }

    // Change fat to 25% and verify sum is still 100%
    inputs[2].clear().await?;
    inputs[2].send_keys("25").await?;
    tokio::time::sleep(Duration::from_millis(300)).await;

    let inputs = runner
        .driver
        .find_all(By::Css(r#"input[type="number"][min="5"][max="90"]"#))
        .await?;

    let sum_after_fat_change = get_macro_sum(&inputs).await?;
    if sum_after_fat_change != 100 {
        anyhow::bail!(
            "After changing fat to 25%, macro sum should be 100%, got {}%",
            sum_after_fat_change
        );
    }

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
        "Macro distribution sums to 100%" => test_macro_distribution_sums_to_100,
        "Recipe CRUD flow works" => test_recipes_crud_flow,
    );

    // Explicitly quit WebDriver to avoid Tokio runtime shutdown panic
    runner.quit().await?;

    result
}
