use std::time::Instant;

use argh::FromArgs;
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256StarStar;
use rayon::prelude::*;

type Precision = f32;

/// estimate Pi using the Monte Carlo method
#[derive(FromArgs)]
struct Opts {
    /// total number of iterations
    #[argh(option, short = 'n')]
    iterations: u64,

    /// run in parallel with specified number of iterations per work unit
    #[argh(option, short = 'p')]
    parallel_chunk_size: Option<u64>,
}

fn main() {
    let opts: Opts = argh::from_env();

    println!(
        "Parallel: {}\nTotal: {}",
        if let Some(chunk_size) = opts.parallel_chunk_size {
            format!("yes ({} per chunk)", chunk_size)
        } else {
            "no".to_string()
        },
        opts.iterations
    );

    let start = Instant::now();
    let res = if let Some(chunk_size) = opts.parallel_chunk_size {
        run_par(opts.iterations, chunk_size)
    } else {
        run(opts.iterations)
    };
    let duration = start.elapsed();

    let pi = (res as f64 / opts.iterations as f64) * 4.0;
    let iterations_per_s = opts.iterations as f64 / duration.as_secs_f64();
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
    let mut rng = Xoshiro256StarStar::seed_from_u64(rand::random());

    let mut cnt = 0u64;
    for _ in 0..n {
        let x: Precision = rng.random();
        let y: Precision = rng.random();
        if (x * x + y * y).sqrt() <= 1.0 {
            cnt += 1;
        }
    }
    cnt
}

fn run_par(n: u64, chunk_size: u64) -> u64 {
    use std::cell::RefCell;
    thread_local!(static RNG: RefCell<Option<Xoshiro256StarStar>> = const { RefCell::new(None) });

    let (chunk, num_chunks) =
        if chunk_size < n { (0..chunk_size, (n / chunk_size) as usize) } else { (0..n, 1) };

    rayon::iter::repeatn(chunk, num_chunks)
        .take(num_chunks)
        .map(|chunk| {
            RNG.with(|cell| {
                let mut local_store = cell.borrow_mut();
                if local_store.is_none() {
                    let rng = Xoshiro256StarStar::seed_from_u64(rand::random());
                    *local_store = Some(rng);
                }

                let rng = local_store.as_mut().unwrap();

                chunk.fold(0u64, |acc, _| {
                    let x: Precision = rng.random();
                    let y: Precision = rng.random();
                    if (x * x + y * y).sqrt() <= 1.0 { acc + 1 } else { acc }
                })
            })
        })
        .sum()
}
