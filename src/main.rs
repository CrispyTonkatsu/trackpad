#![no_std]
#![no_main]

use core::ptr::write_volatile;

use cortex_m_rt::entry;

#[entry]
unsafe fn main() -> ! {
    let port_address: u32 = 0x5000_0000;

    let dirset = (port_address + 0x518) as *mut u32;
    let outclr = (port_address + 0x50C) as *mut u32;

    let pin_number: u32 = 1 << 26;

    unsafe {
        // Setting the VTOR table to the right address
        write_volatile(0xE000_ED08 as *mut u32, 0x00027000);

        write_volatile(dirset, pin_number);
        write_volatile(outclr, pin_number);
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
