use rug::{Float, Integer, Complete, ops::Pow};

pub fn calculate(digits: u32) -> (String, usize) {

    let digits_per_iter = 151931373056000.0_f64.log10();
    // Digits + 1 in iterations as first iteration only has 13 digits so it needs anotther iteration if at bound. - 1 at end of calc line as that ghost digit needs to be subtractted from digit count.
    let iterations = ((digits + 1) as f64 / digits_per_iter).ceil() as u32; // Ceil to make sure iterations cover all digits.
    let precision = (((iterations as f64 * digits_per_iter) + 1.0) * 10.0_f64.log2()).floor() as u32; // Bits of precision (log base 2) for our float.

    let base = Float::with_val(precision, 640320); // Optimizing the 640320 power as well as factorials 
    let (base_cubed, mut running_base) = (base.clone().pow(3), base.pow(1.5_f64).clone());
    let (mut fact_k, mut fact_3k, mut fact_6k) = (Integer::from(1), Integer::from(1), Integer::from(1));

    println!("Uses {iterations} iterations of the Chudnovsky algorithm.");
    print!("0 digits calculated so far out of {digits}...");

    let mut pi_approx = Float::with_val(precision, 0.0);

    for k in 0..iterations {
        let numerator = Integer::from((-1_i64).pow(k as u32)) * fact_6k.clone() * ((545140134 * k as u64) + 13591409);
        let denominator = fact_3k.clone() * fact_k.clone().pow(3) * running_base.clone();

        running_base *= &base_cubed; // Starts at 64320 ^ 1.5, then to the ^4.5 tto avoid calculating everything each iter.
        // Adding the next steps to the factorial with the increase in k
        fact_k *= Integer::from(k + 1);
        fact_3k *= Integer::from(3*k + 1) * Integer::from(3*k + 2) * Integer::from(3*k + 3); 
        fact_6k *= Integer::from(6*k + 1) * Integer::from(6*k + 2) * Integer::from(6*k + 3) 
                * Integer::from(6*k + 4) * Integer::from(6*k + 5) * Integer::from(6*k + 6);
        
        pi_approx += numerator.clone() / denominator.clone();
        
        print!("\r{} digits calculated so far out of {digits}...", (digits_per_iter * (k + 1) as f64).floor() as u32 - 1);

    }

    pi_approx = 1.0 / (12.0 * pi_approx);
    let mut final_digits = pi_approx.to_string_radix(10, Some((digits_per_iter * iterations as f64).floor() as usize + 1));
    final_digits.pop();
    println!("Done!\n{final_digits}");

    (final_digits, ((digits_per_iter * iterations as f64).floor()) as usize - 1)
}