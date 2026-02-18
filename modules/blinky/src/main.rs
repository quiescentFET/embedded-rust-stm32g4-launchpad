#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use panic_probe as _;

#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("loading config...");
    let p = embassy_stm32::init(Default::default());
    // let mut config = embassy_stm32::Config::default();
    info!("config loaded!");

    let mut ld2 = Output::new(p.PA5, Level::Low, Speed::Low);

    loop {
        Timer::after_millis(500).await;
        ld2.toggle();
    }
}
