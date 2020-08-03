//! replace std library, implement panic and abort
use core::panic::PanicInfo;
use crate::sbi::shutdown;

/// print panic info and [`shutdown`]
///
/// `#[panic_handler]` properties
/// declare this function as callback of panic
#[panic_handler]
fn panic_hanlder(info: &PanicInfo) -> ! {
	// `\x1b[??m` is an command controling terminal output format, text color can be changed on supported platforms.Red is used here
	// reference: https://misc.flogisoft.com/bash/tip_colors_and_formatting
	//
	// need golbal options feature(panic_info_message) to call .message() function
	println!("\x1b[1;31mpanic: '{}'\x1b[0m", info.message().unwrap());
	shutdown()
}

/// stop program
///
/// call  [`panic_handler`]
#[no_mangle]
extern "C" fn abort() -> ! {
	panic!("abort()")
}