use chronix_derive::chronixer;

#[chronixer(
    warmup = 12,
    reps = 100,
    agg = "median",
    pin = true,
    accesses = 1,
    cpu_ghz = 3.5
)]
fn a(b: i32) {
    let _v = b + 2;
}

#[chronixer(
    warmup = 12,
    reps = 100,
    agg = "median",
    pin = true,
    accesses = 1,
    cpu_ghz = 3.5
)]
fn calcul(num: u64) -> u64 {
    let mut c = num;
    for i in 0..100000 {
        c += i as u64;
    }
    c
}

fn main() {
    a(2);
    let _ = calcul(1423);
}
