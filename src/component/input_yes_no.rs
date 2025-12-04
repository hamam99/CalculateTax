use std::io::{self, Write};

pub fn input_yes_no(prompt: &str) -> bool {
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
