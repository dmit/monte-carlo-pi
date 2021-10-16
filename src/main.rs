use std::{env, time::Instant};

use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256StarStar;
use rayon::prelude::*;

type Precision = f32;

fn main() {
    let total_iterations = env::args()
        .nth(1)
        .expect("number of iterations not specified")
        .parse::<u64>()
        .expect("invalid number of iterations");
    let parallel =
        env::args().nth(2).map(|n| n.parse::<u64>().unwrap_or(1000).min(total_iterations));

    println!(
        "Parallel: {}\nTotal: {}",
        if let Some(chunk_size) = parallel {
            format!("yes ({} per chunk)", chunk_size)
        } else {
            "no".to_string()
        },
        total_iterations
    );

    let start = Instant::now();
    let res = if let Some(chunk_size) = parallel {
        run_par(total_iterations, chunk_size)
    } else {
        run(total_iterations)
    };
    let duration = start.elapsed();

    let pi = (res as f64 / total_iterations as f64) * 4.0;
    let iterations_per_s = total_iterations as f64 / duration.as_secs_f64();
    println!(
        "Inside: {}\nπ: {}\nTime elapsed: {}.{:0<3}s\nIterations/s: {:.3}M",
        res,
        pi,
        duration.as_secs(),
        duration.subsec_millis(),
        iterations_per_s / 1_000_000.0,
    );
}

fn run(n: u64) -> u64 {
    let mut rng = Xoshiro256StarStar::seed_from_u64(rand::thread_rng().gen());

    let mut cnt = 0u64;
    for _ in 0..n {
        let x: Precision = rng.gen();
        let y: Precision = rng.gen();
        if (x * x + y * y).sqrt() <= 1.0 {
            cnt += 1;
        }
    }
    cnt
}

fn run_par(n: u64, chunk_size: u64) -> u64 {
    use std::cell::RefCell;
    thread_local!(static RNG: RefCell<Option<Xoshiro256StarStar>> = RefCell::new(None));

    let (chunk, num_chunks) =
        if chunk_size < n { (0..chunk_size, (n / chunk_size) as usize) } else { (0..n, 1) };

    rayon::iter::repeatn(chunk, num_chunks)
        .take(num_chunks)
        .map(|chunk| {
            RNG.with(|cell| {
                let mut local_store = cell.borrow_mut();
                if local_store.is_none() {
                    let rng = Xoshiro256StarStar::seed_from_u64(rand::thread_rng().gen());
                    *local_store = Some(rng);
                }

                let rng = local_store.as_mut().unwrap();

                chunk.fold(0u64, |acc, _| {
                    let x: Precision = rng.gen();
                    let y: Precision = rng.gen();
                    if (x * x + y * y).sqrt() <= 1.0 { acc + 1 } else { acc }
                })
            })
        })
        .sum()
}
