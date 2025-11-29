use std::io::{self, Write};
use std::str::FromStr;
use std::process;

const MAX_INCOME: u64 = 5_000_000_000;

#[derive(Debug, Clone)]
enum Ptkp {
    TK0,
    TK1,
    K0,
    TK2,
    TK3,
    K1,
    K2,
    K3
}

// Map PTKP string to PTKP enum
impl FromStr for Ptkp {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "tk0" => Ok(Ptkp::TK0),
            "tk1" => Ok(Ptkp::TK1),
            "k0" => Ok(Ptkp::K0),
            "tk2" => Ok(Ptkp::TK2),
            "tk3" => Ok(Ptkp::TK3),
            "k1" => Ok(Ptkp::K1),
            "k2" => Ok(Ptkp::K2),
            "k3" => Ok(Ptkp::K3),
            _ => Ok(Ptkp::TK0), 
        }
    }
}

fn get_ptkp_value(ptkp: Ptkp) -> u32 {
    match ptkp {
        Ptkp::TK0 => 54_000_000,
        Ptkp::TK1 | Ptkp::K0 => 58_500_000,
        Ptkp::TK2 | Ptkp::K1 => 63_000_000,
        Ptkp::TK3 | Ptkp::K2 => 67_500_000,
        Ptkp::K3 => 72_000_000
    }
}


#[derive(Debug)]
struct TaxBracket {
    income_up_to: u64,
    percentage: f64, 
}

static TER_A_BRACKET: &[TaxBracket] = &[
        TaxBracket { income_up_to: 5_400_000, percentage: 0.00 },
        TaxBracket { income_up_to: 5_650_000, percentage: 0.0025 },
        TaxBracket { income_up_to: 5_950_000, percentage: 0.005 },
        TaxBracket { income_up_to: 6_300_000, percentage: 0.0075 },
        TaxBracket { income_up_to: 6_750_000, percentage: 0.01 },
        TaxBracket { income_up_to: 7_500_000, percentage: 0.0125 },
        TaxBracket { income_up_to: 8_550_000, percentage: 0.015 },
        TaxBracket { income_up_to: 9_650_000, percentage: 0.0175 },
        TaxBracket { income_up_to: 10_050_000, percentage: 0.02 },
        TaxBracket { income_up_to: 10_350_000, percentage: 0.0225 },
        TaxBracket { income_up_to: 10_700_000, percentage: 0.025 },
        TaxBracket { income_up_to: 11_050_000, percentage: 0.03 },
        TaxBracket { income_up_to: 11_600_000, percentage: 0.035 },
        TaxBracket { income_up_to: 12_500_000, percentage: 0.04 },
        TaxBracket { income_up_to: 13_750_000, percentage: 0.05 },
        TaxBracket { income_up_to: 15_100_000, percentage: 0.06 },
        TaxBracket { income_up_to: 16_950_000, percentage: 0.07 },
        TaxBracket { income_up_to: 19_750_000, percentage: 0.08 },
        TaxBracket { income_up_to: 24_150_000, percentage: 0.09 },
        TaxBracket { income_up_to: 26_450_000, percentage: 0.10 },
        TaxBracket { income_up_to: 28_000_000, percentage: 0.11 },
        TaxBracket { income_up_to: 30_050_000, percentage: 0.12 },
        TaxBracket { income_up_to: 32_400_000, percentage: 0.13 },
        TaxBracket { income_up_to: 35_400_000, percentage: 0.14 },
        TaxBracket { income_up_to: 39_100_000, percentage: 0.15 },
        TaxBracket { income_up_to: 43_850_000, percentage: 0.16 },
        TaxBracket { income_up_to: 47_800_000, percentage: 0.17 },
        TaxBracket { income_up_to: 51_400_000, percentage: 0.18 },
        TaxBracket { income_up_to: 56_300_000, percentage: 0.19 },
        TaxBracket { income_up_to: 62_200_000, percentage: 0.20 },
        TaxBracket { income_up_to: 68_600_000, percentage: 0.21 },
        TaxBracket { income_up_to: 77_500_000, percentage: 0.22 },
        TaxBracket { income_up_to: 89_000_000, percentage: 0.23 },
        TaxBracket { income_up_to: 103_000_000, percentage: 0.24 },
        TaxBracket { income_up_to: 125_000_000, percentage: 0.25 },
        TaxBracket { income_up_to: 157_000_000, percentage: 0.26 },
        TaxBracket { income_up_to: 206_000_000, percentage: 0.27 },
        TaxBracket { income_up_to: 337_000_000, percentage: 0.28 },
        TaxBracket { income_up_to: 454_000_000, percentage: 0.29 },
        TaxBracket { income_up_to: 550_000_000, percentage: 0.30 },
        TaxBracket { income_up_to: 695_000_000, percentage: 0.31 },
        TaxBracket { income_up_to: 910_000_000, percentage: 0.32 },
        TaxBracket { income_up_to: 1_400_000_000, percentage: 0.33 },
        TaxBracket { income_up_to: u64::MAX, percentage: 0.34 }, 
];

