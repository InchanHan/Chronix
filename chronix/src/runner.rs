use crate::timer::Timer;

#[derive(Clone, Debug)]
pub enum Aggregation {
    Min,
    Median,
    P95,
}

#[derive(Clone, Debug)]
pub struct BenchConfig {
    pub warmup: u32,
    pub reps: u32,
    pub aggregation: Aggregation,
    pub pin_core0: bool,
}

impl BenchConfig {
    pub fn new(warmup: u32, reps: u32, aggregation: Aggregation, pin_core0: bool) -> Self {
        Self {
            warmup,
            reps,
            aggregation,
            pin_core0,
        }
    }
}

impl Default for BenchConfig {
    fn default() -> Self {
        Self {
            warmup: 1,
            reps: 10,
            aggregation: Aggregation::Min,
            pin_core0: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct BenchStat {
    pub elapsed_ns: f64,
    pub accesses: u32,
    pub ns_per_access: f64,
    pub cycles_per_access: Option<f64>,
    pub aggregation: Aggregation,
}

pub struct Runner<T: Timer> {
    pub timer: T,
    pub cfg: BenchConfig,
}

impl<T: Timer> Runner<T> {
    pub fn new(timer: T, cfg: BenchConfig) -> Self {
        Self { timer, cfg }
    }

    pub fn measure<F, R>(&mut self, mut f: F, accesses: u32) -> (BenchStat, R)
    where
        F: FnMut() -> R,
    {
        use crate::utils::{median_mut, percentile_mut};

        if self.cfg.pin_core0 {
            crate::prelude::pin_current_thread_to_core0();
        }

        // warm up
        for _ in 0..self.cfg.warmup {
            f();
        }

        // total ns for InstantTimer, or total cycles for RdtscpTimer
        let mut total: u64 = 0;

        // reps
        let mut ns: Vec<f64> = Vec::with_capacity(self.cfg.reps as usize);
        for _ in 0..self.cfg.reps {
            self.timer.barrier_before();
            let start = self.timer.now();
            f();
            let end = self.timer.now();

            total += end - start;
            let dt_ns = self.timer.to_ns(end - start);
            ns.push(dt_ns);
        }

        let agg_ns = match self.cfg.aggregation {
            Aggregation::Min => ns.iter().cloned().fold(f64::INFINITY, f64::min),
            Aggregation::Median => {
                let mut v = ns.clone();
                median_mut(&mut v)
            }
            Aggregation::P95 => {
                let mut v = ns.clone();
                percentile_mut(&mut v, 95.0)
            }
        };

        let result = Some(f());
        let npa = agg_ns / accesses as f64;
        let cpa = if self.timer.has_cycles() {
            Some(total as f64 / accesses as f64)
        } else {
            None
        };

        (
            BenchStat {
                elapsed_ns: agg_ns,
                accesses,
                ns_per_access: npa,
                cycles_per_access: cpa,
                aggregation: self.cfg.aggregation.clone(),
            },
            result.unwrap(),
        )
    }
}
