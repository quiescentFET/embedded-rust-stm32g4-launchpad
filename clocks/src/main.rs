/*
 * Module: Clocks
 * Development Board: NUCLEO-G474RE (STM32G474RE)
 * Author: quiescentFET
 */

#![no_std]
#![no_main]

mod imports;
use imports::*;

// Init the defmt panic handler, uses panic_probe's handler with defmt
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

//*** BINDINGS & VARS ***//
//*** /BINDINGS & VARS ***//

//*** MAIN ***//
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Load config (do once only)
    info!("loading config...");
    let mut config = embassy_stm32::Config::default();
    config.rcc.ls = rcc::LsConfig::default_lse();
    config.rcc.hse = Some(rcc::Hse {
        freq: embassy_stm32::time::Hertz(24_000_000),
        mode: rcc::HseMode::Oscillator,
    });
    config.rcc.sys = rcc::Sysclk::HSE;
    let p = embassy_stm32::init(config); // Initialize the peripherals
    info!("config loaded!");

    let clocks = embassy_stm32::rcc::clocks(&p.RCC);
    info!("SYSCLK: {} Hz", clocks.sys.to_hertz().unwrap().0);
    info!("HCLK:   {} Hz", clocks.hclk1.to_hertz().unwrap().0);
    info!("PCLK1:  {} Hz", clocks.pclk1.to_hertz().unwrap().0);
    info!("PCLK2:  {} Hz", clocks.pclk2.to_hertz().unwrap().0);
}
//*** /MAIN ***//

//*** TASKS ***//
//*** /TASKS ***//
