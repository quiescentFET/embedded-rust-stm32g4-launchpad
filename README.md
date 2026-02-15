# Launchpad for Embedded Rust on STM32(G474RE)

A minimal bare-metal Rust launchpad for the STM32G474RE microcontroller.

## Dependencies

- [Rust](https://www.rust-lang.org/) (with the `thumbv7em-none-eabihf` target)
- [probe-rs](https://probe.rs/) (for flashing and running on hardware)
- [ARM GCC Toolchain](https://developer.arm.com/Tools%20and%20Software/GNU%20Toolchain)(brew available [here](https://formulae.brew.sh/cask/gcc-arm-embedded))
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
  Be sure to add cargo binaries to your PATH in .zshrc file so you can use these tools:
   ```sh
   export PATH="$PATH:$HOME/.cargo/bin"
   ```

4. Clone the repository and build:

   ```sh
   git clone <repo-url>
   cd embedded-rust-stm32g4-launchpad
   cd  module_X
   cargo build
   ```

## Running

Connect your STM32G474RE board via a debug probe, then:

```sh
cargo run
```

This will compile the project, flash it to the chip, and run the debugger. The debugger is configured in `.cargo/config.toml` to use `probe-rs run --chip STM32G474RE`.
