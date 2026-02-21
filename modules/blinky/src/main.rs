#![no_std]
#![no_main]

mod imports;
use imports::*;

// Init the defmt panic handler, uses panic_probe's handler with defmt
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

// Init the array of delay values (ms)
const DELAY_LIST: [u64; 3] = [500, 250, 125];

// Init global variable as a mutex to prevent multiple access
static DELAY: CriticalSectionMutex<u64> = CriticalSectionMutex::new(DELAY_LIST[0]); // CritSecMutex handles shared references

// Init the interrupt handler for EXTI13 (for button on PC13 pin)
bind_interrupts!(struct Irqs {
    EXTI15_10 => InterruptHandler<EXTI15_10>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("loading config...");
    let p = embassy_stm32::init(Default::default()); // Initialize the peripherals
    info!("config loaded!");

    // Init B1 button with external interrupt
    let button = ExtiInput::new(p.PC13, p.EXTI13, Pull::None, Irqs);

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
        // let delay_ref = DELAY.get_mut();
        // delay_ref =
        info!("Button pressed!");
    }
}
