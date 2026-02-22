#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use panic_probe as _;

#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("loading config...");
    let config = embassy_stm32::Config::default(); // Need to initialize the chip for compilation
    info!("config loaded!");
    loop {}
}
