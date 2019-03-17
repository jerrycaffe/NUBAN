use std::process;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Ok(true) = is_valid(&args) {
        println!("{}", true);
        process::exit(0);
    } else {
        println!("{}", false);
        process::exit(1);
    }
}

fn is_valid(args: &[String]) -> Result<bool, bool> {
    let bank_code = args[1].clone();
    let mut account_number = args[2].clone();
    let check_digit = account_number.pop().unwrap();

    let mut number = String::new();
    number.push_str(&bank_code);
    number.push_str(&account_number);

    let mut check_sum = 0;
    for num in number.chars() {
        check_sum += num.to_digit(10).expect("Invalid digit!");
    }

    let mut r_check_digit = 10 - (check_sum % 10);
    if r_check_digit == 10 { r_check_digit = 0; }

    if r_check_digit == check_digit.to_digit(10).unwrap() {
        Ok(true)
    } else {
        Err(false)
    }
}