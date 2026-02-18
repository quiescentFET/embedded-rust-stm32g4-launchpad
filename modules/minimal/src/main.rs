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
    let config = embassy_stm32::Config::default();
    info!("config loaded!");
    loop {
        info!("Loop running!");
    }
}

// TODO Get info! defmt to show on probe-rs/console
