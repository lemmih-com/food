//! Local storage caching utilities
//!
//! Provides functions to cache and retrieve data from browser localStorage,
//! enabling instant display of cached data while fresh data loads in the background.

use gloo_storage::{LocalStorage, Storage};
use serde::{de::DeserializeOwned, Serialize};

/// Cache key for ingredients list
pub const INGREDIENTS_CACHE_KEY: &str = "food_ingredients_cache";

/// Cache key for recipes list
pub const RECIPES_CACHE_KEY: &str = "food_recipes_cache";

/// Store data in localStorage
pub fn set_cache<T: Serialize>(key: &str, data: &T) {
    if let Err(e) = LocalStorage::set(key, data) {
        log::warn!("Failed to cache data for key '{}': {:?}", key, e);
    }
}

/// Retrieve data from localStorage
pub fn get_cache<T: DeserializeOwned>(key: &str) -> Option<T> {
    match LocalStorage::get::<T>(key) {
        Ok(data) => Some(data),
        Err(gloo_storage::errors::StorageError::KeyNotFound(_)) => None,
        Err(e) => {
            log::warn!("Failed to read cache for key '{}': {:?}", key, e);
            None
        }
    }
}

/// Clear a specific cache entry
#[allow(dead_code)]
pub fn clear_cache(key: &str) {
    LocalStorage::delete(key);
}
