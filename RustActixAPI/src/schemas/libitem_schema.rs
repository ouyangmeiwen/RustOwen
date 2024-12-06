use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct FilterOptions {
    pub page: Option<usize>,  // 改为 snake_case
    pub limit: Option<usize>, // 改为 snake_case
}

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: String, // 改为 snake_case
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateLibItemSchema {
    pub title: String,          // VARCHAR(512)
    pub author: Option<String>, // VARCHAR(512)
    pub barcode: String,        // VARCHAR(32)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_no: Option<String>, // VARCHAR(64)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_call_no: Option<String>, // VARCHAR(64)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_code: Option<String>, // VARCHAR(32)
    pub item_state: i16,          // SMALLINT
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pressmark_id: Option<String>, // VARCHAR(32)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pressmark_name: Option<String>, // VARCHAR(64)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>, // VARCHAR(32)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>, // VARCHAR(128)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub book_barcode: Option<String>, // VARCHAR(32)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isbn: Option<String>, // VARCHAR(32)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pub_no: Option<i16>, // SMALLINT
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>, // VARCHAR(512)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pub_date: Option<String>, // VARCHAR(512)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>, // VARCHAR(32)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages: Option<String>, // VARCHAR(32)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>, // TEXT
    pub item_type: i16,           // SMALLINT
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>, // VARCHAR(256)
    pub origin_type: i16,         // SMALLINT
    pub create_type: i16,         // SMALLINT
    pub tenant_id: i32,          // INT
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateLibItemSchema {
    pub title: Option<String>,         // VARCHAR(512)
    pub author: Option<String>,        // VARCHAR(512)
    pub barcode: Option<String>,       // VARCHAR(32)
    pub call_no: Option<String>,        // VARCHAR(64)
    pub pre_call_no: Option<String>,     // VARCHAR(64)
    pub catalog_code: Option<String>,   // VARCHAR(32)
    pub item_state: Option<i16>,         // SMALLINT
    pub pressmark_id: Option<String>,   // VARCHAR(32)
    pub pressmark_name: Option<String>, // VARCHAR(64)
    pub location_id: Option<String>,    // VARCHAR(32)
    pub location_name: Option<String>,  // VARCHAR(128)
    pub book_barcode: Option<String>,   // VARCHAR(32)
    pub isbn: Option<String>,          // VARCHAR(32)
    pub pub_no: Option<i16>,            // SMALLINT
    pub publisher: Option<String>,     // VARCHAR(512)
    pub pub_date: Option<String>,       // VARCHAR(512)
    pub price: Option<String>,         // VARCHAR(32)
    pub pages: Option<String>,         // VARCHAR(32)
    pub summary: Option<String>,       // TEXT
    pub item_type: Option<i16>,          // SMALLINT
    pub remark: Option<String>,        // VARCHAR(256)
    pub origin_type: Option<i16>,        // SMALLINT
    pub create_type: Option<i16>,        // SMALLINT
    pub tenant_id: Option<i32>,         // INT
}
