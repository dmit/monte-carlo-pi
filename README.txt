A toy Monte Carlo estimation of π, including automatic parallelization.

Build:
  $ cargo build --release

Run:
  $ # Single-threaded
  $ ./target/release/monte-carlo-pi 10000000

  $ # Parallel
  $ ./target/release/monte-carlo-pi 10000000 par
