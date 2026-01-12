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

    db.lock().unwrap().execute(query).unwrap();

    let obj = Response {
        message: "Success".to_string(),
    };
    HttpResponse::Ok().json(obj)
}

fn sql_escape_string(value: &str) -> String {
    value.replace('\'', "''")
}
