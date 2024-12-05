use crate::{
    models::libitem_model::LibItemModel,
    schemas::libitem_schema::{CreateLibItemSchema, FilterOptions, UpdateLibItemSchema},
    AppState,
};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use chrono::prelude::*;
use serde_json::json;
use uuid::Uuid;

//http://127.0.0.1:7788/api/libitems
#[get("/libitems")]
pub async fn libitem_list_handler(
    opts: web::Query<FilterOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
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
    ORDER BY Id
    LIMIT ? OFFSET ?"#,
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await;
    if let Err(_) = query_result {
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error", "message": "Error fetching libitems"}));
    }

    let libitems = query_result.unwrap();

    HttpResponse::Ok().json(json!({
        "status": "success",
        "results": libitems.len(),
        "libitems": libitems
    }))
}

// Create libitem
#[post("/libitems/")]
async fn create_libitem_handler(
    body: web::Json<CreateLibItemSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let new_id = Uuid::new_v4().to_string(); // Generate new UUID

    let query_result = sqlx::query!(
        r#"
    INSERT INTO libitem (
        Id, Title, Barcode, IsEnable, ItemState, TenantId
    ) VALUES (?, ?, ?, ?, ?, ?)
    "#,
        new_id,
        body.Title.to_string(),
        body.Barcode.to_string(),
        (true) as i8,   // Ensure it's of type i8 (bool to i8)
        body.ItemState, // Default to 1 if None (u8)
        body.TenantId
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
                Ok(libitem) => HttpResponse::Ok().json(json!({
                    "status": "success",
                    "data": {"libitem": libitem}
                })),
                Err(e) => HttpResponse::InternalServerError().json(json!( {
                    "status": "error",
                    "message": e.to_string()
                })),
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": e.to_string()
        })),
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
        Ok(libitem) => HttpResponse::Ok().json(json!({
            "status": "success",
            "data": {"libitem": libitem}
        })),
        Err(_) => HttpResponse::NotFound().json(json!({
            "status": "fail",
            "message": format!("LibItem with ID: {} not found", libitem_id)
        })),
    }
}

// Update libitem
#[patch("/libitems/{id}")]
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
        return HttpResponse::NotFound().json(json!({
            "status": "fail",
            "message": format!("LibItem with ID: {} not found", libitem_id)
        }));
    }

    let libitem = query_result.unwrap();
    let now = Utc::now().naive_utc();
    let empty_string = "".to_string(); // 提前创建一个 String
    let query_result = sqlx::query!(
        r#"
    UPDATE libitem 
    SET 
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
                Ok(updated_libitem) => HttpResponse::Ok().json(json!({
                    "status": "success",
                    "data": {"libitem": updated_libitem}
                })),
                Err(e) => HttpResponse::InternalServerError().json(json!( {
                    "status": "error",
                    "message": e.to_string()
                })),
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(json!( {
            "status": "error",
            "message": e.to_string()
        })),
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
        return HttpResponse::NotFound().json(json!({
            "status": "fail",
            "message": format!("LibItem with ID: {} not found", libitem_id)
        }));
    }

    HttpResponse::NoContent().finish()
}
