#![no_std]
#![no_main]

// use core::cell::RefCell; // Not needed, embassy's mutex handles mutability
use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{AnyPin, Input, Level, Output, Pull, Speed};
use embassy_time::Timer;
// use embassy_sync::mutex::Mutex; // Not needed, use CriticalSectionMutex instead
use embassy_sync::blocking_mutex::CriticalSectionMutex; // use CritSecMutex over Thread/Noop due to interruptions from GPIO (button press)
use panic_probe as _;

#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

// Initialize the atomic variable with the first delay value
const DELAY_LIST: [u64; 3] = [500, 250, 125];

// Init the CURR_DELAY as a Mutex to prevent multiple access, and use a RefCell to allow mutability through a shared reference
static DELAY: CriticalSectionMutex<u64> = CriticalSectionMutex::new(DELAY_LIST[0]);

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("loading config...");
    let p = embassy_stm32::init(Default::default());
    info!("config loaded!");

    //Init B1 Button
    // let mut b1 = Input::new(p.PC13, Pull::None);
    _spawner.spawn(blink_led(p.PA5)).await;
}

#[embassy_executor::task]
async fn blink_led(led) {
    // Init LD2 LED
    let mut ld2 = Output::new(led, Level::Low, Speed::Low);

    loop {
        Timer::after_millis(*DELAY.borrow(cs)).await;
        ld2.toggle();
    }
}

// TODO Get info! defmt to show on probe-rs/console
