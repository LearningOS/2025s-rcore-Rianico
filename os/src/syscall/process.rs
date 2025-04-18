//! Process management syscalls
use crate::task::{exit_current_and_run_next, get_sys_call_cnt, suspend_current_and_run_next};
use crate::timer::get_time_us;

const SYS_TRACE_READ: usize = 0;
const SYS_TRACE_WRITE: usize = 1;
const SYS_TRACE_GET_CNT: usize = 2;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

// TODO: implement the syscall
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    trace!("kernel: sys_trace");
    match trace_request {
        SYS_TRACE_READ => unsafe { (id as *const u8).read() as isize },
        SYS_TRACE_WRITE => {
            unsafe { core::ptr::write(id as *mut u8, data as u8) }
            0
        }
        SYS_TRACE_GET_CNT => get_sys_call_cnt(id),
        _ => {
            unreachable!()
        }
    }
}
