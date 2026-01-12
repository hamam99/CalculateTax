use std::sync::{Arc, Mutex};

use actix_web::{HttpResponse, Responder, get, web};
use sqlite::Connection;

use crate::model::tax_record::TaxRecord;

#[get("/tax")]
pub async fn tax_get_all(db: web::Data<Arc<Mutex<Connection>>>) -> impl Responder {
    let query = "SELECT * FROM TAX";

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
