// Based on https://github.com/sgmarz/osblog

use crate::uart;
use alloc::{collections::VecDeque, sync::Arc};
use core::fmt::{self, Write};
use lazy_static::*;
use spin::Mutex;

pub const DEFAULT_OUT_BUFFER_SIZE: usize = 10_000;
pub const DEFAULT_IN_BUFFER_SIZE: usize = 1_000;

lazy_static! {
    pub static ref IN_BUFFER: Arc<Mutex<VecDeque<u8>>> =
        Arc::new(Mutex::new(VecDeque::with_capacity(DEFAULT_IN_BUFFER_SIZE)));
    pub static ref OUT_BUFFER: Arc<Mutex<VecDeque<u8>>> =
        Arc::new(Mutex::new(VecDeque::with_capacity(DEFAULT_OUT_BUFFER_SIZE)));
}

#[cfg(any(feature = "board_qemu", feature = "board_lrv"))]
#[allow(dead_code)]
pub fn push_stdout(c: u8) {
    let uart = uart::UART.lock();
    if !uart.is_transmitter_holding_register_empty_interrupt_enabled() {
        uart.write_byte(c);
        uart.enable_transmitter_holding_register_empty_interrupt();
    } else {
        let mut out_buffer = OUT_BUFFER.lock();
        if out_buffer.len() < DEFAULT_OUT_BUFFER_SIZE {
            out_buffer.push_back(c);
        }
    }
}

#[cfg(feature = "board_lrv_uartlite")]
#[allow(dead_code)]
pub fn push_stdout(c: u8) {
    let uart = uart::UART.lock();
    if uart.is_tx_fifo_empty() && OUT_BUFFER.lock().is_empty() {
        uart.write_byte(c);
    } else {
        let mut out_buffer = OUT_BUFFER.lock();
        if out_buffer.len() < DEFAULT_OUT_BUFFER_SIZE {
            out_buffer.push_back(c);
        }
    }
}

#[allow(dead_code)]
pub fn pop_stdout() -> u8 {
    let mut out_buffer = OUT_BUFFER.lock();
    out_buffer.pop_front().unwrap_or(0)
}

#[allow(dead_code)]
pub fn push_stdin(c: u8) {
    let mut in_buffer = IN_BUFFER.lock();
    if in_buffer.len() < DEFAULT_IN_BUFFER_SIZE {
        in_buffer.push_back(c);
    }
}

pub fn pop_stdin() -> u8 {
    let mut in_buffer = IN_BUFFER.lock();
    if let Some(ch) = in_buffer.pop_front() {
        ch
    } else {
        #[cfg(any(feature = "board_qemu", feature = "board_lrv"))]
        {
            // Drain UART Rx FIFO
            let uart = uart::UART.lock();
            while let Some(ch_read) = uart.read_byte() {
                in_buffer.push_back(ch_read);
            }
        }
        in_buffer.pop_front().unwrap_or(0)
    }
}

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            push_stdout(c as u8);
        }
        Ok(())
    }
}

#[allow(dead_code)]
pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console_blog::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console_blog::print(format_args!(concat!($fmt, "\r\n") $(, $($arg)+)?));
    }
}
