/*
 * Module: Blinky
 * Development Board: NUCLEO-G474RE (STM32G474RE)
 * Author: quiescentFET
 */

#![no_std]
#![no_main]

mod imports;
use imports::*;

#[defmt::panic_handler] // Init the defmt panic handler, uses panic_probe's handler with defmt
fn panic() -> ! {
    cortex_m::asm::udf()
}

//*** BINDINGS & VARS ***//
const DELAY_LIST: [u32; 5] = [500, 250, 125, 50, 10]; // Init the array of delays (ms) that will be cycled through
static DELAY: AtomicU32 = AtomicU32::new(500); // Init global variable as an Atomic for safe concurrency
bind_interrupts!(struct Irqs { // Bind EXTI13's interrupts to a handler (for button on PC13 pin)
    EXTI15_10 => InterruptHandler<embassy_stm32::interrupt::typelevel::EXTI15_10>;
});

// OLD METHOD
// static DELAY: CriticalSectionMutex<Cell<u64>> = CriticalSectionMutex::new(Cell::new(DELAY_LIST[0])); // Cell<u64> allows interior mutability (get/set) within the critical section
//*** /BINDINGS & VARS ***//

//*** MAIN ***//
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("loading config...");
    let mut config = embassy_stm32::Config::default(); // Load default config values

    // Modify config to enable HSE and use as SYSCLK
    config.rcc.hse = Some(rcc::Hse {
        freq: embassy_stm32::time::Hertz(24_000_000),
        mode: rcc::HseMode::Oscillator,
    });
    config.rcc.sys = rcc::Sysclk::HSE;
    let p = embassy_stm32::init(config); // Apply config and init peripherals (do once only)
    info!("config loaded!");

    let button = ExtiInput::new(p.PC13, p.EXTI13, Pull::None, Irqs); // Init B1 button with external interrupt and its handler
    let blinky = Output::new(p.PA5, Level::Low, Speed::Low); // Init LD2 LED at PA5

    _spawner.spawn(advance_timing(button)).unwrap(); //Spawn timing advancer task
    _spawner.spawn(blink_led(blinky)).unwrap(); // Spawn Blinky task
}
//*** /MAIN ***//

//*** TASKS ***//
#[embassy_executor::task] // Blink LED Task
async fn blink_led(mut blinky: Output<'static>) {
    info!("Blinky Started!"); // Print

    // Start blinking
    loop {
        Timer::after_millis(DELAY.load(Ordering::Relaxed) as u64).await; // Read delay value inside a critical section and wait
        blinky.toggle();

        // OLD METHOD
        // Timer::after_millis(critical_section::with(|cs| DELAY.borrow(cs).get())).await;
    }
}

#[embassy_executor::task] // Advance Timing Task (button press)
async fn advance_timing(mut button: ExtiInput<'static>) {
    let mut delay_iter = DELAY_LIST.iter().cycle(); // Init cycling iterator for delay list
    DELAY.store(*delay_iter.next().unwrap(), Ordering::Relaxed); // Advance interator to second value since first is already in effect

    // OLD METHOD
    // critical_section::with(|cs| DELAY.borrow(cs).set(*delay_iter.next().unwrap()));

    loop {
        button.wait_for_rising_edge().await;
        DELAY.store(*delay_iter.next().unwrap(), Ordering::Relaxed); // Write new delay value inside a critical section
        info!("Button pressed!");

        // OLD METHOD
        // critical_section::with(|cs| DELAY.borrow(cs).set(*delay_iter.next().unwrap()));
    }
}
//*** /TASKS ***//
