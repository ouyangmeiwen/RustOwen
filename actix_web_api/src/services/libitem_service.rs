// src/services/libitem_service.rs

use crate::repository::libitem_repository::{create_libitem as repo_create, get_libitems as repo_get_all, get_libitem_by_id as repo_get_by_id, update_libitem as repo_update, delete_libitem as repo_delete};
use crate::models::libitem::{LibItem, NewLibItem};
use crate::db::establish_connection;
use crate::config::Config;

pub fn create_libitem(new_item: NewLibItem, config: Config) -> LibItem {
    let conn = establish_connection(&config);
    repo_create(&conn, new_item)
}

pub fn get_libitems(page: i64, page_size: i64, config: Config) -> Vec<LibItem> {
    let conn = establish_connection(&config);
    repo_get_all(&conn, page, page_size)
}

pub fn get_libitem_by_id(item_id: &str, config: Config) -> Option<LibItem> {
    let conn = establish_connection(&config);
    repo_get_by_id(&conn, item_id)
}

pub fn update_libitem(item_id: &str, updated_item: NewLibItem, config: Config) -> LibItem {
    let conn = establish_connection(&config);
    repo_update(&conn, item_id, updated_item)
}

pub fn delete_libitem(item_id: &str, config: Config) {
    let conn = establish_connection(&config);
    repo_delete(&conn, item_id)
}
