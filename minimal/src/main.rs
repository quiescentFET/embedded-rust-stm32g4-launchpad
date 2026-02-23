/*
 * Module: Minimal
 * Development Board: NUCLEO-G474RE (STM32G474RE)
 * Author: quiescentFET
 */

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
    let config = embassy_stm32::Config::default(); // Load default config values
    let p = embassy_stm32::init(config); // Use default values and init peripherals
    info!("config loaded!");
    loop {}
}
