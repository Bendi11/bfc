#![no_main]
#![no_std]

use cortex_m_rt::entry;

#[entry]
fn entry() -> ! {
    loop {}
}

#[panic_handler]
fn panic_halt(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
