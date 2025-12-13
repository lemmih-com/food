-- Add rotation field to food_logs for image crop rotation
ALTER TABLE food_logs ADD COLUMN crop_rotation INTEGER NOT NULL DEFAULT 0;
