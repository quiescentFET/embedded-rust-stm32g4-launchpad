# Launchpad for Embedded Rust on STM32(G474RE)

A minimal async embedded Rust launchpad for the STM32G474RE microcontroller, using the Embassy framework.

## Dependencies

- [Rust](https://www.rust-lang.org/) (with the `thumbv7em-none-eabihf` target)
- [probe-rs](https://probe.rs/) (for flashing and running on hardware)
- An STM32G474RE development board and a compatible debug probe

### Rust Crates

- `cortex-m` - Low-level Cortex-M peripheral access (with `critical-section-single-core`)
- `cortex-m-rt` - Cortex-M runtime startup and interrupt handling
- `embassy-executor` - Async executor for embedded systems
- `embassy-stm32` - Embassy HAL for STM32 (configured for `stm32g474re`)
- `defmt` - Efficient logging framework for embedded systems
- `defmt-rtt` - RTT transport for defmt log output
- `panic-probe` - Panic handler that prints via defmt over RTT

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

   Be sure to add cargo binaries to your PATH in `.zshrc`:

   ```sh
   export PATH="$PATH:$HOME/.cargo/bin"
   ```

4. Clone the repository and navigate to a module:

   ```sh
   git clone <repo-url>
   cd embedded-rust-stm32g4-launchpad
   cd modules/minimal
   cargo build
   ```

## Running

Connect your STM32G474RE board via a debug probe, then:

```sh
cargo run
```

This compiles the project, flashes it to the chip, and starts the probe-rs runner. The runner is configured in `.cargo/config.toml` to use `probe-rs run --chip STM32G474RE`.

defmt log output will appear in your terminal via RTT. The log level is set to `info` by default (configured via `DEFMT_LOG = "info"` in `.cargo/config.toml`).

## Notes

- I recommend installing [cargo-binutils](https://crates.io/crates/cargo-binutils) and checking the built firmware via:
  ```sh
  cargo size -- -A
  ```
  This ensures `.vector_table` points to flash and `.data` points to RAM.
- Cargo binaries may not be on your PATH by default on macOS — add them manually if needed.
- Configure your IDE/editor to use ONLY the target architecture for `rust-analyzer`, otherwise it will show errors irrelevant to the microcontroller. Settings for the Zed editor are provided in this repo.
