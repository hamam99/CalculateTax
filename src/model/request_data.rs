use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RequestData {
    pub name: String,
    pub salary_per_month: u32,
    pub is_already_married: bool,
    pub number_of_children: u32,
}
