#![feature(duration_float)]

use rand::Rng;
use rayon::prelude::*;
use std::env;
use std::time::Instant;
use xoshiro::Xoshiro256Plus;

type Precision = f32;

fn main() {
    let n = env::args()
        .nth(1)
        .expect("number of iterations not specified")
        .parse::<u64>()
        .expect("invalid number of iterations");
    let parallel = env::args().nth(2).is_some();

    let start = Instant::now();
    let res = if parallel { run_par(n) } else { run(n) };
    let duration = start.elapsed();

    let pi = (res as f64 / n as f64) * 4.0;
    println!(
        "Total: {}\nInside: {}\nπ: {}\nTime elapsed: {}.{:0<3}s\nIterations/s: {:.3}M",
        n,
        res,
        pi,
        duration.as_secs(),
        duration.subsec_millis(),
        n as f64 / duration.as_float_secs() / 1_000_000.0,
    );
}

fn run(n: u64) -> u64 {
    let mut rng = Xoshiro256Plus::from_seed_u64(rand::thread_rng().gen());

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

fn run_par(n: u64) -> u64 {
    use std::cell::RefCell;
    thread_local!(static RNG: RefCell<Option<Xoshiro256Plus>> = RefCell::new(None));

    (0..n)
        .into_par_iter()
        .map(|_| {
            RNG.with(|cell| {
                let mut local_store = cell.borrow_mut();
                if local_store.is_none() {
                    let rng = Xoshiro256Plus::from_seed_u64(rand::thread_rng().gen());
                    *local_store = Some(rng);
                }

                let rng = local_store.as_mut().unwrap();
                let x: Precision = rng.gen();
                let y: Precision = rng.gen();
                if (x * x + y * y).sqrt() <= 1.0 {
                    1
                } else {
                    0
                }
            })
        })
        .sum()
}
