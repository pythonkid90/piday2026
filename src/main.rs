// mod nilakantha;
mod chudnovsky;
// mod why;
use std::io::{self, Write};
use std::{env, fs};
use std::time::Instant;

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
    let now = Instant::now();
    let (final_digits, digit_count) = chudnovsky::calculate(digits);
    println!("Finished in {}ms.", now.elapsed().as_millis());
    filesave(final_digits, digit_count);

    // let now = Instant::now();
    // println!("{}", why::pi((((70514 as f64 * 151931373056000.0_f64.log10()) + 1.0) * 10.0_f64.log2()).floor() as u32, 70514));
    // println!("{}", now.elapsed().as_millis());
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
