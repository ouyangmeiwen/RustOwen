use crate::handlers::base_handle::check_auth;
use crate::models::apiresponse_model::ApiResponse;
use crate::models::appstate_model::AppState;
use crate::schemas::libitem_schema::ItemsExcelImportInput;
use crate::utils::file_utils::FileUtils;
use crate::{
    models::libitem_model::LibItemModel,
    schemas::libitem_schema::{CreateLibItemSchema, FilterOptions, UpdateLibItemSchema},
};
use actix_web::web::Json;
use actix_web::{body, delete, get, patch, post, web, HttpRequest, HttpResponse, Responder};
use calamine::RangeDeserializerBuilder;
use calamine::{open_workbook_auto, DataType, Reader};
use chrono::prelude::*;
use serde_json::json;
use std::error::Error;
use std::f32::consts::E;
use std::path;
use std::time::Instant;
use uuid::Uuid;

//http://127.0.0.1:7788/api/libitems
#[get("/libitems")]
pub async fn libitem_list_handler(
    opts: web::Query<FilterOptions>,
    data: web::Data<AppState>,
    req: HttpRequest, // 接收请求对象作为参数
) -> impl Responder {
    let mut user_id = String::new();
    let mut user_role = String::new();
    match check_auth(&req).await {
        Err(err) => {
            return HttpResponse::Unauthorized().json(ApiResponse::<()>::error(&err.to_string()));
        }
        Ok(claims) => {
            user_id = claims.user_id.to_string();
            println!("user_id:{}", &user_id);
            user_role = claims.username.to_string();
            println!("user_role:{}", &user_role);
        }
    }
    let limit = opts.Limit.unwrap_or(10);
    let offset = (opts.Page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx::query_as!(
        LibItemModel,
        r#"
    SELECT 
        Id, 
        CreationTime, 
        CreatorUserId, 
        LastModificationTime, 
        LastModifierUserId, 
        IsDeleted, 
        DeleterUserId, 
        DeletionTime, 
        InfoId, 
        Title, 
        Author, 
        Barcode, 
        IsEnable, 
        CallNo, 
        PreCallNo, 
        CatalogCode, 
        ItemState, 
        PressmarkId, 
        PressmarkName, 
        LocationId, 
        LocationName, 
        BookBarcode, 
        ISBN, 
        PubNo, 
        Publisher, 
        PubDate, 
        Price, 
        Pages, 
        Summary, 
        ItemType, 
        Remark, 
        OriginType, 
        CreateType, 
        TenantId 
    FROM libitem
    ORDER BY CreationTime DESC 
    LIMIT ? OFFSET ?"#,
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await;
    if let Err(_) = query_result {
        return HttpResponse::InternalServerError()
            .json(ApiResponse::<()>::error("Error fetching libitems"));
    }
    let libitems = query_result.unwrap();

    // 获取总数
    let count_result_query = sqlx::query!(
        r#"
    SELECT COUNT(*) as total
    FROM libitem
    "#,
    )
    .fetch_one(&data.db)
    .await;
    // 处理结果
    let mut total_count: i64 = 0;
    match count_result_query {
        Ok(count_row) => {
            total_count = count_row.total;
        }
        Err(_) => {
            total_count = 0;
        }
    }
    HttpResponse::Ok().json(ApiResponse::success_with_count(libitems, total_count))
}

// Create libitem
#[post("/libitems/create")]
async fn create_libitem_handler(
    body: web::Json<CreateLibItemSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let new_id = Uuid::new_v4().to_string().replace("-", ""); // 去掉破折号
    let empty_string = "".to_string(); // 提前创建一个 String
    let now: NaiveDateTime = Utc::now().naive_utc();
    let query_result = sqlx::query!(
        r#"
        INSERT INTO libitem (
            Id, CreationTime,IsDeleted, Title, Author, Barcode, IsEnable,CallNo, PreCallNo, CatalogCode, ItemState,
            PressmarkId, PressmarkName, LocationId, LocationName, BookBarcode, ISBN, PubNo,
            Publisher, PubDate, Price, Pages, Summary, ItemType, Remark, OriginType, CreateType, TenantId
        ) VALUES (?,?, ?, ?, ?, ?, ?, ?, ?, ?,?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,?)
        "#,
        new_id,                                      // Id
        now,
        (false) as i8,                               // IsDeleted
        body.Title.to_string(),                      // Title
        body.Author.as_ref().unwrap_or(&empty_string),     // Author
        body.Barcode.to_string(),                    // Barcode
        (true) as i8,                               // IsDeleted
        body.CallNo.as_ref().unwrap_or(&empty_string),     // CallNo
        body.PreCallNo.as_ref().unwrap_or(&empty_string),  // PreCallNo
        body.CatalogCode.as_ref().unwrap_or(&empty_string),// CatalogCode
        body.ItemState,                              // ItemState
        body.PressmarkId.as_ref().unwrap_or(&empty_string), // PressmarkId
        body.PressmarkName.as_ref().unwrap_or(&empty_string), // PressmarkName
        body.LocationId.as_ref().unwrap_or(&empty_string), // LocationId
        body.LocationName.as_ref().unwrap_or(&empty_string), // LocationName
        body.BookBarcode.as_ref().unwrap_or(&empty_string), // BookBarcode
        body.ISBN.as_ref().unwrap_or(&empty_string), // ISBN
        body.PubNo.unwrap_or_default(),              // PubNo
        body.Publisher.as_ref().unwrap_or(&empty_string), // Publisher
        body.PubDate.as_ref().unwrap_or(&empty_string), // PubDate
        body.Price.as_ref().unwrap_or(&empty_string), // Price
        body.Pages.as_ref().unwrap_or(&empty_string), // Pages
        body.Summary.as_ref().unwrap_or(&empty_string), // Summary
        body.ItemType,                               // ItemType
        body.Remark.as_ref().unwrap_or(&empty_string), // Remark
        body.OriginType,                             // OriginType
        body.CreateType,                             // CreateType
        body.TenantId                                // TenantId
    )
    .execute(&data.db)
    .await;
    match query_result {
        Ok(_) => {
            let libitem =
                sqlx::query_as!(LibItemModel, "SELECT * FROM libitem WHERE id = ?", new_id)
                    .fetch_one(&data.db)
                    .await;

            match libitem {
                Ok(libitem) => HttpResponse::Ok().json(ApiResponse::success(libitem)),
                Err(e) => HttpResponse::InternalServerError()
                    .json(ApiResponse::<()>::error(&e.to_string())),
            }
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(&e.to_string()))
        }
    }
}

// Get libitem by id
#[get("/libitems/{id}")]
async fn get_libitem_handler(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let libitem_id = path.into_inner();
    let query_result = sqlx::query_as!(
        LibItemModel,
        "SELECT * FROM libitem WHERE id = ?",
        libitem_id
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(libitem) => HttpResponse::Ok().json(ApiResponse::success(libitem)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(&format!(
            "LibItem with ID: {} not found",
            libitem_id
        ))),
    }
}
//http://127.0.0.1:7788/api/libitems/getitembybarcode/0307900
#[get("/libitems/getitembybarcode/{barcode}")]
async fn get_item_bybarcode_handle(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> impl Responder {
    let barcode = path.into_inner();
    let item_query = sqlx::query_as!(
        LibItemModel,
        "select * from libitem where barcode=?",
        barcode
    )
    .fetch_all(&data.db)
    .await;
    match item_query {
        Ok(libitems) => HttpResponse::Ok().json(ApiResponse::success(libitems)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error("")),
    }
}

// Update libitem
#[patch("/libitems/update/{id}")]
async fn edit_libitem_handler(
    path: web::Path<String>,
    body: web::Json<UpdateLibItemSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let libitem_id = path.into_inner();

    let query_result = sqlx::query_as!(
        LibItemModel,
        "SELECT * FROM libitem WHERE id = ?",
        libitem_id
    )
    .fetch_one(&data.db)
    .await;

    if query_result.is_err() {
        return HttpResponse::NotFound().json(ApiResponse::<()>::error(&format!(
            "LibItem with ID: {} not found",
            libitem_id
        )));
    }

    let libitem = query_result.unwrap();
    let now = Utc::now().naive_utc();
    let empty_string = "".to_string(); // 提前创建一个 String
    let query_result = sqlx::query!(
        r#"
    UPDATE libitem 
    SET 
        LastModificationTime=?,
        Title = ?, 
        Barcode = ?, 
        ItemState = ?, 
        PressmarkId = ?, 
        PressmarkName = ?, 
        LocationId = ?, 
        LocationName = ?, 
        BookBarcode = ?, 
        ISBN = ?, 
        PubNo = ?, 
        Publisher = ?, 
        PubDate = ?, 
        Price = ?, 
        Pages = ?, 
        Summary = ?, 
        ItemType = ?, 
        Remark = ?, 
        OriginType = ?, 
        CreateType = ?, 
        TenantId = ?
    WHERE Id = ?
    "#,
        now,
        body.Title.as_deref().unwrap_or(&libitem.Title), // 使用 as_deref() 转换为 &str
        body.Barcode.as_deref().unwrap_or(&libitem.Barcode),
        body.ItemState.unwrap_or(libitem.ItemState) as u8,
        body.PressmarkId
            .as_ref()
            .unwrap_or(&libitem.PressmarkId.as_ref().unwrap_or(&empty_string)), // 使用 as_ref() 获取 Option<&String>
        body.PressmarkName
            .as_ref()
            .unwrap_or(&libitem.PressmarkName.as_ref().unwrap_or(&empty_string)), // 使用 as_ref() 获取 Option<&String>
        body.LocationId
            .as_ref()
            .unwrap_or(&libitem.LocationId.as_ref().unwrap_or(&empty_string)), // 使用 as_ref() 获取 Option<&String>
        body.LocationName
            .as_ref()
            .unwrap_or(&libitem.LocationName.as_ref().unwrap_or(&empty_string)), // 使用 as_ref() 获取 Option<&String>
        body.BookBarcode
            .as_ref()
            .unwrap_or(&libitem.BookBarcode.as_ref().unwrap_or(&empty_string)), // 使用 as_ref() 获取 Option<&String>
        body.ISBN
            .as_ref()
            .unwrap_or(&libitem.ISBN.as_ref().unwrap_or(&empty_string)), // 使用 as_ref() 获取 Option<&String>
        body.PubNo.unwrap_or(libitem.PubNo.unwrap_or(0i16)),
        body.Publisher
            .as_ref()
            .unwrap_or(&libitem.Publisher.as_ref().unwrap_or(&empty_string)), // 使用 as_ref() 获取 Option<&String>
        body.PubDate
            .as_ref()
            .unwrap_or(&libitem.PubDate.as_ref().unwrap_or(&empty_string)), // 使用 as_ref() 获取 Option<&String>
        body.Price
            .as_ref()
            .unwrap_or(&libitem.Price.as_ref().unwrap_or(&empty_string)), // 使用 as_ref() 获取 Option<&String>
        body.Pages
            .as_ref()
            .unwrap_or(&libitem.Pages.as_ref().unwrap_or(&empty_string)), // 使用 as_ref() 获取 Option<&String>
        body.Summary
            .as_ref()
            .unwrap_or(&libitem.Summary.as_ref().unwrap_or(&empty_string)), // 使用 as_ref() 获取 Option<&String>
        body.ItemType.unwrap_or(libitem.ItemType) as u8,
        body.Remark
            .as_ref()
            .unwrap_or(&libitem.Remark.as_ref().unwrap_or(&empty_string)), // 使用 as_ref() 获取 Option<&String>
        body.OriginType.unwrap_or(libitem.OriginType) as u8,
        body.CreateType.unwrap_or(libitem.CreateType) as u8,
        body.TenantId.unwrap_or(libitem.TenantId),
        libitem_id // 使用正确的 ID 参数
    )
    .execute(&data.db)
    .await;

    match query_result {
        Ok(_) => {
            let updated_libitem = sqlx::query_as!(
                LibItemModel,
                "SELECT * FROM libitem WHERE id = ?",
                libitem_id
            )
            .fetch_one(&data.db)
            .await;

            match updated_libitem {
                Ok(updated_libitem) => {
                    HttpResponse::Ok().json(ApiResponse::success(updated_libitem))
                }
                Err(e) => HttpResponse::InternalServerError()
                    .json(ApiResponse::<()>::error(&e.to_string())),
            }
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(&e.to_string()))
        }
    }
}

// Delete libitem
#[delete("/libitems/{id}")]
async fn delete_libitem_handler(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> impl Responder {
    let libitem_id = path.into_inner();
    let rows_affected = sqlx::query!("DELETE FROM libitem WHERE id = ?", libitem_id)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        return HttpResponse::NotFound().json(ApiResponse::<()>::error(&format!(
            "LibItem with ID: {} not found",
            libitem_id
        )));
    }
    //HttpResponse::NoContent().finish()
    HttpResponse::Ok().json(ApiResponse::<()>::success_without_data())
}

#[get("/libitems/import")]
pub async fn libitem_import_handler(
    body: web::Json<ItemsExcelImportInput>,
    data: web::Data<AppState>,
) -> impl Responder {
    if (FileUtils::exists(&body.Path)) {
        match open_workbook_auto(&body.Path) {
            Ok(mut workbook) => {
                // Start measuring time for a specific code block
                let start = Instant::now();
                if let Some(Ok(range)) = workbook.worksheet_range(&body.Sheet) {
                    for (row_index, row) in range.rows().enumerate() {
                        // Iterate over each cell in the row with a column index
                        for (col_index, cell) in row.iter().enumerate() {
                            match cell {
                                DataType::Empty => println!("Empty"),
                                DataType::String(s) => println!("String: {}", s), // Handle as a string
                                DataType::Float(f) => println!("Float: {}", f),
                                DataType::Int(i) => println!("Int: {}", i),
                                DataType::Bool(b) => println!("Bool: {}", b),
                                _ => println!("Other: {:?}", cell),
                            }
                        }
                    }
                } else {
                    return HttpResponse::NotFound().json(ApiResponse::<()>::error(&format!(
                        "文件{}打开{}失败！",
                        &body.Path, &body.Sheet
                    )));
                }
                // Code after the time-measured block
                let duration = start.elapsed();
                println!("Time taken for the block: {} seconds", duration.as_secs());
            }
            Err(e) => {
                return HttpResponse::NotFound().json(ApiResponse::<()>::error(&format!(
                    "文件{}打开失败！",
                    &body.Path
                )));
            }
        }
    } else {
        return HttpResponse::NotFound().json(ApiResponse::<()>::error(&format!(
            "文件{}不存在",
            &body.Path
        )));
    }
    HttpResponse::Ok().json(ApiResponse::<()>::success_with_msg("导入成功".to_string()))
}
