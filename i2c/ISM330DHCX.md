# ISM330DHCX Reference

Low-level reference for communicating with the ISM330DHCX 6-axis IMU over I2C or SPI and reading
accelerometer data.

---

## Table of Contents

1. [Overview](#overview)
2. [Interface Selection](#interface-selection)
3. [I2C Interface](#i2c-interface)
4. [SPI Interface](#spi-interface)
5. [Boot Sequence](#boot-sequence)
6. [Key Registers](#key-registers)
7. [Output Data & Conversion](#output-data--conversion)

---

## Overview

The ISM330DHCX is a 6-axis IMU with a 3D accelerometer and 3D gyroscope on a single die. It
communicates over I2C (up to 400 kHz) or SPI (up to 10 MHz) and produces 16-bit signed output for
each axis.

| Property | Value |
|----------|-------|
| Accel full-scale | ±2 / ±4 / ±8 / ±16 g |
| Accel ODR | 12.5 Hz – 6.66 kHz (plus power-down) |
| Output resolution | 16-bit two's complement |
| Supply voltage | 1.71 V – 3.6 V |
| Operating temp | −40 °C to +105 °C |
| Package | 14-pin LGA (2.5 × 3.0 × 0.83 mm) |

---

## Interface Selection

The CS pin determines the active interface at power-on:

| CS pin state | Interface active |
|---|---|
| HIGH (tied to VDD) | I2C |
| LOW (pulled low / driven) | SPI |

The **SA0/SDO** pin has a dual role:
- In **I2C mode**: selects the least significant bit of the slave address (see below)
- In **SPI mode**: serves as the MISO data output line

---

## I2C Interface

### Addresses

The 7-bit I2C address is `110101x` where the LSb is set by the SA0/SDO pin.

| SA0 wiring | 7-bit address | 8-bit write | 8-bit read |
|---|---|---|---|
| SDO → GND | `0x6A` | `0xD4` | `0xD5` |
| SDO → VDD | `0x6B` | `0xD6` | `0xD7` |

> **Embassy note**: `I2c::write_read` and related methods take a **7-bit address**.
> Use `0x6A` or `0x6B` — not `0xD4`/`0xD6`.
> The existing code has `ACCEL_ADDR = 0xD6` which is the 8-bit write address and is incorrect
> for Embassy. It should be `0x6B`.

### Protocol

- **Max clock**: 400 kHz (Fast Mode)
- **Auto-increment**: enabled by default via `IF_INC` bit in CTRL3_C (`0x12`). Allows burst reads
  of consecutive registers in a single transaction.
- **Block data update (BDU)**: enable via `BDU` bit in CTRL3_C to prevent the high and low bytes
  of an output register from being updated between two consecutive reads.

### Read/Write pattern

```
Write register:
  START | addr_7bit W | reg_addr | data_byte | STOP

Read register:
  START | addr_7bit W | reg_addr | RESTART | addr_7bit R | data_byte | STOP

Burst read (e.g. all 6 accel bytes at once):
  START | addr_7bit W | 0x28 | RESTART | addr_7bit R | x_l | x_h | y_l | y_h | z_l | z_h | STOP
```

---

## SPI Interface

### Electrical

- 4-wire: CS (active low), SCK, MOSI (SDI), MISO (SDO)
- **SPI Mode 3** — CPOL=1, CPHA=1 (clock idle HIGH; data sampled on rising edge)
- **Max clock**: 10 MHz
- **Bit order**: MSb first

### Address byte format

The first byte sent in any SPI transaction encodes the operation and register address:

```
Bit 7:    R/W  (0 = write, 1 = read)
Bits 6-0: register address (7 bits)
```

Examples:

| Operation | Register | First byte sent |
|---|---|---|
| Write CTRL1_XL | 0x10 | `0x10` |
| Read WHO_AM_I  | 0x0F | `0x8F` |
| Read 6 accel bytes starting at 0x28 | 0x28 | `0xA8` |

### Burst read

Hold CS low and clock out additional bytes after the address byte. The register address
auto-increments for each byte (requires `IF_INC=1` in CTRL3_C, which is the default).

---

## Boot Sequence

After power-on (or software reset):

1. **Wait** ~10 ms for POR to complete
2. **Verify comms**: read WHO_AM_I (`0x0F`) → expect `0x6B`
3. **Configure device control**: write `0x44` to CTRL3_C (`0x12`)
   - `BDU = 1` (bit 6): block data update
   - `IF_INC = 1` (bit 2): address auto-increment for burst reads
4. **Enable accelerometer**: write CTRL1_XL (`0x10`) with desired ODR and full-scale range
5. **Wait for data**: poll STATUS_REG (`0x1E`) bit 0 (`XLDA`) until it reads `1`
6. **Read data**: burst-read 6 bytes starting at `0x28`

### Example CTRL1_XL value

To configure 104 Hz ODR, ±4g full-scale:

```
ODR_XL = 0b0100  (104 Hz)
FS_XL  = 0b10   (±4g)
CTRL1_XL = 0b0100_1000 = 0x48
```

---

## Key Registers

| Register | Address | R/W | Reset | Description |
|---|---|---|---|---|
| WHO_AM_I | `0x0F` | R | `0x6B` | Device ID — always reads `0x6B` |
| CTRL1_XL | `0x10` | R/W | `0x00` | Accelerometer ODR, full-scale, LPF |
| CTRL2_G  | `0x11` | R/W | `0x00` | Gyroscope ODR and full-scale |
| CTRL3_C  | `0x12` | R/W | `0x04` | BDU, SW_RESET, IF_INC, BOOT |
| STATUS_REG | `0x1E` | R | — | Data-ready flags |
| OUTX_L_A | `0x28` | R | — | Accel X low byte |
| OUTX_H_A | `0x29` | R | — | Accel X high byte |
| OUTY_L_A | `0x2A` | R | — | Accel Y low byte |
| OUTY_H_A | `0x2B` | R | — | Accel Y high byte |
| OUTZ_L_A | `0x2C` | R | — | Accel Z low byte |
| OUTZ_H_A | `0x2D` | R | — | Accel Z high byte |

### CTRL1_XL (`0x10`)

```
[7:4] ODR_XL  — output data rate
[3:2] FS_XL   — full-scale selection
[1]   LPF2_XL_EN — second low-pass filter stage
[0]   reserved
```

**ODR_XL values:**

| Bits | ODR |
|---|---|
| `0000` | Power-down (off) |
| `0001` | 12.5 Hz |
| `0010` | 26 Hz |
| `0011` | 52 Hz |
| `0100` | 104 Hz |
| `0101` | 208 Hz |
| `0110` | 416 Hz |
| `0111` | 833 Hz |
| `1000` | 1.66 kHz |
| `1001` | 3.33 kHz |
| `1010` | 6.66 kHz |

**FS_XL values:**

| Bits | Full-scale |
|---|---|
| `00` | ±2 g |
| `10` | ±4 g |
| `11` | ±8 g |
| `01` | ±16 g |

### CTRL3_C (`0x12`)

```
[7] BOOT      — reboot memory content (self-clears)
[6] BDU       — block data update: 1 = output regs not updated until both bytes read
[5] H_LACTIVE — interrupt active level
[4] PP_OD     — push-pull / open-drain on INT pins
[3] SIM       — SPI serial interface mode (0 = 4-wire)
[2] IF_INC    — register address auto-increment: 1 = enabled (default)
[1] reserved
[0] SW_RESET  — software reset (self-clears)
```

Recommended value after boot: `0x44` (`BDU=1`, `IF_INC=1`).

### STATUS_REG (`0x1E`)

```
[2] TDA  — temperature data available
[1] GDA  — gyroscope data available
[0] XLDA — accelerometer data available  ← check this before reading accel output regs
```

---

## Output Data & Conversion

### Raw format

Each axis is a **16-bit signed integer (two's complement)**, stored little-endian (low byte at
lower address):

```rust
let x_raw: i16 = i16::from_le_bytes([outx_l, outx_h]);
let y_raw: i16 = i16::from_le_bytes([outy_l, outy_h]);
let z_raw: i16 = i16::from_le_bytes([outz_l, outz_h]);
```

### Sensitivity table

| FS_XL | Full-scale | Sensitivity |
|---|---|---|
| `0b00` | ±2 g  | 0.061 mg/LSB |
| `0b10` | ±4 g  | 0.122 mg/LSB |
| `0b11` | ±8 g  | 0.244 mg/LSB |
| `0b01` | ±16 g | 0.488 mg/LSB |

### Conversion

```rust
// result in milli-g
let sensitivity: f32 = 0.061; // for ±2g; change per table above
let x_mg = x_raw as f32 * sensitivity;

// result in g
let x_g = x_mg / 1000.0;
```

At rest flat on a surface, one axis should read ~1000 mg (1 g) and the other two ~0 mg.
