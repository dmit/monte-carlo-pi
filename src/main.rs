#![feature(duration_float)]

extern crate pcg_rand;
extern crate rand;
extern crate rayon;

use pcg_rand::seeds::PcgSeeder;
use pcg_rand::Pcg32;
use rand::{Rng, SeedableRng};
use rayon::prelude::*;
use std::env;
use std::time::Instant;

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
    let seeder = {
        let seed: u64 = rand::thread_rng().gen();
        let seq: u64 = rand::thread_rng().gen();
        PcgSeeder::seed_with_stream(seed, seq)
    };
    let mut rng: Pcg32 = SeedableRng::from_seed(seeder);

    let mut cnt = 0u64;
    for _ in 0..n {
        let x: f32 = rng.gen();
        let y: f32 = rng.gen();
        if (x * x + y * y).sqrt() <= 1.0 {
            cnt += 1;
        }
    }
    cnt
}

fn run_par(n: u64) -> u64 {
    use std::cell::RefCell;
    thread_local!(static RNG: RefCell<Option<Pcg32>> = RefCell::new(None));

    (0..n)
        .into_par_iter()
        .map(|_| {
            RNG.with(|cell| {
                let mut local_store = cell.borrow_mut();
                if local_store.is_none() {
                    let seeder = {
                        let seed: u64 = rand::thread_rng().gen();
                        let seq: u64 = rand::thread_rng().gen();
                        PcgSeeder::seed_with_stream(seed, seq)
                    };
                    let rng: Pcg32 = SeedableRng::from_seed(seeder);
                    *local_store = Some(rng);
                }

                let rng = local_store.as_mut().unwrap();
                let x: f32 = rng.gen();
                let y: f32 = rng.gen();
                if (x * x + y * y).sqrt() <= 1.0 {
                    1
                } else {
                    0
                }
            })
        })
        .sum()
}
