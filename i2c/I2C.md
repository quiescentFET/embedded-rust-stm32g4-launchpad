# I2C with Embassy STM32

Reference for using `embassy_stm32::i2c` on the NUCLEO-G474RE (STM32G474RE).

---

## Table of Contents

1. [Overview](#overview)
2. [Cargo.toml Dependencies](#cargotoml-dependencies)
3. [Core Types](#core-types)
4. [Configuration](#configuration)
5. [Interrupt Binding](#interrupt-binding)
6. [Constructing an I2c Instance](#constructing-an-i2c-instance)
7. [Master Mode Operations](#master-mode-operations)
8. [Slave / MultiMaster Mode](#slave--multimaster-mode)
9. [Error Handling](#error-handling)
10. [Full Working Example](#full-working-example)

---

## Overview

The `embassy_stm32::i2c` module provides async and blocking I2C for STM32 peripherals.
Two operating modes are exposed as type-level markers:

| Mode | Type | Description |
|------|------|-------------|
| Master | `i2c::Master` | Single master on the bus |
| MultiMaster | `i2c::MultiMaster` | Can act as both master and slave |

The `I2c` struct is generic over the mode: `I2c<'d, Async, M>` or `I2c<'d, Blocking, M>`.

---

## Cargo.toml Dependencies

```toml
[dependencies]
embassy-stm32 = { version = "0.5.0", features = [
    "defmt", "memory-x", "stm32g474re", "single-bank"
]}
embassy-executor = { version = "0.9.1", features = [
    "arch-cortex-m", "defmt", "executor-thread"
]}
embassy-time = "0.5.0"
embassy-sync = { version = "0.7.2", features = ["defmt"] }
defmt = "1.0.1"
defmt-rtt = "1.1.0"
panic-probe = { version = "1.0.0", features = ["print-defmt"] }
cortex-m = { version = "0.7.7", features = ["critical-section-single-core"] }
cortex-m-rt = "0.7.5"
```

---

## Core Types

### `i2c::I2c<'d, Mode, BusMode>`

The main driver struct. Mode is either `Async` or `Blocking` (from `embassy_stm32::mode`).
BusMode is either `i2c::Master` or `i2c::MultiMaster`.

### `i2c::Config`

Bus configuration passed at construction time.

| Field | Type | Description |
|-------|------|-------------|
| `frequency` | `Hertz` | Clock frequency (default 100 kHz) |
| `gpio_speed` | — | GPIO pin speed setting |
| `scl_pullup` | `bool` | Enable internal SCL pull-up |
| `sda_pullup` | `bool` | Enable internal SDA pull-up |
| `timeout` | `Duration` | Per-operation timeout |

### `i2c::SlaveAddrConfig`

Configures the address(es) a slave/multimaster device responds to.

| Field | Type | Description |
|-------|------|-------------|
| `addr` | `OwnAddresses` | Which own address(es) to enable |
| `general_call` | `bool` | Respond to general-call address (0x00) |

### `i2c::Address`

```rust
pub enum Address {
    SevenBit(u8),   // 7-bit address (most common)
    TenBit(u16),    // 10-bit extended address
}
```

### `i2c::OwnAddresses`

```rust
pub enum OwnAddresses {
    OA1(Address),           // Primary own address only
    OA2(Address),           // Secondary own address only
    Both(Address, Address), // Both OA1 and OA2
}
```

### `i2c::SlaveCommand`

Returned by `listen()`. Describes what the master wants.

```rust
pub struct SlaveCommand {
    pub kind: SlaveCommandKind,
    pub address: Address,
}

pub enum SlaveCommandKind {
    Read,   // Master wants to read from us
    Write,  // Master wants to write to us
}
```

### `i2c::SendStatus`

Returned by `respond_to_read()`.

```rust
pub enum SendStatus {
    Done,                  // All bytes consumed
    LeftoverBytes(usize),  // Master NAK'd before all bytes were read
}
```

### `i2c::Error`

```rust
pub enum Error {
    Arbitration,          // Bus arbitration lost
    Bus,                  // General bus error
    Crc,                  // CRC check failure
    Nack,                 // No acknowledgement from target
    Overrun,              // Data received faster than processed
    Timeout,              // Operation timed out
    ZeroLengthTransfer,   // Attempted zero-length transfer
}
```

---

## Configuration

```rust
use embassy_stm32::time::Hertz;
use embassy_stm32::i2c;

let mut config = i2c::Config::default(); // 100 kHz, no pull-ups
config.frequency = Hertz::khz(400);     // Fast mode: 400 kHz
```

Standard frequencies:

| Speed | Frequency |
|-------|-----------|
| Standard | `Hertz::khz(100)` |
| Fast | `Hertz::khz(400)` |
| Fast-Plus | `Hertz::mhz(1)` |

---

## Interrupt Binding

Async mode requires binding both the event and error interrupt handlers for each I2C peripheral.

```rust
use embassy_stm32::{bind_interrupts, i2c, peripherals};

bind_interrupts!(struct Irqs {
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C2_ER => i2c::ErrorInterruptHandler<peripherals::I2C2>;
    I2C2_EV => i2c::EventInterruptHandler<peripherals::I2C2>;
});
```

One `_ER`/`_EV` pair per peripheral. Only bind the peripherals you use.

---

## Constructing an I2c Instance

### Async (DMA-backed) — Master Mode

```rust
// I2C1: SCL=PB7, SDA=PB8
let controller = i2c::I2c::new(
    p.I2C1,      // peripheral
    p.PB7,       // SCL
    p.PB8,       // SDA
    Irqs,        // interrupt bindings
    p.DMA1_CH3,  // TX DMA channel
    p.DMA1_CH4,  // RX DMA channel
    config,
);
// Type: I2c<'static, Async, Master>
```

### Async — MultiMaster (Slave) Mode

```rust
use embassy_stm32::i2c::{Address, OwnAddresses, SlaveAddrConfig};

const DEV_ADDR: u8 = 0x42;

let slave_config = i2c::SlaveAddrConfig {
    addr: OwnAddresses::OA1(Address::SevenBit(DEV_ADDR)),
    general_call: false,
};

let device = i2c::I2c::new(
    p.I2C2,
    p.PA9,       // SCL
    p.PA8,       // SDA
    Irqs,
    p.DMA1_CH1,
    p.DMA1_CH2,
    config,
).into_slave_multimaster(slave_config);
// Type: I2c<'static, Async, MultiMaster>
```

### Blocking — Master Mode

```rust
let controller = i2c::I2c::new_blocking(
    p.I2C1,
    p.PB7,  // SCL
    p.PB8,  // SDA
    config,
);
// No DMA channels, no interrupts needed
```

---

## Master Mode Operations

All async methods are `await`-able. Blocking equivalents have a `blocking_` prefix.

### Write

Send bytes to a device at `addr`.

```rust
// Async
con.write(DEV_ADDR, &[0xC2, value]).await?;

// Blocking
con.blocking_write(DEV_ADDR, &[0xC2, value])?;
```

### Read

Read bytes from a device at `addr`.

```rust
let mut buf = [0u8; 4];

// Async
con.read(DEV_ADDR, &mut buf).await?;

// Blocking
con.blocking_read(DEV_ADDR, &mut buf)?;
```

### Write-then-Read (Combined Transaction)

Sends a write then a repeated-start read without releasing the bus.
This is the standard pattern for register reads.

```rust
let mut resp = [0u8; 1];

// Async: send register address 0xC2, read 1 byte back
con.write_read(DEV_ADDR, &[0xC2, payload], &mut resp).await?;

// Blocking
con.blocking_write_read(DEV_ADDR, &[0xC2], &mut resp)?;
```

### Vectored Write

Write from multiple non-contiguous buffers in a single transaction.

```rust
let header = [0xC2u8];
let data   = [0x01, 0x02, 0x03];

// Async
con.write_vectored(DEV_ADDR, &[&header, &data]).await?;

// Blocking
con.blocking_write_vectored(DEV_ADDR, &[&header, &data])?;
```

### Transaction (Advanced)

Compose arbitrary sequences of reads and writes with repeated starts.

```rust
use embedded_hal::i2c::{Operation};

let mut buf = [0u8; 2];
con.transaction(DEV_ADDR, &mut [
    Operation::Write(&[0x01]),
    Operation::Read(&mut buf),
]).await?;
```

---

## Slave / MultiMaster Mode

In MultiMaster mode the device can receive commands from a master while also acting as a master itself on the same bus.

### Listening for Commands

`listen()` suspends until the master addresses this device.

```rust
match dev.listen().await {
    Ok(i2c::SlaveCommand { kind: SlaveCommandKind::Read, .. }) => {
        // Master wants to read data from us
    }
    Ok(i2c::SlaveCommand { kind: SlaveCommandKind::Write, .. }) => {
        // Master wants to write data to us
    }
    Err(e) => error!("listen error: {}", e),
}
```

### Responding to a Read Request

```rust
match dev.respond_to_read(&[state_byte]).await {
    Ok(SendStatus::Done) => {}
    Ok(SendStatus::LeftoverBytes(n)) => {
        info!("master didn't read {} bytes", n);
    }
    Err(e) => error!("respond_to_read error: {}", e),
}
```

### Responding to a Write Request

```rust
let mut buf = [0u8; 128];
match dev.respond_to_write(&mut buf).await {
    Ok(len) => {
        // buf[..len] contains the received bytes
        info!("received {} bytes: {}", len, buf[..len]);
    }
    Err(e) => error!("respond_to_write error: {}", e),
}
```

### Write-then-Read Transaction (Slave Side)

A master may perform a write immediately followed by a read (with repeated start).
After handling the write, call `respond_to_read()`. If the master only did a write
and no read follows, `respond_to_read()` returns `Error::Timeout`.

```rust
Ok(SlaveCommandKind::Write) => {
    let len = dev.respond_to_write(&mut buf).await?;
    // Process buf[..len] ...
    match dev.respond_to_read(&[result]).await {
        Ok(status) => info!("read phase: {}", status),
        Err(i2c::Error::Timeout) => {
            // Master did a write-only transaction, no read followed
        }
        Err(e) => error!("{}", e),
    }
}
```

---

## Error Handling

All operations return `Result<_, i2c::Error>`. Use `match` or `?` to propagate.

```rust
match con.write_read(DEV_ADDR, &[0x01], &mut buf).await {
    Ok(_)                           => { /* success */ }
    Err(i2c::Error::Nack)          => { /* device not present or NAK'd */ }
    Err(i2c::Error::Timeout)       => { /* bus hung or device slow */ }
    Err(i2c::Error::Arbitration)   => { /* lost bus in multimaster */ }
    Err(e)                         => error!("i2c error: {}", e),
}
```

---

## Full Working Example

The example below runs two tasks on the same MCU: one acts as an I2C master (controller),
the other as a slave (device) using MultiMaster mode on I2C2.

```rust
#![no_std]
#![no_main]

mod imports;
use imports::*;

#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

bind_interrupts!(struct Irqs {
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C2_ER => i2c::ErrorInterruptHandler<peripherals::I2C2>;
    I2C2_EV => i2c::EventInterruptHandler<peripherals::I2C2>;
});

const DEV_ADDR: u8 = 0x42;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let mut config = i2c::Config::default();
    config.frequency = Hertz::khz(400);

    // --- Device (slave) on I2C2 ---
    let slave_config = i2c::SlaveAddrConfig {
        addr: OwnAddresses::OA1(Address::SevenBit(DEV_ADDR)),
        general_call: false,
    };
    let device = i2c::I2c::new(p.I2C2, p.PA9, p.PA8, Irqs, p.DMA1_CH1, p.DMA1_CH2, config)
        .into_slave_multimaster(slave_config);
    _spawner.spawn(unwrap!(device_task(device)));

    // --- Controller (master) on I2C1 ---
    let controller = i2c::I2c::new(p.I2C1, p.PB7, p.PB8, Irqs, p.DMA1_CH3, p.DMA1_CH4, config);
    _spawner.spawn(unwrap!(controller_task(controller)));
}

// Slave task: maintains a state byte that the master can set or reset
#[embassy_executor::task]
async fn device_task(mut dev: i2c::I2c<'static, Async, i2c::MultiMaster>) -> ! {
    let mut state: u8 = 0;
    loop {
        let mut buf = [0u8; 128];
        match dev.listen().await {
            // Master wants to read our state
            Ok(i2c::SlaveCommand {
                kind: SlaveCommandKind::Read,
                address: Address::SevenBit(DEV_ADDR),
            }) => {
                match dev.respond_to_read(&[state]).await {
                    Ok(i2c::SendStatus::LeftoverBytes(n)) => info!("{} extra bytes not read", n),
                    Ok(i2c::SendStatus::Done) => {}
                    Err(e) => error!("respond_to_read: {}", e),
                }
            }

            // Master wants to write a command
            Ok(i2c::SlaveCommand {
                kind: SlaveCommandKind::Write,
                address: Address::SevenBit(DEV_ADDR),
            }) => {
                match dev.respond_to_write(&mut buf).await {
                    Ok(len) => {
                        info!("received: {}", buf[..len]);
                        let updated = match buf[0] {
                            0xC2 => { state = buf[1]; true }   // set state
                            0xC8 => { state = 0;      true }   // reset state
                            x    => { error!("unknown cmd {:x}", x); false }
                        };
                        if updated {
                            // Handle optional read phase of a write/read transaction
                            match dev.respond_to_read(&[state]).await {
                                Ok(status) => info!("write/read read phase: {}", status),
                                Err(i2c::Error::Timeout) => {} // write-only, no read
                                Err(e) => error!("respond_to_read: {}", e),
                            }
                        }
                    }
                    Err(e) => error!("respond_to_write: {}", e),
                }
            }

            Ok(i2c::SlaveCommand { address, .. }) => {
                defmt::unreachable!("unexpected address: {}", address);
            }
            Err(e) => error!("listen: {}", e),
        }
    }
}

// Master task: cycles through write/read and read transactions
#[embassy_executor::task]
async fn controller_task(mut con: i2c::I2c<'static, Async, i2c::Master>) {
    loop {
        let mut resp = [0u8; 1];

        // Set state to values 0..9, verify each round-trip
        for i in 0..10u8 {
            match con.write_read(DEV_ADDR, &[0xC2, i], &mut resp).await {
                Ok(_) => {
                    info!("set state={}, got back {}", i, resp[0]);
                    defmt::assert_eq!(resp[0], i);
                }
                Err(e) => error!("write_read: {}", e),
            }
            Timer::after_millis(100).await;
        }

        // Read state (should be 9)
        match con.read(DEV_ADDR, &mut resp).await {
            Ok(_) => {
                info!("read state={}", resp[0]);
                defmt::assert_eq!(resp[0], 9);
            }
            Err(e) => error!("read: {}", e),
        }

        // Reset state, verify it's 0
        match con.write_read(DEV_ADDR, &[0xC8], &mut resp).await {
            Ok(_) => {
                info!("reset, state={}", resp[0]);
                defmt::assert_eq!(resp[0], 0);
            }
            Err(e) => error!("write_read reset: {}", e),
        }

        Timer::after_millis(100).await;
    }
}
```

### `imports.rs`

```rust
pub use defmt::info;
pub use embassy_executor::Spawner;
pub use embassy_stm32::i2c::{Address, OwnAddresses, SlaveCommandKind};
pub use embassy_stm32::mode::Async;
pub use embassy_stm32::time::Hertz;
pub use embassy_stm32::{bind_interrupts, i2c, peripherals};
pub use embassy_time::Timer;
pub use {defmt_rtt as _, panic_probe as _};
```

---

## Quick Reference

| Goal | Method |
|------|--------|
| Send bytes as master | `write(addr, &data).await` |
| Receive bytes as master | `read(addr, &mut buf).await` |
| Write then read (register access) | `write_read(addr, &cmd, &mut buf).await` |
| Write from multiple buffers | `write_vectored(addr, &[&a, &b]).await` |
| Wait for master command (slave) | `listen().await` |
| Send response to master read (slave) | `respond_to_read(&data).await` |
| Receive data from master write (slave) | `respond_to_write(&mut buf).await` |
| Convert to slave/multimaster | `.into_slave_multimaster(slave_config)` |
| Blocking equivalents | Prefix `blocking_` on any of the above |
