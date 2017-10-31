extern crate pcg_rand;
extern crate rand;
extern crate rayon;

use pcg_rand::Pcg32;
use rand::{Rng, SeedableRng};
use rayon::prelude::*;
use std::env;

fn main() {
    let n = env::args()
        .nth(1)
        .expect("number of iterations not specified")
        .parse::<u64>()
        .expect("invalid number of iterations");
    let parallel = env::args().nth(2).is_some();
    let res = if parallel { run_par(n) } else { run(n) };
    let pi = (res as f64 / n as f64) * 4.0;
    println!("Total: {}\nInside: {}\nπ: {}", n, res, pi);
}

fn run(n: u64) -> u64 {
    let seed: [u64; 2] = rand::thread_rng().gen();
    let mut rng: Pcg32 = SeedableRng::from_seed(seed);

    let mut cnt = 0u64;
    for _ in 0..n {
        let x = rng.next_f32();
        let y = rng.next_f32();
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
                    let seed: [u64; 2] = rand::thread_rng().gen();
                    let rng: Pcg32 = SeedableRng::from_seed(seed);
                    *local_store = Some(rng);
                }

                let rng = local_store.as_mut().unwrap();
                let x = rng.next_f32();
                let y = rng.next_f32();
                if (x * x + y * y).sqrt() <= 1.0 {
                    1
                } else {
                    0
                }
            })
        })
        .sum()
}
