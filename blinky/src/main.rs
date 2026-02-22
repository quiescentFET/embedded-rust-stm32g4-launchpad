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

// Init the array of delay values (ms) that will be cycled through
const DELAY_LIST: [u64; 5] = [500, 250, 125, 50, 10];

// Init global variable as a mutex to prevent multiple access
// Cell<u64> allows interior mutability (get/set) within the critical section
static DELAY: CriticalSectionMutex<Cell<u64>> = CriticalSectionMutex::new(Cell::new(DELAY_LIST[0]));

// Bind EXTI13's interrupts to a handler (for button on PC13 pin)
bind_interrupts!(struct Irqs {
    EXTI15_10 => InterruptHandler<EXTI15_10>;
});
//*** /BINDINGS & VARS ***//

//*** MAIN ***//

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Load config (do once only)
    info!("loading config...");
    let p = embassy_stm32::init(Default::default()); // Initialize the peripherals
    info!("config loaded!");

    // Init B1 button with external interrupt and its handler
    let button = ExtiInput::new(p.PC13, p.EXTI13, Pull::None, Irqs);

    // Init LD2 LED at PA5
    let blinky = Output::new(p.PA5, Level::Low, Speed::Low);

    //Spawn timing advancer task
    _spawner.spawn(advance_timing(button)).unwrap();

    // Spawn Blinky task with pin for LD2
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
        // Read delay value inside a critical section
        Timer::after_millis(critical_section::with(|cs| DELAY.borrow(cs).get())).await;
        blinky.toggle();
    }
}

// Advance Timing Task (button press)
#[embassy_executor::task]
async fn advance_timing(mut button: ExtiInput<'static>) {
    // Init cycling iterator for delay list
    let mut delay_iter = DELAY_LIST.iter().cycle();

    loop {
        button.wait_for_rising_edge().await;
        // Write new delay value inside a critical section
        critical_section::with(|cs| DELAY.borrow(cs).set(*delay_iter.next().unwrap()));
        info!("Button pressed!");
    }
}
//*** /TASKS ***//
