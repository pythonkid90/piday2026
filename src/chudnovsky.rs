use rug::{Float, Integer, Complete, ops::Pow};

fn fctorl(n: u32) -> Integer {
    Integer::factorial(n).complete()
}

pub fn calculate(digits: u32) -> (String, usize) {
    let digits_per_iter = 151931373056000.0_f64.log10();
    // Digits + 1 in iterations as first iteration only has 13 digits so it needs anotther iteration if at bound. - 1 at end of calc line as that ghost digit needs to be subtractted from digit count.
    let iterations = ((digits + 1) as f64 / digits_per_iter).ceil() as u32; // Ceil to make sure iterations cover all digits.
    let precision = ((iterations as f64 * digits_per_iter) as f32 * 10.0_f32.log2()).ceil() as u32; // Bits of precision (log base 2) for our float.

    println!("Uses {iterations} iterations of the Chudnovsky algorithm.");
    print!("0 digits calculated so far out of {digits}...");

    let mut pi_approx = Float::with_val(precision, 0.0);
    for k in 0..iterations {
        let numerator = Integer::from((-1_i64).pow(k as u32)) * fctorl(6*k) * ((545140134 * k as u64) + 13591409);
        let denominator = fctorl(3*k) * fctorl(k).pow(3) * Float::with_val(precision, 640320).pow((3*k) as f64 + 1.5);
        pi_approx += numerator.clone() / denominator.clone();
        
        print!("\r{} digits calculated so far out of {digits}...", (digits_per_iter * (k + 1) as f64).floor() as u32 - 1);

    }

    let mut final_digits = format!("{:.1$}", 1.0 / (12.0 * pi_approx), (digits_per_iter * iterations as f64).floor() as usize + 1);
    final_digits.pop();
    println!("Done!\n{final_digits}");

    (final_digits, ((digits_per_iter * iterations as f64).floor()) as usize - 1)
}