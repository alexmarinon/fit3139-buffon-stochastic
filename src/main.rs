use rand::Rng;
use std::f64::consts::PI;

fn main() {
    let num_simulations: u64 = 1_000_000;
    let needle_length = 1.0;
    let line_spacing = 2.0;

    let mut rng = rand::thread_rng();
    let mut crossings = 0;

    for _ in 0..num_simulations {
        let y_center: f64 = rng.gen_range(0.0..line_spacing / 2.0);
        let theta: f64 = rng.gen_range(0.0..PI);

        let y_edge = needle_length / 2.0 * theta.sin();

        if y_center <= y_edge {
            crossings += 1;
        }
    }

    let pi_estimate = (2.0 * needle_length * num_simulations as f64) / (crossings as f64 * line_spacing);

    println!("Estimated value of Pi: {}", pi_estimate);
}