use std::sync::{Arc, Mutex};

use actix_web::{HttpResponse, Responder, delete, get, web};
use sqlite::Connection;

use crate::model::response::Response;

#[delete("/tax/{id}")]
pub async fn tax_delete(
    db: web::Data<Arc<Mutex<Connection>>>,
    id: web::Path<i64>,
) -> impl Responder {
    let db_guard = match db.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };

    let query = format!("DELETE FROM TAX WHERE id = {}", id);
    let result = db_guard.execute(query);

    match result {
        Ok(_) => HttpResponse::Ok().json(Response {
            message: "Deletion success".to_string(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(Response {
            message: format!("Database error: {}", e),
        }),
    }
}
