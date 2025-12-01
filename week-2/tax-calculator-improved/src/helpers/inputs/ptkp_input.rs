use std::io::{self, Write};

use crate::constants::ptkp::Ptkp;

pub fn get() -> Ptkp {
    print!("PTKP tier: ");
    io::stdout().flush().expect("Failed to flush stdout");
    
    let mut ptkp_tier_input = String::new();
    io::stdin() 
        .read_line(&mut ptkp_tier_input)
        .expect("Failed to read line");
        
    let ptkp_tier: Ptkp = ptkp_tier_input.trim().parse().unwrap();
    
    return ptkp_tier;
}
