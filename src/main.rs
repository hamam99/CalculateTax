use actix_web::{App, HttpServer};

mod constant;
mod db;
mod helper;
mod model;
mod routes;

use db::get_connection::get_connection;
use routes::{hello, tax_delete, tax_get_all, tax_get_detail, tax_insert, tax_update};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(get_connection())
            .service(hello::hello)
            .service(tax_get_all::tax_get_all)
            .service(tax_insert::tax_insert)
            .service(tax_get_detail::tax_get_detail)
            .service(tax_delete::tax_delete)
            .service(tax_update::tax_update)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
