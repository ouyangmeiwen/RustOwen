// src/db/mod.rs

use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use crate::config::Config;

pub fn establish_connection(config: &Config) -> MysqlConnection {
    MysqlConnection::establish(&config.db_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", config.db_url))
}
