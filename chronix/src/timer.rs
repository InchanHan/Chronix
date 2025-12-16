use std::sync::OnceLock;
use std::time::Instant;

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
