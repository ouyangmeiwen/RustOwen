use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;


#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct LibItemModel {
    pub id: String,                                  // Corresponds to `id` (VARCHAR(32))
    pub creation_time: NaiveDateTime,                 // Corresponds to `creation_time` (TIMESTAMP(6))
    pub creator_user_id: Option<i64>,                 // Corresponds to `creator_user_id` (BIGINT)
    pub last_modification_time: Option<NaiveDateTime>, // Corresponds to `last_modification_time` (TIMESTAMP(6))
    pub last_modifier_user_id: Option<i64>,           // Corresponds to `last_modifier_user_id` (BIGINT)
    pub is_deleted: bool,                             // Corresponds to `is_deleted` (BOOLEAN)
    pub deleter_user_id: Option<i64>,                 // Corresponds to `deleter_user_id` (BIGINT)
    pub deletion_time: Option<NaiveDateTime>,         // Corresponds to `deletion_time` (TIMESTAMP(6))
    pub info_id: Option<String>,                      // Corresponds to `info_id` (VARCHAR(32))
    pub title: String,                                // Corresponds to `title` (VARCHAR(512))
    pub author: Option<String>,                       // Corresponds to `author` (VARCHAR(512))
    pub barcode: String,                              // Corresponds to `barcode` (VARCHAR(32))
    pub is_enable: bool,                              // Corresponds to `is_enable` (BOOLEAN)
    pub call_no: Option<String>,                      // Corresponds to `call_no` (VARCHAR(64))
    pub pre_call_no: Option<String>,                  // Corresponds to `pre_call_no` (VARCHAR(64))
    pub catalog_code: Option<String>,                 // Corresponds to `catalog_code` (VARCHAR(32))
    pub item_state: i16,                              // Corresponds to `item_state` (SMALLINT)
    pub pressmark_id: Option<String>,                 // Corresponds to `pressmark_id` (VARCHAR(32))
    pub pressmark_name: Option<String>,               // Corresponds to `pressmark_name` (VARCHAR(64))
    pub location_id: Option<String>,                  // Corresponds to `location_id` (VARCHAR(32))
    pub location_name: Option<String>,                // Corresponds to `location_name` (VARCHAR(128))
    pub book_barcode: Option<String>,                 // Corresponds to `book_barcode` (VARCHAR(32))
    pub isbn: Option<String>,                         // Corresponds to `isbn` (VARCHAR(32))
    pub pub_no: Option<i16>,                          // Corresponds to `pub_no` (SMALLINT)
    pub publisher: Option<String>,                    // Corresponds to `publisher` (VARCHAR(512))
    pub pub_date: Option<String>,                     // Corresponds to `pub_date` (VARCHAR(512))
    pub price: Option<String>,                        // Corresponds to `price` (VARCHAR(32))
    pub pages: Option<String>,                        // Corresponds to `pages` (VARCHAR(32))
    pub summary: Option<String>,                      // Corresponds to `summary` (TEXT)
    pub item_type: i16,                               // Corresponds to `item_type` (SMALLINT)
    pub remark: Option<String>,                       // Corresponds to `remark` (VARCHAR(256))
    pub origin_type: i16,                             // Corresponds to `origin_type` (SMALLINT)
    pub create_type: i16,                             // Corresponds to `create_type` (SMALLINT)
    pub tenant_id: i32,                               // Corresponds to `tenant_id` (INT)
}
