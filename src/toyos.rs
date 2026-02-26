//! # sys (ToyOS)
//!
//! ToyOS-specific structs and functions. Will be imported as `sys` on ToyOS.

use std::io::{self, BufRead};

use crate::Error;
// ToyOS has no XDG directories; return empty to avoid probing nonexistent paths.
pub fn conf_dirs() -> Vec<String> { Vec::new() }
pub fn data_dirs() -> Vec<String> { Vec::new() }

/// Terminal mode: stores nothing, but used as a token for restore.
#[derive(Clone, Copy)]
pub struct TermMode;

/// Return the current window size as (rows, columns).
pub fn get_window_size() -> Result<(usize, usize), Error> {
    let (rows, cols) = toyos_abi::syscall::screen_size();
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

/// Restore canonical (line-buffered) stdin mode.
pub fn set_term_mode(_term: &TermMode) -> io::Result<()> {
    std::os::toyos::io::set_stdin_raw(false);
    Ok(())
}

/// Switch stdin to raw mode (byte-at-a-time, no echo, no line editing).
pub fn enable_raw_mode() -> io::Result<TermMode> {
    std::os::toyos::io::set_stdin_raw(true);
    Ok(TermMode)
}

/// Construct and lock a new handle to the standard input.
pub fn stdin() -> io::Result<impl BufRead> { Ok(io::stdin().lock()) }

/// Convert a filename string to a PathBuf.
pub fn path(filename: &str) -> std::path::PathBuf { std::path::PathBuf::from(filename) }
