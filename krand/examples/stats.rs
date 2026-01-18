use std::env;

use krand::Krand;

fn main() {
    // Parse args: N and seed (optional)
    let args: Vec<String> = env::args().collect();
    let n: usize = args
        .get(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(1_000_000);
    let seed: u64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(123);

    let mut rng = Krand::new(seed);

    let mut sum = 0.0_f64;
    let mut sum_sq = 0.0_f64;
    for _ in 0..n {
        let x = rng.next_f32_0_1() as f64;
        sum += x;
        sum_sq += x * x;
    }

    let n_f = n as f64;
    let mean = sum / n_f;
    // population variance
    let var = (sum_sq / n_f) - mean * mean;

    println!("N={}, seed={}", n, seed);
    println!("mean ≈ {:.6}", mean);
    println!("var  ≈ {:.6}", var);
    println!("theory: mean=0.5, var=1/12≈0.083333");
}
