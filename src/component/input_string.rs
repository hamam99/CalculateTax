use std::io::{self, Write};
pub fn input_string(prompt: &str) -> String {
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
