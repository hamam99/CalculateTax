use std::io::{self, Write};

struct PTKP {
    tk: u128,
    k0: u128,
    k1: u128,
    k2: u128,
    k3: u128,
}

const DATA_PTKP: PTKP = PTKP {
    tk: 54000000,
    k0: 58500000,
    k1: 63000000,
    k2: 67500000,
    k3: 72000000,
};



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

fn get_tax_cut(is_already_married: &bool, number_of_children: &u32) -> u128 {
    if !(*is_already_married) {
        return DATA_PTKP.tk;
    }

    match number_of_children {
        0 => DATA_PTKP.k0,
        1 => DATA_PTKP.k1,
        2 => DATA_PTKP.k2,
        _ => DATA_PTKP.k3,
    }
}



fn get_tax_rate(salary_taxed: u128) -> u32 {
    match salary_taxed {
        0..=60_000_000 => 5,
        60_000_001..=250_000_000 => 15,
        250_000_001..=500_000_000 => 25,
        500_000_001..=5_000_000_000 => 30,
        _ => 35,
    }
}