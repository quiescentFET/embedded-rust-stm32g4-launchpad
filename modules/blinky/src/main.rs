#![no_std]
#![no_main]

// use core::cell::RefCell; // Not needed, embassy's mutex handles mutability
use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::Peri;
use embassy_stm32::bind_interrupts;
use embassy_stm32::exti::{ExtiInput, InterruptHandler}; // Import external interrupt wrapper for inputs
use embassy_stm32::gpio::{Level, Output, Pull, Speed};
use embassy_stm32::interrupt::typelevel::EXTI15_10; // Import interrupt typelevel that button will bind to
use embassy_stm32::peripherals::{EXTI13, PA5}; // Use a concrete peripheral type rather than p.PA5
use embassy_time::Timer;
// use embassy_sync::mutex::Mutex; // Not needed, use CriticalSectionMutex instead
use embassy_sync::blocking_mutex::CriticalSectionMutex; // use CritSecMutex over Thread/Noop due to interruptions from GPIO (button press)
use panic_probe as _;

#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

// Init the atomic variable with the first delay value
const DELAY_LIST: [u64; 3] = [500, 250, 125];

// Init the CURR_DELAY as a Mutex to prevent multiple access, and use a RefCell to allow mutability through a shared reference
static DELAY: CriticalSectionMutex<u64> = CriticalSectionMutex::new(DELAY_LIST[0]);

// Init the interrupt handler for EXTI13
bind_interrupts!(struct Irqs {
    EXTI15_10 => InterruptHandler<EXTI15_10>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("loading config...");
    let p = embassy_stm32::init(Default::default()); // Initialize the peripherals
    info!("config loaded!");

    // Init B1 button with external interrupt
    let mut button = ExtiInput::new(p.PC13, p.EXTI13, Pull::None, Irqs);

    //Spawn timing advancer task
    _spawner.spawn(advance_timing(button)).unwrap();

    // Spawn Blinky task with pin for LD2
    _spawner.spawn(blink_led(p.PA5)).unwrap();
}

// Blink LED Task
#[embassy_executor::task]
async fn blink_led(led: Peri<'static, PA5>) {
    // Init LD2 LED
    let mut blinky = Output::new(led, Level::Low, Speed::Low);
    info!("Blinky Started!");

    loop {
        // Access delay value by disabling interrupts, then borrowing.
        Timer::after_millis(critical_section::with(|cs| *DELAY.borrow(cs))).await;
        blinky.toggle();
    }
}

// Advance Timing Task (button press)
#[embassy_executor::task]
async fn advance_timing(mut button: ExtiInput<'static>) {
    // Init cycling interator for delay list
    // let mut delay_iter = DELAY_LIST.iter().cycle();

    loop {
        button.wait_for_rising_edge().await;
        // DELAY.get_mut() = *delay_iter.next().unwrap();
        info!("Button pressed!");
    }
}
