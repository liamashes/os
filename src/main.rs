//! # global properties
//! - `#![no_std]`
//!   ban std lib
#![no_std]
//!
//! - `#![no_main]`
#![no_main]
#![feature(llvm_asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod panic;
mod sbi;

global_asm!(include_str!("asm/entry.asm"));


/// cover _start function of crt0
/// we temporarily implement it as an endless loop
/// no name mangling during debug
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
	  // output "OK\n", then endless loop
	  println!("hello rCore!");
	  panic!("end of rust_main")
}
