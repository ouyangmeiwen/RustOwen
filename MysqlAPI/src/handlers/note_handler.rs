use crate::{
    models::{apiresponse_model::ApiResponse, note_model::NoteModel},
    schemas::note_schema::{CreateNoteSchema, FilterOptions, UpdateNoteSchema},
    AppState,
};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use chrono::prelude::*;
use serde_json::json;
use uuid::Uuid;
//http://127.0.0.1:7788/api/notes
#[get("/notes")]
pub async fn note_list_handler(
    opts: web::Query<FilterOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx::query_as!(
        NoteModel,
        "SELECT id, title, content, category, published, created_at, updated_at FROM notes ORDER BY id LIMIT ? OFFSET ?",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await;

    if let Err(_) = query_result {
        return HttpResponse::InternalServerError()
            .json(ApiResponse::<()>::error("Error fetching notes"));
    }
    let notes = query_result.unwrap();
    HttpResponse::Ok().json(ApiResponse::success(notes))
}
#[post("/notes/")]
async fn create_note_handler(
    body: web::Json<CreateNoteSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let new_id = Uuid::new_v4().to_string(); // Generate new UUID

    // Insert new note
    let query_result = sqlx::query!(
        "INSERT INTO notes (id, title, content, category, published) VALUES (?, ?, ?, ?, ?)",
        new_id,
        body.title.to_string(),
        body.content.to_string(),
        body.category.to_owned().unwrap_or("".to_string()),
        body.published.unwrap_or(true) as i8 // Ensure it's of type i8
    )
    .execute(&data.db)
    .await;

    // Check insert result
    match query_result {
        Ok(_) => {
            // Retrieve the newly inserted note
            let note = sqlx::query_as!(
                NoteModel,
                "SELECT id, title, content, category, published, created_at, updated_at FROM notes WHERE id = ?",
                new_id
            )
            .fetch_one(&data.db)
            .await;

            match note {
                Ok(note) => HttpResponse::Ok().json(ApiResponse::success(note)),
                Err(e) => HttpResponse::InternalServerError()
                    .json(ApiResponse::<()>::error(&e.to_string())),
            }
        }
        Err(e) => {
            if e.to_string().contains("Duplicate entry") {
                HttpResponse::BadRequest().json(ApiResponse::<()>::error(
                    "Note with that title already exists",
                ))
            } else {
                HttpResponse::InternalServerError().json(ApiResponse::<()>::error(&e.to_string()))
            }
        }
    }
}

//http://127.0.0.1:7788/api/notes/1
#[get("/notes/{id}")]
async fn get_note_handler(
    path: web::Path<String>, // `id` 作为 String 处理
    data: web::Data<AppState>,
) -> impl Responder {
    let note_id = path.into_inner(); // 直接使用 String 类型的 `id`
    let query_result = sqlx::query_as!(
        NoteModel,
        "SELECT id, title, content, category, published, created_at, updated_at FROM notes WHERE id = ?",
        note_id
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(note) => HttpResponse::Ok().json(ApiResponse::success(note)),
        Err(_) => HttpResponse::NotFound().json(ApiResponse::<()>::error(&format!(
            "Note with ID: {} not found",
            note_id
        ))),
    }
}

//http://127.0.0.1:7788/api/notes/1
#[patch("/notes/{id}")]
async fn edit_note_handler(
    path: web::Path<String>,
    body: web::Json<UpdateNoteSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let note_id = path.into_inner(); // 直接使用 String 类型的 `id`

    // 查询现有的记录
    let query_result = sqlx::query_as!(
        NoteModel,
        "SELECT id, title, content, category, published, created_at, updated_at FROM notes WHERE id = ?",
        note_id
    )
    .fetch_one(&data.db)
    .await;

    // 如果查询不到该记录，返回 404 错误
    if query_result.is_err() {
        return HttpResponse::NotFound().json(ApiResponse::<()>::error(&format!(
            "Note with ID: {} not found",
            note_id
        )));
    }

    let note = query_result.unwrap();

    // 获取当前时间
    let now = Utc::now().naive_utc();

    // 处理发布字段，将 Option<bool> 转为 i8 (0 或 1)
    let published_value = body.published.unwrap_or(note.published.unwrap_or(0)) as i8;
    let category = body
        .category
        .as_deref()
        .unwrap_or_else(|| note.category.as_deref().unwrap_or_default());
    // 执行更新操作
    let query_result = sqlx::query!(
        "UPDATE notes SET title = ?, content = ?, category = ?, published = ?, updated_at = ? WHERE id = ?",
        body.title.as_deref().unwrap_or(&note.title),
        body.content.as_deref().unwrap_or(&note.content),
        category,
        published_value,
        now,
        note_id
    )
    .execute(&data.db)
    .await;

    match query_result {
        Ok(_) => {
            // 更新成功后，查询更新后的记录
            let updated_note = sqlx::query_as!(
                NoteModel,
                "SELECT id, title, content, category, published, created_at, updated_at FROM notes WHERE id = ?",
                note_id
            )
            .fetch_one(&data.db)
            .await;

            match updated_note {
                Ok(updated_note) => HttpResponse::Ok().json(ApiResponse::success(updated_note)),
                Err(e) => HttpResponse::InternalServerError()
                    .json(ApiResponse::<()>::error(&e.to_string())),
            }
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(&e.to_string()))
        }
    }
}
#[delete("/notes/{id}")]
async fn delete_note_handler(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let note_id = path.into_inner(); // 直接使用 String 类型的 `id`
    let rows_affected = sqlx::query!("DELETE FROM notes WHERE id = ?", note_id)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        return HttpResponse::NotFound().json(ApiResponse::<()>::error(&format!(
            "Note with ID: {} not found",
            note_id
        )));
    }
    //HttpResponse::NoContent().finish()
    HttpResponse::Ok().json(ApiResponse::<()>::success_without_data())
}
