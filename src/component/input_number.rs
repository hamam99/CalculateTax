use std::io::{self, Write};
pub fn input_number(prompt: &str) -> u32 {
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
