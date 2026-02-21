// pub use core::cell::RefCell; // Not needed, embassy's mutex handles mutability

pub use defmt::info;
pub use defmt_rtt as _;
pub use embassy_executor::Spawner;
pub use embassy_stm32::Peri;
pub use embassy_stm32::bind_interrupts;
pub use embassy_stm32::exti::{ExtiInput, InterruptHandler}; // Import external interrupt wrapper for inputs
pub use embassy_stm32::gpio::{Level, Output, Pull, Speed};
pub use embassy_stm32::interrupt::typelevel::EXTI15_10; // Import interrupt typelevel that button will bind to
pub use embassy_stm32::peripherals::PA5; // Use a concrete peripheral type rather than p.PA5
pub use embassy_sync::blocking_mutex::CriticalSectionMutex; // use CritSecMutex over Thread/Noop due to interruptions from GPIO (button press)
// pub use embassy_sync::mutex::Mutex; // Not needed, use CriticalSectionMutex instead
pub use embassy_time::Timer;
pub use panic_probe as _; // Import panic handler as a side effect
