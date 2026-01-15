use crate::{runner::{BenchConfig, BenchStat}, timer::Duration};

pub trait Sink {
    fn report(&self, _label: &str, _bench_stat: &BenchStat, _bench_config: &BenchConfig) {}
}

pub struct Stdout;

impl Sink for Stdout {
    fn report(&self, label: &str, bench_stat: &BenchStat, bench_config: &BenchConfig) {
        let elapsed_duration = Duration::from_ns(bench_stat.elapsed_ns);
        let per_access_duration = Duration::from_ns(bench_stat.ns_per_access);
        match bench_stat.cycles_per_access {
            None => {
                print!(
                    "[Chronix] fn {}\n\r    aggregation     : {:#?}\n\r    warmup | reps   : {} | {}\n\r    elapsed_ns      : {:<12.3}{}\n\r    ns_per_access   : {:<12.3}{}\n\r    accesses        : {:<12}\n\r\n",
                    label,
                    bench_stat.aggregation,
                    bench_config.warmup,
                    bench_config.reps,
                    elapsed_duration.time(),
                    elapsed_duration.unit().suffix(),
                    per_access_duration.time(),
                    per_access_duration.unit().suffix(),
                    bench_stat.accesses
                )
            }
            Some(cpa) => {
                print!(
                    "[Chronix] fn {}\n\r    aggregation       : {:#?}\n\r    warmup | reps     : {} | {}\n\r\n    elapsed_ns        : {:<12.3}{}\n\r    ns_per_access     : {:<12.3}{}\n\r    cycles_per_access : {:<12.3}cycles\r\n    accesses          : {:<12}\n\r",
                    label,
                    bench_stat.aggregation,
                    bench_config.warmup,
                    bench_config.reps,
                    elapsed_duration.time(),
                    elapsed_duration.unit().suffix(),
                    per_access_duration.time(),
                    per_access_duration.unit().suffix(),
                    cpa,
                    bench_stat.accesses
                )
            }
        }
    }
}
