# Chronix

Chronix is a small and simple micro-benchmark helper for Rust.

It is designed for quick local performance checks, not for statistically rigorous benchmarking.

Workspace crates:

- chronix        : runtime (timers, runner, output)
- chronix-derive : attribute macro (#[chronixer])
- derive-test    : example binary

---

## Features

- Attribute macro based benchmarking
- Warmup + repeated measurements
- Aggregation modes: min, median, p95
- Two timing backends
  - Instant (portable, default)
  - RDTSCP cycle counter (x86_64 only, optional)
- Optional CPU pinning for stability

---

## Quick Start

```toml
[dependencies]
chronix = { path = "../chronix" }
chronix-derive = { path = "../chronix-derive" }
```

```rust
use chronix_derive::chronixer;

#[chronixer(
    warmup = 3,
    reps = 20,
    agg = "median",
    pin = true,
    accesses = 1,
    cpu_ghz = 3.5
)]
fn my_fn(x: u64) -> u64 {
    x.wrapping_mul(1234567)
}

fn main() {
    let _ = my_fn(42);
}
```

When the function is called, Chronix runs warmup and benchmark iterations
and prints timing statistics to stdout.

Running the example
From the workspace root:

### Enabling rdtscp timer(only for x86_64)

```
cargo run --features rdtscp
```
### Enable CPU pinning

```
cargo run --features affinity
```

---

## chronixer attribute options
- warmup : number of warmup iterations
- reps : number of measured repetitions
- agg : aggregation method ("min", "median", "p95")
- pin : pin thread to CPU core 0 (requires affinity feature)
- accesses : logical operation count for per-access metrics
- cpu_ghz : CPU frequency used for cycle conversion

> Note:
accesses is user-defined.
If your function performs N operations internally, set accesses = N
so ns_per_access becomes meaningful.

---

## Philosophy
Chronix optimizes for:
- minimal setup

- fast iteration

- low mental overhead

It intentionally avoids complex statistics and heavy configuration.
If you need strict benchmarking, use Criterion instead.
