use crate::{
    models::libitem_model::LibItemModel,
    schemas::libitem_schema::{CreateLibItemSchema, FilterOptions, UpdateLibItemSchema},
    AppState,
};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use chrono::prelude::*;
use serde_json::json;
use uuid::Uuid;



//http://127.0.0.1:7788/api/lib_items
#[get("/libitems")]
pub async fn lib_item_list_handler(
    opts: web::Query<FilterOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx::query_as!(
        LibItemModel,
        "SELECT * FROM libitem  ORDER BY creation_time DESC LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await;
    if let Err(e) = query_result {
        let message = format!("Error fetching data: {:?}", e);
        return HttpResponse::InternalServerError().json(json!({"status": "error", "message": message}));
    }
    
    let lib_items = query_result.unwrap();
    
    let json_response: serde_json::Value = serde_json::json!({
        "status": "success",
        "results": lib_items.len(),
        "lib_items": lib_items
    });
    
    HttpResponse::Ok().json(json_response)
    
}

#[post("/libitems/create")]
async fn create_lib_item_handler(
    body: web::Json<CreateLibItemSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let new_id = Uuid::new_v4().to_string().replace("-", "");  // 去掉破折号
    let now: NaiveDateTime = Utc::now().naive_utc();
    let query_result = sqlx::query_as!(
        LibItemModel,
        r#"
            INSERT INTO libitem (
                id, creation_time, title, author, barcode, item_state, item_type, tenant_id,
                is_deleted, is_enable, origin_type, create_type
            ) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12) 
            RETURNING *
        "#,
        new_id,  // 自动生成UUID作为id
        now,                  // 使用当前时间作为 creation_time
        body.title.to_string(),      // title
        body.author.to_owned(),     // author
        body.barcode.to_string(),   // barcode
        body.item_state,             // item_state
        body.item_type,              // item_type
        body.tenant_id,              // tenant_id
        false,                       // is_deleted 默认为 false
        true,                        // is_enable 默认为 true
        body.origin_type,            // origin_type
        body.create_type             // create_type
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(lib_item) => {
            let lib_item_response = serde_json::json!({
                "status": "success",
                "data": serde_json::json!({
                    "lib_item": lib_item
                })
            });

            return HttpResponse::Ok().json(lib_item_response);
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                return HttpResponse::BadRequest()
                    .json(serde_json::json!({"status": "fail", "message": "Item with that barcode already exists"}));
            }

            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error", "message": format!("{:?}", e)}));
        }
    }
}

#[get("/libitems/{id}")]
async fn get_lib_item_handler(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> impl Responder {
    let item_id = path.into_inner();
    let query_result = sqlx::query_as!(
        LibItemModel, 
        "SELECT * FROM libitem WHERE id = $1 ", 
        item_id
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(lib_item) => {
            let lib_item_response = serde_json::json!({
                "status": "success",
                "data": serde_json::json!({
                    "lib_item": lib_item
                })
            });

            return HttpResponse::Ok().json(lib_item_response);
        }
        Err(_) => {
            let message = format!("Lib item with ID: {} not found", item_id);
            return HttpResponse::NotFound()
                .json(serde_json::json!({"status": "fail", "message": message}));
        }
    }
}

#[patch("/libitems/update/{id}")]
async fn edit_lib_item_handler(
    path: web::Path<String>,
    body: web::Json<UpdateLibItemSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let item_id = path.into_inner();
    let query_result = sqlx::query_as!(
        LibItemModel,
        "SELECT * FROM libitem WHERE id = $1",
        item_id
    )
    .fetch_one(&data.db)
    .await;

    if query_result.is_err() {
        let message = format!("Lib item with ID: {} not found", item_id);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status": "fail", "message": message}));
    }

    let lib_item = query_result.unwrap();
    let empty_string = "".to_string(); // 提前创建一个 String
    let query_result = sqlx::query_as!(
        LibItemModel,
        "UPDATE libitem SET title = $1, author = $2, barcode = $3, item_state = $4, item_type = $5 
         WHERE id = $6 RETURNING *",
        body.title.to_owned().unwrap_or(lib_item.title),
        body.author
        .as_ref()
        .unwrap_or(&lib_item.author.as_ref().unwrap_or(&empty_string)), // 使用 as_ref() 获取 Option<&String>
        body.barcode.to_owned().unwrap_or(lib_item.barcode),
        body.item_state.unwrap_or(lib_item.item_state),
        body.item_type.unwrap_or(lib_item.item_type),
        item_id
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(updated_lib_item) => {
            let updated_lib_item_response = serde_json::json!({
                "status": "success",
                "data": updated_lib_item
            });

            return HttpResponse::Ok().json(updated_lib_item_response);
        }
        Err(err) => {
            let message = format!("Error: {:?}", err);
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error", "message": message}));
        }
    }
}

#[delete("/libitems/{id}")]
async fn delete_lib_item_handler(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> impl Responder {
    let item_id = path.into_inner();

    let rows_affected = sqlx::query!( "DELETE FROM libitem WHERE id = $1", item_id)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();


    if rows_affected == 0 {
        let message = format!("Lib item with ID: {} not found", item_id);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status": "fail", "message": message}));
    }

    // 成功删除，返回 204 No Content
    HttpResponse::NoContent().finish()
}
