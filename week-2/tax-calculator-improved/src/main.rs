mod constants;
mod helpers;
use helpers::ptkp_value;
use helpers::ptkp_ter;
use helpers::ptkp_ter_percentage;
use helpers::inputs::gross_income_input;
use helpers::inputs::ptkp_input;

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
    let gross_income = gross_income_input::get();
    
    // PTKP tier input
    let ptkp_tier = ptkp_input::get();
    let ptkp_value = ptkp_value::get(ptkp_tier.clone());
    
    // Calculate tax
    let ptkp_ter = ptkp_ter::get(ptkp_tier);
    let ptkp_ter_percentage = ptkp_ter_percentage::get(gross_income, ptkp_ter).unwrap_or(0.0);
 
    let tax =  ptkp_ter_percentage * gross_income as f64;
    let gross_income_with_thr = gross_income * 2;
    let ptkp_ter_percentage_with_thr = ptkp_ter_percentage::get(gross_income_with_thr, ptkp_ter).unwrap_or(0.0);
    let tax_with_thr = ptkp_ter_percentage_with_thr * gross_income_with_thr as f64;
    
    println!("PTKP value: {}", ptkp_value);
    println!("Monthly TER percentage (exclude Mar): {:.1}%", ptkp_ter_percentage * 100.0);
    println!("Monthly tax Jan-Nov (exclude Mar): {:.2}", tax);
    println!("Mar TER percentage: {:.1}%", ptkp_ter_percentage_with_thr * 100.0);
    println!("Mar tax with THR {:.2}", tax_with_thr);
}
