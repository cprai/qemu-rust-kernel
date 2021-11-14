#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}


#[no_mangle]
pub extern "C" fn _start() -> ! {
    for i in 0..100 {
        println!("Printing line {}.", i + 1);
    }
    println!("Finished printing 100 lines.");

    panic!("Kernel ran out of code to execute");
}
