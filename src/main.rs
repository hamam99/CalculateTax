use std::io::{self, Write};
// #[path = "constant/data_ptkp.rs"]
// mod data_ptkp;

#[path = "helper/get_tax_rate.rs"]
mod get_tax_rate;
use crate::get_tax_rate::get_tax_rate;

#[path = "helper/get_tax_cut.rs"]
mod get_tax_cut;
use crate::get_tax_cut::get_tax_cut;

fn main() {

    let name = input_string("Enter your name: ");
    let salary_per_month = input_number("Salary per month: ");
    let is_already_married = input_yes_no("Is already married? (Y/N): ");
    let number_of_children = if is_already_married {
        input_number("Number of children: ")
    } else {
        0
    };


    let salary_per_year:u128 = salary_per_month as u128 * 12;
    let total_tax_cut:u128 = get_tax_cut(&is_already_married, &number_of_children) as u128;
    let salary_taxed:u128 = salary_per_year.saturating_sub(total_tax_cut);

    println!("\n\nName : {}\nSalary per month : Rp {}\nIs already married? : {}\nNumber of children : {}\n", name, salary_per_month, is_already_married, number_of_children);

    if salary_taxed == 0 {
        println!("Tax to pay per year : Rp {}", 0);
        return;
    }
    let tax_rate = get_tax_rate(salary_taxed);
    let tax_to_pay:u128 = salary_taxed * tax_rate as u128 / 100;
    
    println!("Tax to pay per year : {}", tax_to_pay);
    
}

fn input_string(prompt: &str) -> String {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let input = input.trim();

        if input.is_empty() {
            println!("❌ Input cannot be empty.\n");
            continue;
        }

        return input.to_string();
    }
}

fn input_number(prompt: &str) -> u32 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let input = input.trim();

        if input.is_empty() {
            println!("❌ Input cannot be empty.\n");
            continue;
        }

        match input.parse::<u32>() {
            Ok(num) => {
                return num
            },
            Err(_) => println!("❌ Invalid number. Please try again.\n"),
        }
    }
}

fn input_yes_no(prompt: &str) -> bool {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let input = input.trim().to_lowercase();

        if input.is_empty() {
            println!("❌ Input cannot be empty.\n");
            continue;
        }

        if input.len() != 1 {
            println!("❌ Input only accept Y / N.\n");      
            continue;
        }


        match input.as_str() {
            "y" | "Y"  => return true,
            "n" | "N" => return false,
            _ => println!("❌ Invalid input. Please try again.\n"),
        }
    }
}
