// mod nilakantha;
mod chudnovsky;
use std::io::{self, Write};
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Invalid arguments.\nUsage: pi <digits>. Prints to stdout the number of digits of pi specified using the Chudnovsky algorithm.");
        return;
    }

    let digits_parse = args[1].trim().parse::<u32>();

    let Ok(digits) = digits_parse else {
        println!("Error: Please enter a valid number within the unsigned 32-bit integer limit.");
        return;
    };
    let (final_digits, digit_count) = chudnovsky::calculate(digits);
    filesave(final_digits, digit_count);
}

fn filesave(final_digits: String, digit_count: usize) {
    print!("Would you like to save digits to a file (y/n)?: ");
    io::stdout().flush().unwrap();

    let mut response = String::new();
    io::stdin().read_line(&mut response).expect("Failed to read line.");

    match response.trim() {
        "y" => {
            let filepath = format!("pi_digits_to_{digit_count}.txt");
            fs::write(&filepath, final_digits).expect("Failed to write digits, sorry.");
            println!("Saved at ./{filepath}.");
        },
        "n" => {}
        _ => {
            println!("Sorry, please enter either the character 'y' for yes or 'n' for no.");
            filesave(final_digits, digit_count);
        }
    }

}