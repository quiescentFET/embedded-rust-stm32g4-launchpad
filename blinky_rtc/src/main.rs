/*
 * Module: Blinky RTC
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

// Init the array of ticks that will be cycled through
// Use ticks to save math (1 tick = 1/32768s, e.g. 500ms = 16384 ticks) TODO: 3
// const DELAY_LIST: [u64; 5] = [16384, 8192, 4096, 1639, 328];
// In milliseconds = [500, 250, 125, ~50, ~10]
const DELAY_LIST: [u64; 5] = [500, 250, 125, 50, 10];

// Init global variable as a mutex to prevent multiple access
// Cell<u64> allows interior mutability (get/set) within the critical section
static DELAY: CriticalSectionMutex<Cell<u64>> = CriticalSectionMutex::new(Cell::new(DELAY_LIST[0]));

// Bind EXTI13's interrupts to a handler (for button on PC13 pin)
bind_interrupts!(struct Irqs {
    EXTI15_10 => InterruptHandler<embassy_stm32::interrupt::typelevel::EXTI15_10>;
});
//*** /BINDINGS & VARS ***//

//*** MAIN ***//
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Init device
    info!("loading config...");

    // Load default config values
    let mut config = embassy_stm32::Config::default();

    // Enable 32.768KHz LSE for RTC TODO: 3
    // config.rcc.ls = rcc::LsConfig::default_lse();
    // Not useful atm, embassy time based on SYSCLK ticks,need LSE/lptim1 time-driver

    // Modify config to enable HSE and use as SYSCLK
    config.rcc.hse = Some(rcc::Hse {
        freq: embassy_stm32::time::Hertz(24_000_000),
        mode: rcc::HseMode::Oscillator,
    });
    config.rcc.sys = rcc::Sysclk::HSE;

    // Apply config and init peipherals
    let p = embassy_stm32::init(config);
    info!("config loaded!");

    // Init B1 button with external interrupt and its handler
    let button = ExtiInput::new(p.PC13, p.EXTI13, Pull::None, Irqs);

    // Init LD2 LED at PA5
    let blinky = Output::new(p.PA5, Level::Low, Speed::Low);

    //Spawn timing advancer task
    _spawner.spawn(advance_timing(button)).unwrap();

    // Spawn Blinky task
    _spawner.spawn(blink_led(blinky)).unwrap();
}
//*** /MAIN ***//

//*** TASKS ***//

// Blink LED Task
#[embassy_executor::task]
async fn blink_led(mut blinky: Output<'static>) {
    // Print
    info!("Blinky Started!");

    // Start blinking
    loop {
        // Read delay value inside a critical section and wait
        Timer::after_millis(critical_section::with(|cs| DELAY.borrow(cs).get())).await;
        // Timer::after_ticks(critical_section::with(|cs| DELAY.borrow(cs).get())).await; // TODO: 3
        blinky.toggle();
    }
}

// Advance Timing Task (button press)
#[embassy_executor::task]
async fn advance_timing(mut button: ExtiInput<'static>) {
    // Init cycling iterator for delay list
    let mut delay_iter = DELAY_LIST.iter().cycle();

    // Advance interator to second value since first is already in effect
    critical_section::with(|cs| DELAY.borrow(cs).set(*delay_iter.next().unwrap()));

    loop {
        button.wait_for_rising_edge().await;
        // Write new delay value inside a critical section
        critical_section::with(|cs| DELAY.borrow(cs).set(*delay_iter.next().unwrap()));
        info!("Button pressed!");
    }
}
//*** /TASKS ***//
