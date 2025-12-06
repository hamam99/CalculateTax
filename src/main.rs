mod constant;

mod component;
use crate::component::input_number::input_number;
use crate::component::input_string::input_string;
use crate::component::input_yes_no::input_yes_no;

mod helper;
use helper::get_salary_taxed::get_salary_taxed;

fn main() {
    let name: String = input_string("Enter your name: ");
    let salary_per_month = input_number("Salary per month: ");
    let is_already_married = input_yes_no("Is already married? (Y/N): ");
    let number_of_children = if is_already_married {
        input_number("Number of children: ")
    } else {
        0
    };

    let [salary_taxed, tax_to_pay] =
        get_salary_taxed(&salary_per_month, &is_already_married, &number_of_children);

    println!(
        "\n\nName : {}\nSalary per month : Rp {}\nIs already married? : {}\nNumber of children : {}\n",
        name, salary_per_month, is_already_married, number_of_children
    );

    println!(
        "Tax to pay per year : Rp {}",
        if salary_taxed > 0 { tax_to_pay } else { 0 }
    );
}
