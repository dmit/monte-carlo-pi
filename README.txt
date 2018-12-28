A toy Monte Carlo estimation of π, including automatic parallelization.

Build:
  $ cargo build --release

Run:
  $ # Single-threaded
  $
  $ cargo run --release <number of iterations>

  $ # Parallel
  $ #
  $ # This mode can perform multiple iterations per unit of work (chunk). In
  $ # order to optimize overall efficiency the chunk size should be big enough
  $ # so that time to process a single chunk significantly outweighs the
  $ # parallelization overhead.
  $
  $ cargo run --release <number of iterations> <chunk size>
