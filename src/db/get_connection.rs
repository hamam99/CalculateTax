use std::sync::{Arc, Mutex};

use actix_web::web::{self, Data};
use sqlite::Connection;

pub fn get_connection() -> Data<Arc<Mutex<Connection>>> {
    let connection = sqlite::open(":memory:").unwrap();

    let query_create_table = "CREATE TABLE IF NOT EXISTS TAX(
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT,
        salary_per_month INTEGER,
        is_already_married INTEGER,
        number_of_children INTEGER,
        salary_taxed INTEGER,
        tax_to_pay INTEGER
    )";

    connection.execute(query_create_table).unwrap();

    let db_connection = web::Data::new(Arc::new(Mutex::new(connection)));

    return db_connection;
}
