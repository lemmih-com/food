-- Create ingredients table
CREATE TABLE IF NOT EXISTS ingredients (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    category TEXT NOT NULL CHECK(category IN ('protein', 'carbs', 'veggies', 'other')),
    -- Nutrients per 100g
    calories REAL NOT NULL DEFAULT 0,
    protein REAL NOT NULL DEFAULT 0,
    fat REAL NOT NULL DEFAULT 0,
    saturated_fat REAL NOT NULL DEFAULT 0,
    carbs REAL NOT NULL DEFAULT 0,
    sugar REAL NOT NULL DEFAULT 0,
    fiber REAL NOT NULL DEFAULT 0,
    salt REAL NOT NULL DEFAULT 0,
    -- Package info
    package_size_g REAL NOT NULL DEFAULT 0,
    package_price REAL NOT NULL DEFAULT 0,
    store TEXT NOT NULL DEFAULT '',
    -- Timestamps
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Index for category filtering
CREATE INDEX IF NOT EXISTS idx_ingredients_category ON ingredients(category);
