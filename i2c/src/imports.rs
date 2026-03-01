pub use core::sync::atomic::{AtomicBool, Ordering};
pub use defmt::info;
pub use embassy_executor::Spawner;
pub use embassy_stm32::gpio::{Level, Output, Speed};
pub use embassy_stm32::mode::Async;
pub use embassy_stm32::{bind_interrupts, i2c};
pub use embassy_time::Timer;
pub use {defmt_rtt as _, panic_probe as _}; // Import side effects
