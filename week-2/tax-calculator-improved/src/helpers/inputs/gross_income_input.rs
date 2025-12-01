use std::io::{self, Write};
use std::process;

const MAX_INCOME: u64 = 5_000_000_000;

pub fn get() -> u64 {
    print!("Gross income: ");
    io::stdout().flush().expect("Failed to flush stdout");
    
    let mut gross_income_input = String::new();
    io::stdin() 
        .read_line(&mut gross_income_input)
        .expect("Failed to read line");
        
    let gross_income: u64 = match gross_income_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: Gross income is not a valid positive integer.");
            process::exit(1); 
        }
    };
    if gross_income > MAX_INCOME {
        eprintln!("Error: The gross income {} exceeds the maximum limit of {}.", gross_income, MAX_INCOME);
        process::exit(1);
    }
    
    return gross_income;
}

