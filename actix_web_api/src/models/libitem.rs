// src/models/libitem.rs

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use crate::schema::libitem; // 引入 schema

// 结构体用于从数据库读取数据
#[derive(Queryable, Serialize, Deserialize, Debug)]
#[table_name = "libitem"]
pub struct LibItem {
    pub id: String,
    pub creation_time: Option<NaiveDateTime>,
    pub creator_user_id: Option<i64>,
    pub last_modification_time: Option<NaiveDateTime>,
    pub last_modifier_user_id: Option<i64>,
    pub is_deleted: bool,
    pub deleter_user_id: Option<i64>,
    pub deletion_time: Option<NaiveDateTime>,
    pub info_id: Option<String>,
    pub title: String,
    pub author: Option<String>,
    pub barcode: String,
    pub is_enable: bool,
    pub call_no: Option<String>,
    pub pre_call_no: Option<String>,
    pub catalog_code: Option<String>,
    pub item_state: u8,
    pub pressmark_id: Option<String>,
    pub pressmark_name: Option<String>,
    pub location_id: Option<String>,
    pub location_name: Option<String>,
    pub book_barcode: Option<String>,
    pub isbn: Option<String>,
    pub pub_no: Option<i16>,
    pub publisher: Option<String>,
    pub pub_date: Option<String>,
    pub price: Option<String>,
    pub pages: Option<String>,
    pub summary: Option<String>,
    pub item_type: u8,
    pub remark: Option<String>,
    pub origin_type: u8,
    pub create_type: u8,
    pub tenant_id: i32,
}

// 结构体用于插入新数据
#[derive(Insertable, Deserialize, Debug)]
#[table_name = "libitem"]
pub struct NewLibItem {
    pub id: String,
    pub title: String,
    pub barcode: String,
    pub is_deleted: bool,
    pub is_enable: bool,
    pub item_state: u8,
    pub item_type: u8,
    pub origin_type: u8,
    pub create_type: u8,
    pub tenant_id: i32,
    // 其他字段可以根据需要添加
}

// 结构体用于更新现有数据
#[derive(AsChangeset, Deserialize, Debug)]
#[table_name = "libitem"]
pub struct UpdateLibItem {
    pub title: Option<String>,
    pub barcode: Option<String>,
    pub is_deleted: Option<bool>,
    pub is_enable: Option<bool>,
    pub item_state: Option<u8>,
    pub item_type: Option<u8>,
    pub origin_type: Option<u8>,
    pub create_type: Option<u8>,
    // 其他字段...
}
