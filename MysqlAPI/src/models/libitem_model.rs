use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
#[serde(rename_all = "PascalCase")] // 如果希望遵循数据库字段的命名方式
pub struct LibItemModel {
    pub Id: String,                                  // Corresponds to `Id`
    pub CreationTime: Option<NaiveDateTime>,         // Corresponds to `CreationTime`
    pub CreatorUserId: Option<i64>,                  // Corresponds to `CreatorUserId`
    pub LastModificationTime: Option<NaiveDateTime>, // Corresponds to `LastModificationTime`
    pub LastModifierUserId: Option<i64>,             // Corresponds to `LastModifierUserId`
    pub IsDeleted: u8,                               // Corresponds to `IsDeleted`
    pub DeleterUserId: Option<i64>,                  // Corresponds to `DeleterUserId`
    pub DeletionTime: Option<NaiveDateTime>,         // Corresponds to `DeletionTime`
    pub InfoId: Option<String>,                      // Corresponds to `InfoId`
    pub Title: String,                               // Corresponds to `Title`
    pub Author: Option<String>,                      // Corresponds to `Author`
    pub Barcode: String,                             // Corresponds to `Barcode`
    pub IsEnable: u8,                                // Corresponds to `IsEnable`
    pub CallNo: Option<String>,                      // Corresponds to `CallNo`
    pub PreCallNo: Option<String>,                   // Corresponds to `PreCallNo`
    pub CatalogCode: Option<String>,                 // Corresponds to `CatalogCode`
    pub ItemState: u8,                               // Corresponds to `ItemState`
    pub PressmarkId: Option<String>,                 // Corresponds to `PressmarkId`
    pub PressmarkName: Option<String>,               // Corresponds to `PressmarkName`
    pub LocationId: Option<String>,                  // Corresponds to `LocationId`
    pub LocationName: Option<String>,                // Corresponds to `LocationName`
    pub BookBarcode: Option<String>,                 // Corresponds to `BookBarcode`
    pub ISBN: Option<String>,                        // Corresponds to `ISBN`
    pub PubNo: Option<i16>,                          // Corresponds to `PubNo`
    pub Publisher: Option<String>,                   // Corresponds to `Publisher`
    pub PubDate: Option<String>,                     // Corresponds to `PubDate`
    pub Price: Option<String>,                       // Corresponds to `Price`
    pub Pages: Option<String>,                       // Corresponds to `Pages`
    pub Summary: Option<String>,                     // Corresponds to `Summary`
    pub ItemType: u8,                                // Corresponds to `ItemType`
    pub Remark: Option<String>,                      // Corresponds to `Remark`
    pub OriginType: u8,                              // Corresponds to `OriginType`
    pub CreateType: u8,                              // Corresponds to `CreateType`
    pub TenantId: i32,                               // Corresponds to `TenantId`
}
