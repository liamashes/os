//! Realize the character input and output of the console
//!
//! # format output
//!
//! [`core::fmt::Write`] trait 
//! -- need [`write_str`]
//! -- comes with implementation, but depends on [`write_str`] and [`write_fmt`]
//!
//! we declare one type, implement [`write_str`], then we can use [`write_fmt`] to format output
//!
//! [`write_str`]: core::fmt::Write::write_str
//! [`write_fmt`]: core::fmt::Write::write_fmt

use crate::sbi::*;
use core::fmt::{self, Write};

/// one [Zero-Sized Type], implement [`core::fmt::Write`] trait to format output
/// ZST only have one value(null), so it is an single piece in itself
struct Stdout;

impl Write for Stdout{
	/// print string
	/// [`console_putchar`] sbi accept one `usize` during each call，but in fact it will be used as `u8` to print string
  /// so, if non ASCII char exists，we need call  [`console_putchar`] for each `u8` in utf-8 encoding
  fn write_str(&mut self, s: &str) -> fmt::Result {
  	let mut buffer = [0u8; 4];
  	for c in s.chars() {
  		for code_point in c.encode_utf8(&mut buffer).as_bytes().iter() {
  			console_putchar(*code_point as usize);
  		}
  	}
  	Ok(())
  }
}


/// print data after [`core::format_args!`]'s formating
///
/// [`print!`] and [`println!`] Macro will expand into this function
///
/// [`core::format_args!`]: https://doc.rust-lang.org/nightly/core/macro.format_args.html
pub fn print(args: fmt::Arguments){
	Stdout.write_fmt(args).unwrap();
}

/// implement method like `print!` macro from stdlib
///
/// use [`core::fmt::Write`] trait's [`console::Stdout`]
#[macro_export]
macro_rules! print {
	($fmt: literal $(, $($arg: tt)+)?) => {
		$crate::console::print(format_args!($fmt $(, $($arg)+)?));
	}
}

/// implement method like `println!` macro from stdlib
///
/// use [`core::fmt::Write`] trait's [`console::Stdout`]
#[macro_export]
macro_rules! println {
	($fmt: literal $(, $($arg: tt)+)?) => {
		$crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
	}
}