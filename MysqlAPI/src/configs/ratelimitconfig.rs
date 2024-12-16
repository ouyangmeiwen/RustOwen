use actix_web::Error;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::sync::RwLock;

#[derive(Serialize, Deserialize, Debug)]
struct RateLimitConfig {
    limit_per_second: u64,
    time_window_secs: u64,
}

lazy_static! {
    // Use RwLock to allow read-only access after the first initialization
    pub static ref GLOBAL_PATH_LIMITS: RwLock<HashMap<String, (u64, u64)>> = {
        let path_limits = load_config().unwrap_or_else(|e| {
            eprintln!("Failed to load config at startup: {:?}", e);
            HashMap::new() // Default to an empty configuration on error
        });
        RwLock::new(path_limits)
    };
}

/// Load configuration from a JSON file
fn load_config() -> Result<HashMap<String, (u64, u64)>, Error> {
    // Open the configuration file
    let file = File::open("config.json").map_err(|e| {
        eprintln!("Error opening config file: {}", e);
        Error::from(e)
    })?;

    // Parse the JSON into a HashMap<String, RateLimitConfig>
    let path_limits: HashMap<String, RateLimitConfig> =
        serde_json::from_reader(file).map_err(|e| {
            eprintln!("Error reading config file: {}", e);
            Error::from(e)
        })?;

    // Convert RateLimitConfig into the desired HashMap format
    Ok(path_limits
        .into_iter()
        .map(|(path, config)| (path, (config.limit_per_second, config.time_window_secs)))
        .collect())
}

/// Seal the global limits, making them effectively read-only
pub fn seal_global_path_limits() {
    let write_guard = GLOBAL_PATH_LIMITS.write().unwrap();
    // Drop the write guard to seal the configuration
    drop(write_guard);
    println!("GLOBAL_PATH_LIMITS is now read-only.");
}
