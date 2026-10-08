#![no_std]
#![no_main]

use cortex_m_rt::entry;

#[entry]
unsafe fn main() -> ! {
    panic!("Main should never exit")
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
