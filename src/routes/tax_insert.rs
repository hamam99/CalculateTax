use std::sync::{Arc, Mutex};

use actix_web::{HttpResponse, Responder, post, web};
use sqlite::Connection;

use crate::{
    helper::get_salary_taxed::get_salary_taxed,
    model::{request_data::RequestData, response::Response},
};

#[post("/tax")]
async fn tax_insert(
    db: web::Data<Arc<Mutex<Connection>>>,
    item: web::Json<RequestData>,
) -> impl Responder {
    if item.salary_per_month <= 0 {
        return HttpResponse::BadRequest().json(Response {
            message: "Salary per month must be greater than 0".to_string(),
        });
    }

    let [salary_taxed, tax_to_pay] = get_salary_taxed(
        &item.salary_per_month,
        &item.is_already_married,
        &item.number_of_children,
    );

    let name = sql_escape_string(&item.name);
    let is_already_married = if item.is_already_married { 1 } else { 0 };
    let query = format!(
        "INSERT INTO TAX(name, salary_per_month, is_already_married, number_of_children, salary_taxed, tax_to_pay) VALUES ('{}', {}, {}, {}, {}, {})",
        name,
        item.salary_per_month,
        is_already_married,
        item.number_of_children,
        salary_taxed,
        tax_to_pay
    );

    let db_guard = match db.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let result = db_guard.execute(query);

    match result {
        Ok(_) => HttpResponse::Ok().json(Response {
            message: "Insertion success".to_string(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(Response {
            message: format!("Database error: {}", e),
        }),
    }
}

fn sql_escape_string(value: &str) -> String {
    value.replace('\'', "''")
}
