//! # sys (ToyOS)
//!
//! ToyOS-specific structs and functions. Will be imported as `sys` on ToyOS.
#![expect(unsafe_code)]

use std::io::{self, BufRead};

use crate::Error;
pub use crate::xdg::*;

const SYS_SCREEN_SIZE: u64 = 7;

fn syscall(num: u64) -> u64 {
    let ret: u64;
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rdi") num,
            in("rsi") 0u64,
            in("rdx") 0u64,
            in("r8") 0u64,
            in("r9") 0u64,
            lateout("rax") ret,
            out("rcx") _,
            out("r11") _,
        );
    }
    ret
}

/// Terminal mode placeholder. ToyOS does not have termios.
#[derive(Clone, Copy)]
pub struct TermMode;

/// Return the current window size as (rows, columns) via the screen_size syscall.
pub fn get_window_size() -> Result<(usize, usize), Error> {
    let raw = syscall(SYS_SCREEN_SIZE);
    let rows = (raw >> 32) as usize;
    let cols = (raw & 0xFFFF_FFFF) as usize;
    if rows == 0 || cols == 0 {
        Err(Error::InvalidWindowSize)
    } else {
        Ok((rows, cols))
    }
}

/// No-op: ToyOS does not support signals.
pub fn register_winsize_change_signal_handler() -> io::Result<()> { Ok(()) }

/// Always returns false: ToyOS does not support window resize signals.
pub fn has_window_size_changed() -> bool { false }

/// No-op: ToyOS does not have terminal modes.
pub fn set_term_mode(_term: &TermMode) -> io::Result<()> { Ok(()) }

/// No-op: ToyOS kernel already provides raw-style input.
pub fn enable_raw_mode() -> io::Result<TermMode> { Ok(TermMode) }

/// Construct and lock a new handle to the standard input.
pub fn stdin() -> io::Result<impl BufRead> { Ok(io::stdin().lock()) }

/// Convert a filename string to a PathBuf.
pub fn path(filename: &str) -> std::path::PathBuf { std::path::PathBuf::from(filename) }
