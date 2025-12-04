use std::io::{self, Write};

#[path = "component/input_yes_no.rs"]
mod input_yes_no;
use crate::input_yes_no::input_yes_no;

#[path = "component/input_number.rs"]
mod input_number;
use crate::input_number::input_number;

#[path = "component/input_string.rs"]
mod input_string;
use crate::input_string::input_string;

#[path = "helper/get_salary_taxed.rs"]
mod get_salary_taxed;
use crate::get_salary_taxed::get_salary_taxed;



fn main() {
    let name = input_string("Enter your name: ");
    let salary_per_month = input_number("Salary per month: ");
    let is_already_married = input_yes_no("Is already married? (Y/N): ");
    let number_of_children = if is_already_married {
        input_number("Number of children: ")
    } else {
        0
    };

    let [salary_taxed, tax_to_pay] = get_salary_taxed( &salary_per_month, &is_already_married, &number_of_children);

    println!("\n\nName : {}\nSalary per month : Rp {}\nIs already married? : {}\nNumber of children : {}\n", name, salary_per_month, is_already_married, number_of_children);
    
    
    println!("Tax to pay per year : {}", if salary_taxed > 0 { tax_to_pay } else { 0 });
    
}
