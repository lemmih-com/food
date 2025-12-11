-- Migration: Replace category/store columns with a labels system
-- This allows flexible filtering similar to recipes

-- Delete all existing ingredients since we're changing the schema significantly
DELETE FROM ingredients;

-- Create a new ingredients table without category/store
CREATE TABLE IF NOT EXISTS ingredients_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
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
    -- Timestamps
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Copy any remaining data (should be empty after DELETE)
INSERT INTO ingredients_new (id, name, calories, protein, fat, saturated_fat, carbs, sugar, fiber, salt, package_size_g, package_price, created_at, updated_at)
SELECT id, name, calories, protein, fat, saturated_fat, carbs, sugar, fiber, salt, package_size_g, package_price, created_at, updated_at
FROM ingredients;

-- Drop the old table and rename the new one
DROP TABLE ingredients;
ALTER TABLE ingredients_new RENAME TO ingredients;

-- Create ingredient_labels junction table for the labels system
CREATE TABLE IF NOT EXISTS ingredient_labels (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    ingredient_id INTEGER NOT NULL,
    label TEXT NOT NULL,
    FOREIGN KEY (ingredient_id) REFERENCES ingredients(id) ON DELETE CASCADE,
    UNIQUE(ingredient_id, label)
);

-- Index for efficient label lookups
CREATE INDEX IF NOT EXISTS idx_ingredient_labels_ingredient ON ingredient_labels(ingredient_id);
CREATE INDEX IF NOT EXISTS idx_ingredient_labels_label ON ingredient_labels(label);
