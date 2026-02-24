/*
 * Module: I2C
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
    let p = embassy_stm32::init(Default::default()); // Apply config and init peripherals
    info!("config loaded!");
}
//*** /MAIN ***//

//*** TASKS ***//
//*** /TASKS ***//
