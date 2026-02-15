# minimal_G474

A minimal bare-metal Rust project for the STM32G474RE microcontroller.

## Dependencies

- [Rust](https://www.rust-lang.org/) (with the `thumbv7em-none-eabihf` target)
- [probe-rs](https://probe.rs/) (for flashing and running on hardware)
- An STM32G474RE development board and a compatible debug probe

### Rust Crates

- `cortex-m-rt` - Cortex-M runtime startup and interrupt handling
- `panic-halt` - Halt-on-panic handler for `no_std` environments

## Installation

1. Install Rust via [rustup](https://rustup.rs/):

   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Add the ARM Cortex-M4F compilation target:

   ```sh
   rustup target add thumbv7em-none-eabihf
   ```

3. Install probe-rs:

   ```sh
   cargo install probe-rs-tools
   ```

4. Clone the repository and build:

   ```sh
   git clone <repo-url>
   cd minimal_G474
   cargo build
   ```

## Running

Connect your STM32G474RE board via a debug probe, then:

```sh
cargo run
```

This will compile the project, flash it to the chip, and begin execution. The runner is configured in `.cargo/config.toml` to use `probe-rs run --chip STM32G474RE`.
