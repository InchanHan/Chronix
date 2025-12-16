pub fn page_pretouch<T>(buf: &[T]) {
    let step = (4096 / std::mem::size_of::<T>()).max(1);
    for i in (0..buf.len()).step_by(step) {
        unsafe {
            std::ptr::read_volatile(&buf[i]);
        }
    }
}

#[cfg(feature = "affinity")]
pub fn pin_current_thread_to_core0() {
    #[cfg(target_os = "linux")]
    unsafe {
        use libc::{CPU_SET, CPU_ZERO, cpu_set_t, sched_setaffinity};
        let mut set: cpu_set_t = std::mem::zeroed();
        CPU_ZERO(&mut set);
        CPU_SET(0, &mut set);
        let _ = sched_setaffinity(0, std::mem::size_of::<cpu_set_t>(), &set);
    }
    #[cfg(target_os = "windows")]
    unsafe {
        use windows_sys::Win32::System::Threading::{GetCurrentThread, SetThreadAffinityMask};
        let _ = SetThreadAffinityMask(GetCurrentThread(), 1);
    }
}

#[cfg(not(feature = "affinity"))]
pub fn pin_current_thread_to_core0() {}
