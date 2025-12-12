-- Food log entries for tracking meals
CREATE TABLE food_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    recipe_id INTEGER,
    image_key TEXT,
    logged_at TEXT NOT NULL DEFAULT (datetime('now')),
    rating INTEGER CHECK (rating >= 1 AND rating <= 5),
    notes TEXT NOT NULL DEFAULT '',
    -- Crop coordinates for the image (stored as percentages 0-100)
    crop_x REAL NOT NULL DEFAULT 0,
    crop_y REAL NOT NULL DEFAULT 0,
    crop_width REAL NOT NULL DEFAULT 100,
    crop_height REAL NOT NULL DEFAULT 100,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (recipe_id) REFERENCES recipes(id) ON DELETE SET NULL
);

-- Index for efficient date-based queries
CREATE INDEX idx_food_logs_logged_at ON food_logs(logged_at DESC);
