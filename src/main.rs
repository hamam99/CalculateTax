mod constant;

mod model;

use std::sync::{Arc, Mutex};

use crate::model::{request_data::RequestData, response::Response, tax_record::TaxRecord};

mod helper;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use helper::get_salary_taxed::get_salary_taxed;
use sqlite::Connection;

fn sql_escape_string(value: &str) -> String {
    value.replace('\'', "''")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let connection = sqlite::open(":memory:").unwrap();

    let query = "CREATE TABLE IF NOT EXISTS salary (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT,
        salary_per_month INTEGER,
        is_already_married INTEGER,
        number_of_children INTEGER,
        salary_taxed INTEGER,
        tax_to_pay INTEGER
    )";
    connection.execute(query).unwrap();

    let db_connection = web::Data::new(Arc::new(Mutex::new(connection)));
    HttpServer::new(move || {
        App::new()
            .app_data(db_connection.clone())
            .service(hello)
            .service(get_tax_all)
            .service(insert_tax)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[get("/tax")]
async fn get_tax_all(db: web::Data<Arc<Mutex<Connection>>>) -> impl Responder {
    let query = "SELECT * FROM salary";

    let mut items: Vec<TaxRecord> = Vec::new();
    db.lock()
        .unwrap()
        .iterate(query, |pairs| {
            let mut id: i64 = 0;
            let mut name = String::new();
            let mut salary_per_month: i64 = 0;
            let mut is_already_married: bool = false;
            let mut number_of_children: i64 = 0;
            let mut salary_taxed: i64 = 0;
            let mut tax_to_pay: i64 = 0;

            for &(column, value) in pairs.iter() {
                let value_str = value.unwrap_or_default();
                match column {
                    "id" => id = value_str.parse().unwrap_or(0),
                    "name" => name = value_str.to_string(),
                    "salary_per_month" => salary_per_month = value_str.parse().unwrap_or(0),
                    "is_already_married" => {
                        let flag: i64 = value_str.parse().unwrap_or(0);
                        is_already_married = flag != 0;
                    }
                    "number_of_children" => {
                        number_of_children = value_str.parse().unwrap_or(0);
                    }
                    "salary_taxed" => salary_taxed = value_str.parse().unwrap_or(0),
                    "tax_to_pay" => tax_to_pay = value_str.parse().unwrap_or(0),
                    _ => {}
                }
            }

            items.push(TaxRecord {
                id,
                name,
                salary_per_month,
                is_already_married,
                number_of_children,
                salary_taxed,
                tax_to_pay,
            });

            true
        })
        .unwrap();

    HttpResponse::Ok().json(items)
}

#[post("/tax")]
async fn insert_tax(
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
        "INSERT INTO salary (name, salary_per_month, is_already_married, number_of_children, salary_taxed, tax_to_pay) VALUES ('{}', {}, {}, {}, {}, {})",
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