static TER_B_BRACKET: &[TaxBracket] = &[
        TaxBracket { income_up_to: 6_200_000, percentage: 0.00 },
        TaxBracket { income_up_to: 6_500_000, percentage: 0.0025 },
        TaxBracket { income_up_to: 6_850_000, percentage: 0.005 },
        TaxBracket { income_up_to: 7_300_000, percentage: 0.0075 },
        TaxBracket { income_up_to: 9_200_000, percentage: 0.01 },
        TaxBracket { income_up_to: 10_750_000, percentage: 0.015 },
        TaxBracket { income_up_to: 11_250_000, percentage: 0.02 },
        TaxBracket { income_up_to: 11_600_000, percentage: 0.025 },
        TaxBracket { income_up_to: 12_600_000, percentage: 0.03 },
        TaxBracket { income_up_to: 13_600_000, percentage: 0.04 },
        TaxBracket { income_up_to: 14_950_000, percentage: 0.05 },
        TaxBracket { income_up_to: 16_400_000, percentage: 0.06 },
        TaxBracket { income_up_to: 18_450_000, percentage: 0.07 },
        TaxBracket { income_up_to: 21_850_000, percentage: 0.08 },
        TaxBracket { income_up_to: 26_000_000, percentage: 0.09 },
        TaxBracket { income_up_to: 27_700_000, percentage: 0.10 },
        TaxBracket { income_up_to: 29_350_000, percentage: 0.11 },
        TaxBracket { income_up_to: 31_450_000, percentage: 0.12 },
        TaxBracket { income_up_to: 33_950_000, percentage: 0.13 },
        TaxBracket { income_up_to: 37_100_000, percentage: 0.14 },
        TaxBracket { income_up_to: 41_100_000, percentage: 0.15 },
        TaxBracket { income_up_to: 45_800_000, percentage: 0.16 },
        TaxBracket { income_up_to: 49_500_000, percentage: 0.17 },
        TaxBracket { income_up_to: 53_800_000, percentage: 0.18 },
        TaxBracket { income_up_to: 58_500_000, percentage: 0.19 },
        TaxBracket { income_up_to: 64_000_000, percentage: 0.20 },
        TaxBracket { income_up_to: 71_000_000, percentage: 0.21 },
        TaxBracket { income_up_to: 80_000_000, percentage: 0.22 },
        TaxBracket { income_up_to: 93_000_000, percentage: 0.23 },
        TaxBracket { income_up_to: 109_000_000, percentage: 0.24 },
        TaxBracket { income_up_to: 129_000_000, percentage: 0.25 },
        TaxBracket { income_up_to: 163_000_000, percentage: 0.26 },
        TaxBracket { income_up_to: 211_000_000, percentage: 0.27 },
        TaxBracket { income_up_to: 374_000_000, percentage: 0.28 },
        TaxBracket { income_up_to: 459_000_000, percentage: 0.29 },
        TaxBracket { income_up_to: 555_000_000, percentage: 0.30 },
        TaxBracket { income_up_to: 704_000_000, percentage: 0.31 },
        TaxBracket { income_up_to: 957_000_000, percentage: 0.32 },
        TaxBracket { income_up_to: 1_405_000_000, percentage: 0.33 },
        TaxBracket { income_up_to: u64::MAX, percentage: 0.34 }, 
];

