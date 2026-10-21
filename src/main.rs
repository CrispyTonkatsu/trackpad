#![no_std]
#![no_main]

use core::ptr::write_volatile;

use cortex_m_rt::entry;

use defmt_rtt as _;
use panic_probe as _;

#[entry]
fn main() -> ! {
    let port_address: u32 = 0x5000_0000;

    let dirset = (port_address + 0x518) as *mut u32;
    let outclr = (port_address + 0x50C) as *mut u32;

    let pin_number: u32 = 1 << 26;

    unsafe {
        write_volatile(dirset, pin_number);
        write_volatile(outclr, pin_number);
    }

    defmt::info!("hihihihi");

    loop {}
}
