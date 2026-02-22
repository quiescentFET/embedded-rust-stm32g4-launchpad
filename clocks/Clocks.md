# STM32G474RE Clock Configuration (embassy-stm32)

## Default Speed

Without any configuration, the chip runs on the **HSI16 internal oscillator at 16 MHz**.

```rust
let p = embassy_stm32::init(Default::default()); // 16 MHz HSI
```

## Configuring Clock Speed

To change the clock speed, build a `Config`, modify `rcc`, then pass it to `init()`:

```rust
let mut config = embassy_stm32::Config::default();
// modify config.rcc here...
let p = embassy_stm32::init(config);
```

> **Important:** The `config` must be passed to `init()`. Creating a `Config` without passing it has no effect.

## Running at 170 MHz (Max Speed)

Uses the internal HSI16 oscillator as the PLL source.

```rust
use embassy_stm32::rcc::*;

let mut config = embassy_stm32::Config::default();
config.rcc.hsi = true;
config.rcc.pll = Some(Pll {
    source: PllSource::HSI,    // 16 MHz HSI input
    prediv: PllPreDiv::DIV4,   // 16 / 4 = 4 MHz VCO input
    mul: PllMul::MUL85,        // 4 * 85 = 340 MHz VCO
    divp: None,
    divq: None,
    divr: Some(PllRDiv::DIV2), // 340 / 2 = 170 MHz SYSCLK
});
config.rcc.sys = Sysclk::PLL1_R;
config.rcc.boost = true; // required above 150 MHz
let p = embassy_stm32::init(config);
```

### PLL Math

```
VCO input  = HSI16 / prediv  = 16 MHz / 4  =   4 MHz
VCO output = VCO input * mul  =  4 MHz * 85 = 340 MHz
SYSCLK     = VCO output / divr = 340 MHz / 2 = 170 MHz
```

## Common Speed Configurations

| Target   | `prediv` | `mul`   | `divr` | `boost` |
|----------|----------|---------|--------|---------|
| 170 MHz  | DIV4     | MUL85   | DIV2   | true    |
| 100 MHz  | DIV4     | MUL50   | DIV2   | false   |
| 64 MHz   | DIV4     | MUL32   | DIV2   | false   |
| 16 MHz   | *(use HSI directly, no PLL needed)* |||

## Rules & Constraints

| Parameter | Constraint |
|-----------|------------|
| VCO input (after `prediv`) | 2.66 MHz – 16 MHz |
| VCO output (after `mul`) | 96 MHz – 344 MHz |
| Max SYSCLK | 170 MHz |
| `boost = true` | Required when SYSCLK > 150 MHz |
| Flash wait states | Set automatically by `embassy_stm32::init()` |

## Clock Sources

### `Sysclk` (system clock selector)
| Variant | Description |
|---------|-------------|
| `Sysclk::HSI` | HSI16 directly (16 MHz, default) |
| `Sysclk::HSE` | External oscillator directly |
| `Sysclk::PLL1_R` | PLL output (required for high speeds) |

### `PllSource` (PLL input)
| Variant | Description |
|---------|-------------|
| `PllSource::HSI` | HSI16 feeds the PLL (no crystal needed) |
| `PllSource::HSE` | HSE feeds the PLL (more accurate, needs crystal) |

## Using an External Crystal (HSE) as PLL Source

```rust
use embassy_stm32::rcc::*;
use embassy_stm32::time::Hertz;

let mut config = embassy_stm32::Config::default();
config.rcc.hse = Some(Hse {
    freq: Hertz(8_000_000), // 8 MHz crystal
    mode: HseMode::Oscillator,
});
config.rcc.pll = Some(Pll {
    source: PllSource::HSE,    // 8 MHz HSE input
    prediv: PllPreDiv::DIV2,   // 8 / 2 = 4 MHz VCO input
    mul: PllMul::MUL85,        // 4 * 85 = 340 MHz VCO
    divp: None,
    divq: None,
    divr: Some(PllRDiv::DIV2), // 340 / 2 = 170 MHz SYSCLK
});
config.rcc.sys = Sysclk::PLL1_R;
config.rcc.boost = true;
let p = embassy_stm32::init(config);
```
