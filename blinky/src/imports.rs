pub use core::sync::atomic::{AtomicU32, Ordering};
pub use defmt::info;
pub use defmt_rtt as _;
pub use embassy_executor::Spawner;
pub use embassy_stm32::bind_interrupts;
pub use embassy_stm32::exti::{ExtiInput, InterruptHandler}; // Import external interrupt wrapper for inputs
pub use embassy_stm32::gpio::{Level, Output, Pull, Speed};
pub use embassy_stm32::rcc;
pub use embassy_time::Timer;
pub use panic_probe as _; // Import panic handler as a side effect
