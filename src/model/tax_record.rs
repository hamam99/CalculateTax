use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct TaxRecord {
    pub id: i64,
    pub name: String,
    pub salary_per_month: i64,
    pub is_already_married: bool,
    pub number_of_children: i64,
    pub salary_taxed: i64,
    pub tax_to_pay: i64,
}
