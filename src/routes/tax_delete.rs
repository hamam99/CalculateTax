use std::sync::{Arc, Mutex};

use actix_web::{HttpResponse, Responder, delete, get, web};
use sqlite::Connection;

use crate::model::response::Response;

#[delete("/tax/{id}")]
pub async fn tax_delete(
    db: web::Data<Arc<Mutex<Connection>>>,
    id: web::Path<i64>,
) -> impl Responder {
    let query = format!("DELETE FROM TAX WHERE id = {}", id);

    db.lock().unwrap().execute(query).unwrap();

    let obj = Response {
        message: "Deletion success".to_string(),
    };
    HttpResponse::Ok().json(obj)
}
