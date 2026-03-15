pub fn calculate(iterations: usize) {
    let mut pi_approx: f64 = 3.0;
    let mut lowest_denom_factor = 2;
    let mut numerator = 4;
    for _count in 1..iterations {
        let denom = lowest_denom_factor * (lowest_denom_factor + 1) * (lowest_denom_factor + 2);
        pi_approx += numerator as f64 / denom as f64;

        numerator *= -1;
        lowest_denom_factor += 2;
    }
    println!("{}", pi_approx);
}