// src/repository/libitem_repository.rs

use diesel::prelude::*;
use crate::models::libitem::{LibItem, NewLibItem, UpdateLibItem};
use crate::schema::libitem;
use crate::db::establish_connection;

// 创建新的 LibItem
pub fn create_libitem(conn: &MysqlConnection, new_item: NewLibItem) -> LibItem {
    diesel::insert_into(libitem::table)
        .values(&new_item)
        .execute(conn)
        .expect("Error inserting new libitem");

    // 查询新插入的项并返回
    libitem::table
        .filter(libitem::id.eq(new_item.id))
        .first(conn)
        .expect("Error loading new libitem")
}

// 获取分页查询的 LibItems
pub fn get_libitems(conn: &MysqlConnection, page: i64, page_size: i64) -> Vec<LibItem> {
    libitem::table
        .filter(libitem::is_deleted.eq(false))
        .limit(page_size)
        .offset(page * page_size)
        .load::<LibItem>(conn)
        .expect("Error loading libitems")
}

// 根据 ID 获取单条 LibItem
pub fn get_libitem_by_id(conn: &MysqlConnection, item_id: &str) -> Option<LibItem> {
    libitem::table
        .filter(libitem::id.eq(item_id))
        .first(conn)
        .optional()
        .expect("Error loading libitem by ID")
}

// 更新 LibItem
pub fn update_libitem(conn: &MysqlConnection, item_id: &str, updated_item: UpdateLibItem) -> LibItem {
    diesel::update(libitem::table.filter(libitem::id.eq(item_id)))
        .set(&updated_item)
        .execute(conn)
        .expect("Error updating libitem");

    libitem::table
        .filter(libitem::id.eq(item_id))
        .first(conn)
        .expect("Error loading updated libitem")
}

// 删除 LibItem
pub fn delete_libitem(conn: &MysqlConnection, item_id: &str) {
    diesel::update(libitem::table.filter(libitem::id.eq(item_id)))
        .set(libitem::is_deleted.eq(true))
        .execute(conn)
        .expect("Error deleting libitem");
}