static TER_C_BRACKET: &[TaxBracket] = &[
        TaxBracket { income_up_to: 6_600_000, percentage: 0.00 },
        TaxBracket { income_up_to: 6_950_000, percentage: 0.0025 },
        TaxBracket { income_up_to: 7_350_000, percentage: 0.005 },
        TaxBracket { income_up_to: 7_800_000, percentage: 0.0075 },
        TaxBracket { income_up_to: 8_850_000, percentage: 0.01 },
        TaxBracket { income_up_to: 9_800_000, percentage: 0.0125 },
        TaxBracket { income_up_to: 10_950_000, percentage: 0.015 },
        TaxBracket { income_up_to: 11_200_000, percentage: 0.0175 },
        TaxBracket { income_up_to: 12_050_000, percentage: 0.02 },
        TaxBracket { income_up_to: 12_950_000, percentage: 0.03 },
        TaxBracket { income_up_to: 14_150_000, percentage: 0.04 },
        TaxBracket { income_up_to: 15_550_000, percentage: 0.05 },
        TaxBracket { income_up_to: 17_050_000, percentage: 0.06 },
        TaxBracket { income_up_to: 19_500_000, percentage: 0.07 },
        TaxBracket { income_up_to: 22_700_000, percentage: 0.08 },
        TaxBracket { income_up_to: 26_600_000, percentage: 0.09 },
        TaxBracket { income_up_to: 28_100_000, percentage: 0.10 },
        TaxBracket { income_up_to: 30_100_000, percentage: 0.11 },
        TaxBracket { income_up_to: 32_600_000, percentage: 0.12 },
        TaxBracket { income_up_to: 35_400_000, percentage: 0.13 },
        TaxBracket { income_up_to: 38_900_000, percentage: 0.14 },
        TaxBracket { income_up_to: 43_000_000, percentage: 0.15 },
        TaxBracket { income_up_to: 47_400_000, percentage: 0.16 },
        TaxBracket { income_up_to: 51_200_000, percentage: 0.17 },
        TaxBracket { income_up_to: 55_800_000, percentage: 0.18 },
        TaxBracket { income_up_to: 60_400_000, percentage: 0.19 },
        TaxBracket { income_up_to: 66_700_000, percentage: 0.20 },
        TaxBracket { income_up_to: 74_500_000, percentage: 0.21 },
        TaxBracket { income_up_to: 83_200_000, percentage: 0.22 },
        TaxBracket { income_up_to: 95_000_000, percentage: 0.23 },
        TaxBracket { income_up_to: 110_000_000, percentage: 0.24 },
        TaxBracket { income_up_to: 134_000_000, percentage: 0.25 },
        TaxBracket { income_up_to: 169_000_000, percentage: 0.26 },
        TaxBracket { income_up_to: 221_000_000, percentage: 0.27 },
        TaxBracket { income_up_to: 390_000_000, percentage: 0.28 },
        TaxBracket { income_up_to: 463_000_000, percentage: 0.29 },
        TaxBracket { income_up_to: 561_000_000, percentage: 0.30 },
        TaxBracket { income_up_to: 709_000_000, percentage: 0.31 },
        TaxBracket { income_up_to: 965_000_000, percentage: 0.32 },
        TaxBracket { income_up_to: 1_419_000_000, percentage: 0.33 },
        TaxBracket { income_up_to: u64::MAX, percentage: 0.34 }, 
];


fn get_ptkp_ter(ptkp: Ptkp) -> &'static [TaxBracket] {
    match ptkp {
        Ptkp::TK0 | Ptkp::TK1 | Ptkp::K0 => TER_A_BRACKET,
        Ptkp::TK2 | Ptkp::TK3 | Ptkp::K2 | Ptkp::K1 => TER_B_BRACKET,
        Ptkp::K3 => TER_C_BRACKET
    }
}

fn get_ptkp_ter_percentage(
    gross_income: u64, 
    ter: &'static [TaxBracket]
) -> Option<f64> {
    ter
        .iter()
        .find(|ter| gross_income < ter.income_up_to)
        .map(|ter| ter.percentage)
}

// Input helpers
fn get_gross_income() -> u64 {
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

fn get_ptkp_tier() -> Ptkp {
    print!("PTKP tier: ");
    io::stdout().flush().expect("Failed to flush stdout");
    
    let mut ptkp_tier_input = String::new();
    io::stdin() 
        .read_line(&mut ptkp_tier_input)
        .expect("Failed to read line");
        
    let ptkp_tier: Ptkp = ptkp_tier_input.trim().parse().unwrap();
    
    return ptkp_tier;
}


fn main() {
    // Welcome banner
    let banner = r#"
+-------------------------------------------------------+
| Welcome to the tax calculator for period Jan-Nov      |
+-------------------------------------------------------+
| Version 1.0.0                                         |
| Built by Rizky Zhang                                  |
+-------------------------------------------------------+
"#;
    println!("{}", banner);

    // Gross income input    
    let gross_income = get_gross_income();
    
    // PTKP tier input
    let ptkp_tier = get_ptkp_tier();
    
    // Calculate tax
    let ptkp_ter = get_ptkp_ter(ptkp_tier);
    let ptkp_ter_percentage = get_ptkp_ter_percentage(gross_income, ptkp_ter).unwrap_or(0.0);
 
    let tax =  ptkp_ter_percentage * gross_income as f64;
    let gross_income_with_thr = gross_income * 2;
    let ptkp_ter_percentage_with_thr = get_ptkp_ter_percentage(gross_income_with_thr, ptkp_ter).unwrap_or(0.0);
    let tax_with_thr = ptkp_ter_percentage_with_thr * gross_income_with_thr as f64;
    
    println!("PTKP value: {}", ptkp_value);
    println!("Monthly TER percentage (exclude Mar): {:.1}%", ptkp_ter_percentage * 100.0);
    println!("Monthly tax Jan-Nov (exclude Mar): {:.2}", tax);
    println!("Mar TER percentage: {:.1}%", ptkp_ter_percentage_with_thr * 100.0);
    println!("Mar tax with THR {:.2}", tax_with_thr);
}
