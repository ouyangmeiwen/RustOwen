use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct FilterOptions {
    pub Page: Option<usize>,  // 改为 PascalCase
    pub Limit: Option<usize>, // 改为 PascalCase
}

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub Id: String, // 改为 PascalCase
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateLibItemSchema {
    pub Title: String,          // 改为 PascalCase
    pub Author: Option<String>, // 改为 PascalCase
    pub Barcode: String,        // 改为 PascalCase
    #[serde(skip_serializing_if = "Option::is_none")]
    pub CallNo: Option<String>, // 改为 PascalCase
    #[serde(skip_serializing_if = "Option::is_none")]
    pub PreCallNo: Option<String>, // 改为 PascalCase
    #[serde(skip_serializing_if = "Option::is_none")]
    pub CatalogCode: Option<String>, // 改为 PascalCase
    pub ItemState: u8,          // 改为 PascalCase
    #[serde(skip_serializing_if = "Option::is_none")]
    pub PressmarkId: Option<String>, // 改为 PascalCase
    #[serde(skip_serializing_if = "Option::is_none")]
    pub PressmarkName: Option<String>, // 改为 PascalCase
    #[serde(skip_serializing_if = "Option::is_none")]
    pub LocationId: Option<String>, // 改为 PascalCase
    #[serde(skip_serializing_if = "Option::is_none")]
    pub LocationName: Option<String>, // 改为 PascalCase
    #[serde(skip_serializing_if = "Option::is_none")]
    pub BookBarcode: Option<String>, // 改为 PascalCase
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ISBN: Option<String>, // 改为 PascalCase
    #[serde(skip_serializing_if = "Option::is_none")]
    pub PubNo: Option<i16>, // 改为 PascalCase
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Publisher: Option<String>, // 改为 PascalCase
    #[serde(skip_serializing_if = "Option::is_none")]
    pub PubDate: Option<String>, // 改为 PascalCase
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Price: Option<String>, // 改为 PascalCase
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Pages: Option<String>, // 改为 PascalCase
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Summary: Option<String>, // 改为 PascalCase
    pub ItemType: u8,           // 改为 PascalCase
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Remark: Option<String>, // 改为 PascalCase
    pub OriginType: u8,         // 改为 PascalCase
    pub CreateType: u8,         // 改为 PascalCase
    pub TenantId: i32,          // 改为 PascalCase
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateLibItemSchema {
    pub Title: Option<String>,         // 改为 PascalCase
    pub Author: Option<String>,        // 改为 PascalCase
    pub Barcode: Option<String>,       // 改为 PascalCase
    pub CallNo: Option<String>,        // 改为 PascalCase
    pub PreCallNo: Option<String>,     // 改为 PascalCase
    pub CatalogCode: Option<String>,   // 改为 PascalCase
    pub ItemState: Option<u8>,         // 改为 PascalCase
    pub PressmarkId: Option<String>,   // 改为 PascalCase
    pub PressmarkName: Option<String>, // 改为 PascalCase
    pub LocationId: Option<String>,    // 改为 PascalCase
    pub LocationName: Option<String>,  // 改为 PascalCase
    pub BookBarcode: Option<String>,   // 改为 PascalCase
    pub ISBN: Option<String>,          // 改为 PascalCase
    pub PubNo: Option<i16>,            // 改为 PascalCase
    pub Publisher: Option<String>,     // 改为 PascalCase
    pub PubDate: Option<String>,       // 改为 PascalCase
    pub Price: Option<String>,         // 改为 PascalCase
    pub Pages: Option<String>,         // 改为 PascalCase
    pub Summary: Option<String>,       // 改为 PascalCase
    pub ItemType: Option<u8>,          // 改为 PascalCase
    pub Remark: Option<String>,        // 改为 PascalCase
    pub OriginType: Option<u8>,        // 改为 PascalCase
    pub CreateType: Option<u8>,        // 改为 PascalCase
    pub TenantId: Option<i32>,         // 改为 PascalCase
}
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ItemsExcelImportInput {
    pub Path: String,
    pub Sheet: String,
    pub Title: i32,
    pub Author: i32,
    pub Tid: i32,
    pub CallNo: i32,
    pub ISBN: i32,
    pub CatalogCode: i32,
    pub Publisher: i32,
    pub PubDate: i32,
    pub Price: i32,
    pub Pages: i32,
    pub Barcode: i32,
    pub Locationname: i32,
    pub Tenantid: i32,
}
