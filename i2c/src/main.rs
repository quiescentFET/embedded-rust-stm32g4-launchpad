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
bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<embassy_stm32::peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<embassy_stm32::peripherals::I2C1>;
});

// ISM330 on X-NUCLEO-IKSO2A1 Mode 1 Address is 0xD6(8-bit), 0x6B(7-bit)
const ISM_330: u8 = 0x6B;

// Global Pulse bool
static ISM_PULSE: AtomicBool = AtomicBool::new(false);
//*** /BINDINGS & VARS ***//

//*** MAIN ***//
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let config = embassy_stm32::Config::default();
    let p = embassy_stm32::init(config);

    let i2c_config = i2c::Config::default();
    let blinky = Output::new(p.PA5, Level::Low, Speed::Low);

    // I2C1: SCL=PB7, SDA=PB8
    let controller = i2c::I2c::new(
        p.I2C1,     // peripheral
        p.PB8,      // SCL
        p.PB9,      // SDA
        Irqs,       // interrupt bindings
        p.DMA1_CH3, // TX DMA channel
        p.DMA1_CH4, // RX DMA channel
        i2c_config,
    );

    _spawner.spawn(controller_task(controller, blinky)).unwrap();
}
//*** /MAIN ***//

//*** TASKS ***//
#[embassy_executor::task]
async fn controller_task(mut con: i2c::I2c<'static, Async, i2c::Master>, mut led: Output<'static>) {
    info!("Controller start");
    loop {
        if ISM_PULSE.load(Ordering::Relaxed) == true {
        } else {
            loop {
                let mut buf = [0u8; 1];
                match con.write_read(ISM_330, &[0x0F], &mut buf).await {
                    Ok(()) => {
                        if buf[0] == 0x6B {
                            ISM_PULSE.store(true, Ordering::Relaxed);
                            info!("ISM330 Found!");
                            led.set_low();
                            break;
                        }
                    }
                    Err(e) => {
                        info!("I2C Error during read: {:?}", e);
                    }
                }
                Timer::after_secs(120).await;
            }
        }
    }
}
//*** /TASKS ***//

//*** FUNCTIONS ***//
//*** /FUNCTIONS ***//
