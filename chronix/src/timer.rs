use std::sync::OnceLock;
use std::time::Instant;

pub struct Duration {
    value: f64,
    unit: TimeUnit,
}

impl Duration {
    pub fn from_ns(ns: f64) -> Self {
        let unit = TimeUnit::auto_from_ns(ns);
        Self {
            value: ns / unit.scale_ns(),
            unit,
        }
    }

    pub fn time(&self) -> f64 {
        self.value
    }

    pub fn unit(&self) -> TimeUnit {
        self.unit.clone()
    }
}

#[derive(Clone)]
pub enum TimeUnit {
    Ns,
    Us,
    Ms,
    S,
}

impl TimeUnit {
    pub fn suffix(&self) -> &'static str {
        match self {
            TimeUnit::Ns => "ns",
            TimeUnit::Us => "µs",
            TimeUnit::Ms => "ms",
            TimeUnit::S => "s",
        }
    }

    pub fn scale_ns(&self) -> f64 {
        match self {
            TimeUnit::Ns => 1.0,
            TimeUnit::Us => 1000.0,
            TimeUnit::Ms => 1_000_000.0,
            TimeUnit::S => 1_000_000_000.0,
        }
    }

    pub fn auto_from_ns(ns: f64) -> Self {
        if ns < 1_000.0 {
            TimeUnit::Ns
        } else if ns < 1_000_000.0 {
            TimeUnit::Us
        } else if ns < 1_000_000_000.0 {
            TimeUnit::Ms
        } else {
            TimeUnit::S
        }
    }
}

pub trait Timer {
    fn now(&self) -> u64;
    fn barrier_before(&self) {}
    fn to_ns(&self, tick: u64) -> f64;
    fn has_cycles(&self) -> bool {
        false
    }
}

pub struct InstantTimer;

impl Timer for InstantTimer {
    fn now(&self) -> u64 {
        static T0: OnceLock<Instant> = OnceLock::new();
        let t0 = T0.get_or_init(Instant::now);
        t0.elapsed().as_nanos() as u64
    }

    fn to_ns(&self, tick: u64) -> f64 {
        tick as f64
    }
}

#[cfg(all(target_arch = "x86_64", feature = "rdtscp"))]
pub struct RdtscpTimer {
    pub cpu_ghz: f64,
}

#[cfg(all(target_arch = "x86_64", feature = "rdtscp"))]
impl Timer for RdtscpTimer {
    fn barrier_before(&self) {
        unsafe {
            core::arch::asm!("lfence", options(nostack, preserves_flags));
        }
    }

    fn now(&self) -> u64 {
        let lo: u32;
        let hi: u32;
        unsafe {
            core::arch::asm!(
                "rdtscp",
                out("eax") lo, out("edx") hi, out("ecx") _,
            );
        }
        ((hi as u64) << 32) | lo as u64
    }

    fn to_ns(&self, tick: u64) -> f64 {
        tick as f64 / self.cpu_ghz
    }

    fn has_cycles(&self) -> bool {
        true
    }
}
