#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

#[entry] // Requires function signature never returns so -> ! does that
fn main() -> ! {
    loop {}
}
