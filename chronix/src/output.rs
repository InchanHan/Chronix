use crate::runner::{BenchConfig, BenchStat};

pub trait Sink {
    fn report(&self, _label: &str, _bench_stat: &BenchStat, _bench_config: &BenchConfig) {}
}

pub struct Stdout;

impl Sink for Stdout {
    fn report(&self, label: &str, bench_stat: &BenchStat, bench_config: &BenchConfig) {
        match bench_stat.cycles_per_access {
            None => {
                print!(
                    "[Chronix] fn {}\n\r    aggregation     : {:#?}\n\r    warmup | reps   : {} | {}\n\r    elapsed_ns      : {:<12.3}ns\n\r    ns_per_access   : {:<12.3}ns\n\r    accesses        : {:<12}\n\r\n",
                    label,
                    bench_stat.aggregation,
                    bench_config.warmup,
                    bench_config.reps,
                    bench_stat.elapsed_ns,
                    bench_stat.ns_per_access,
                    bench_stat.accesses
                )
            }
            Some(cpa) => {
                print!(
                    "[Chronix] fn {}\n\r    aggregation       : {:#?}\n\r    warmup | reps     : {} | {}\n\r\n    elapsed_ns        : {:<12.3}ns\n\r    ns_per_access     : {:<12.3}ns\n\r    cycles_per_access : {:<12.3}cycles\r\n    accesses          : {:<12}\n\r",
                    label,
                    bench_stat.aggregation,
                    bench_config.warmup,
                    bench_config.reps,
                    bench_stat.elapsed_ns,
                    bench_stat.ns_per_access,
                    cpa,
                    bench_stat.accesses
                )
            }
        }
    }
}
