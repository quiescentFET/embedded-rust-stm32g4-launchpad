# ISM330DHCX Complete Register Map

Full register reference for the ISM330DHCX 6-axis IMU (3D accelerometer + 3D gyroscope).
Compiled from the ST datasheet (DS12441 Rev 8), ST PID driver (`ism330dhcx_reg.h`), and the
ST application note AN5398.

**R/W key:** R = read-only, W = write-only, R/W = read-write, r = reserved (do not write)

---

## Table of Contents

1. [Register Map Summary](#register-map-summary)
2. [User Interface Registers (Page 0)](#user-interface-registers-page-0)
   - [FUNC_CFG_ACCESS (0x01)](#func_cfg_access-0x01)
   - [PIN_CTRL (0x02)](#pin_ctrl-0x02)
   - [FIFO_CTRL1 (0x07)](#fifo_ctrl1-0x07)
   - [FIFO_CTRL2 (0x08)](#fifo_ctrl2-0x08)
   - [FIFO_CTRL3 (0x09)](#fifo_ctrl3-0x09)
   - [FIFO_CTRL4 (0x0A)](#fifo_ctrl4-0x0a)
   - [COUNTER_BDR_REG1 (0x0B)](#counter_bdr_reg1-0x0b)
   - [COUNTER_BDR_REG2 (0x0C)](#counter_bdr_reg2-0x0c)
   - [INT1_CTRL (0x0D)](#int1_ctrl-0x0d)
   - [INT2_CTRL (0x0E)](#int2_ctrl-0x0e)
   - [WHO_AM_I (0x0F)](#who_am_i-0x0f)
   - [CTRL1_XL (0x10)](#ctrl1_xl-0x10)
   - [CTRL2_G (0x11)](#ctrl2_g-0x11)
   - [CTRL3_C (0x12)](#ctrl3_c-0x12)
   - [CTRL4_C (0x13)](#ctrl4_c-0x13)
   - [CTRL5_C (0x14)](#ctrl5_c-0x14)
   - [CTRL6_C (0x15)](#ctrl6_c-0x15)
   - [CTRL7_G (0x16)](#ctrl7_g-0x16)
   - [CTRL8_XL (0x17)](#ctrl8_xl-0x17)
   - [CTRL9_XL (0x18)](#ctrl9_xl-0x18)
   - [CTRL10_C (0x19)](#ctrl10_c-0x19)
   - [ALL_INT_SRC (0x1A)](#all_int_src-0x1a)
   - [WAKE_UP_SRC (0x1B)](#wake_up_src-0x1b)
   - [TAP_SRC (0x1C)](#tap_src-0x1c)
   - [D6D_SRC (0x1D)](#d6d_src-0x1d)
   - [STATUS_REG (0x1E)](#status_reg-0x1e)
   - [OUT_TEMP_L (0x20)](#out_temp_l-0x20)
   - [OUT_TEMP_H (0x21)](#out_temp_h-0x21)
   - [OUTX_L_G (0x22)](#outx_l_g-0x22)
   - [OUTX_H_G (0x23)](#outx_h_g-0x23)
   - [OUTY_L_G (0x24)](#outy_l_g-0x24)
   - [OUTY_H_G (0x25)](#outy_h_g-0x25)
   - [OUTZ_L_G (0x26)](#outz_l_g-0x26)
   - [OUTZ_H_G (0x27)](#outz_h_g-0x27)
   - [OUTX_L_A (0x28)](#outx_l_a-0x28)
   - [OUTX_H_A (0x29)](#outx_h_a-0x29)
   - [OUTY_L_A (0x2A)](#outy_l_a-0x2a)
   - [OUTY_H_A (0x2B)](#outy_h_a-0x2b)
   - [OUTZ_L_A (0x2C)](#outz_l_a-0x2c)
   - [OUTZ_H_A (0x2D)](#outz_h_a-0x2d)
   - [EMB_FUNC_STATUS_MAINPAGE (0x35)](#emb_func_status_mainpage-0x35)
   - [FSM_STATUS_A_MAINPAGE (0x36)](#fsm_status_a_mainpage-0x36)
   - [FSM_STATUS_B_MAINPAGE (0x37)](#fsm_status_b_mainpage-0x37)
   - [MLC_STATUS_MAINPAGE (0x38)](#mlc_status_mainpage-0x38)
   - [STATUS_MASTER_MAINPAGE (0x39)](#status_master_mainpage-0x39)
   - [FIFO_STATUS1 (0x3A)](#fifo_status1-0x3a)
   - [FIFO_STATUS2 (0x3B)](#fifo_status2-0x3b)
   - [TIMESTAMP0 (0x40)](#timestamp0-0x40)
   - [TIMESTAMP1 (0x41)](#timestamp1-0x41)
   - [TIMESTAMP2 (0x42)](#timestamp2-0x42)
   - [TIMESTAMP3 (0x43)](#timestamp3-0x43)
   - [TAP_CFG0 (0x56)](#tap_cfg0-0x56)
   - [TAP_CFG1 (0x57)](#tap_cfg1-0x57)
   - [TAP_CFG2 (0x58)](#tap_cfg2-0x58)
   - [TAP_THS_6D (0x59)](#tap_ths_6d-0x59)
   - [INT_DUR2 (0x5A)](#int_dur2-0x5a)
   - [WAKE_UP_THS (0x5B)](#wake_up_ths-0x5b)
   - [WAKE_UP_DUR (0x5C)](#wake_up_dur-0x5c)
   - [FREE_FALL (0x5D)](#free_fall-0x5d)
   - [MD1_CFG (0x5E)](#md1_cfg-0x5e)
   - [MD2_CFG (0x5F)](#md2_cfg-0x5f)
   - [INTERNAL_FREQ_FINE (0x63)](#internal_freq_fine-0x63)
   - [INT_OIS (0x6F)](#int_ois-0x6f)
   - [CTRL1_OIS (0x70)](#ctrl1_ois-0x70)
   - [CTRL2_OIS (0x71)](#ctrl2_ois-0x71)
   - [CTRL3_OIS (0x72)](#ctrl3_ois-0x72)
   - [X_OFS_USR (0x73)](#x_ofs_usr-0x73)
   - [Y_OFS_USR (0x74)](#y_ofs_usr-0x74)
   - [Z_OFS_USR (0x75)](#z_ofs_usr-0x75)
   - [FIFO_DATA_OUT_TAG (0x78)](#fifo_data_out_tag-0x78)
   - [FIFO_DATA_OUT_X_L (0x79)](#fifo_data_out_x_l-0x79)
   - [FIFO_DATA_OUT_X_H (0x7A)](#fifo_data_out_x_h-0x7a)
   - [FIFO_DATA_OUT_Y_L (0x7B)](#fifo_data_out_y_l-0x7b)
   - [FIFO_DATA_OUT_Y_H (0x7C)](#fifo_data_out_y_h-0x7c)
   - [FIFO_DATA_OUT_Z_L (0x7D)](#fifo_data_out_z_l-0x7d)
   - [FIFO_DATA_OUT_Z_H (0x7E)](#fifo_data_out_z_h-0x7e)
3. [Embedded Functions Registers (Page 1)](#embedded-functions-registers-page-1)
4. [Sensor Hub Registers (Page 2)](#sensor-hub-registers-page-2)

---

## Register Map Summary

All registers on the main user interface page (Page 0, accessed when `FUNC_CFG_ACCESS[1:0] = 00`).

| Address | Register Name | R/W | Reset | Description |
|---------|---------------|-----|-------|-------------|
| `0x01` | FUNC_CFG_ACCESS | R/W | `0x00` | Enable access to embedded functions and sensor hub registers |
| `0x02` | PIN_CTRL | R/W | `0x00` | SDO/OIS pin pull-up control |
| `0x07` | FIFO_CTRL1 | R/W | `0x00` | FIFO watermark threshold, bits [7:0] |
| `0x08` | FIFO_CTRL2 | R/W | `0x00` | FIFO watermark threshold MSb, compression, ODR change |
| `0x09` | FIFO_CTRL3 | R/W | `0x00` | Batch data rate for gyroscope and accelerometer |
| `0x0A` | FIFO_CTRL4 | R/W | `0x00` | FIFO mode, temperature batch rate, timestamp decimation |
| `0x0B` | COUNTER_BDR_REG1 | R/W | `0x00` | BDR counter threshold [10:8], trigger, reset, pulsed DRDY |
| `0x0C` | COUNTER_BDR_REG2 | R/W | `0x00` | BDR counter threshold [7:0] |
| `0x0D` | INT1_CTRL | R/W | `0x00` | INT1 pin interrupt routing |
| `0x0E` | INT2_CTRL | R/W | `0x00` | INT2 pin interrupt routing |
| `0x0F` | WHO_AM_I | R | `0x6B` | Device identification register |
| `0x10` | CTRL1_XL | R/W | `0x00` | Accelerometer ODR, full-scale, LPF2 enable |
| `0x11` | CTRL2_G | R/W | `0x00` | Gyroscope ODR, full-scale |
| `0x12` | CTRL3_C | R/W | `0x04` | SW reset, auto-increment, SPI mode, interrupt polarity, BDU, boot |
| `0x13` | CTRL4_C | R/W | `0x00` | Gyro LPF1 select, I2C disable, DRDY mask, INT2-on-INT1, gyro sleep |
| `0x14` | CTRL5_C | R/W | `0x00` | Accel/gyro self-test, rounding |
| `0x15` | CTRL6_C | R/W | `0x00` | Gyro LPF1 bandwidth, user offset weight, accel HM mode, DEN trigger |
| `0x16` | CTRL7_G | R/W | `0x00` | Gyro HM mode, HP filter enable/cutoff, OIS control |
| `0x17` | CTRL8_XL | R/W | `0x00` | Accel HP/LPF2 filter configuration |
| `0x18` | CTRL9_XL | R/W | `0x00` | DEN configuration, device configuration |
| `0x19` | CTRL10_C | R/W | `0x00` | Timestamp enable |
| `0x1A` | ALL_INT_SRC | R | `0x00` | Latched interrupt source flags |
| `0x1B` | WAKE_UP_SRC | R | `0x00` | Wake-up, free-fall, sleep event flags |
| `0x1C` | TAP_SRC | R | `0x00` | Tap and double-tap event flags |
| `0x1D` | D6D_SRC | R | `0x00` | 6D/4D orientation detection flags |
| `0x1E` | STATUS_REG | R | `0x00` | Accelerometer, gyroscope, temperature data-ready flags |
| `0x20` | OUT_TEMP_L | R | `0x00` | Temperature output, low byte |
| `0x21` | OUT_TEMP_H | R | `0x00` | Temperature output, high byte |
| `0x22` | OUTX_L_G | R | `0x00` | Gyroscope X-axis output, low byte |
| `0x23` | OUTX_H_G | R | `0x00` | Gyroscope X-axis output, high byte |
| `0x24` | OUTY_L_G | R | `0x00` | Gyroscope Y-axis output, low byte |
| `0x25` | OUTY_H_G | R | `0x00` | Gyroscope Y-axis output, high byte |
| `0x26` | OUTZ_L_G | R | `0x00` | Gyroscope Z-axis output, low byte |
| `0x27` | OUTZ_H_G | R | `0x00` | Gyroscope Z-axis output, high byte |
| `0x28` | OUTX_L_A | R | `0x00` | Accelerometer X-axis output, low byte |
| `0x29` | OUTX_H_A | R | `0x00` | Accelerometer X-axis output, high byte |
| `0x2A` | OUTY_L_A | R | `0x00` | Accelerometer Y-axis output, low byte |
| `0x2B` | OUTY_H_A | R | `0x00` | Accelerometer Y-axis output, high byte |
| `0x2C` | OUTZ_L_A | R | `0x00` | Accelerometer Z-axis output, low byte |
| `0x2D` | OUTZ_H_A | R | `0x00` | Accelerometer Z-axis output, high byte |
| `0x35` | EMB_FUNC_STATUS_MAINPAGE | R | `0x00` | Embedded functions interrupt status (pedometer, tilt, sig. motion) |
| `0x36` | FSM_STATUS_A_MAINPAGE | R | `0x00` | FSM1–FSM8 interrupt status |
| `0x37` | FSM_STATUS_B_MAINPAGE | R | `0x00` | FSM9–FSM16 interrupt status |
| `0x38` | MLC_STATUS_MAINPAGE | R | `0x00` | Machine Learning Core interrupt status |
| `0x39` | STATUS_MASTER_MAINPAGE | R | `0x00` | Sensor hub master status flags |
| `0x3A` | FIFO_STATUS1 | R | `0x00` | FIFO fill level, bits [7:0] |
| `0x3B` | FIFO_STATUS2 | R | `0x00` | FIFO fill level [9:8], overflow, watermark, full flags |
| `0x40` | TIMESTAMP0 | R | `0x00` | Timestamp counter output, byte 0 (LSB) |
| `0x41` | TIMESTAMP1 | R | `0x00` | Timestamp counter output, byte 1 |
| `0x42` | TIMESTAMP2 | R | `0x00` | Timestamp counter output, byte 2 |
| `0x43` | TIMESTAMP3 | R | `0x00` | Timestamp counter output, byte 3 (MSB) |
| `0x56` | TAP_CFG0 | R/W | `0x00` | Tap detection axis enable, latched interrupt, activity/inactivity |
| `0x57` | TAP_CFG1 | R/W | `0x00` | Tap X-axis threshold, tap priority axis |
| `0x58` | TAP_CFG2 | R/W | `0x00` | Tap Y-axis threshold, inactivity enable, interrupts global enable |
| `0x59` | TAP_THS_6D | R/W | `0x00` | Tap Z-axis threshold, 6D threshold, D4D mode enable |
| `0x5A` | INT_DUR2 | R/W | `0x00` | Double-tap: shock, quiet, and gap duration |
| `0x5B` | WAKE_UP_THS | R/W | `0x00` | Wake-up threshold, single/double tap select, offset on wake-up |
| `0x5C` | WAKE_UP_DUR | R/W | `0x00` | Wake-up duration, sleep duration, free-fall duration MSb |
| `0x5D` | FREE_FALL | R/W | `0x00` | Free-fall threshold and duration |
| `0x5E` | MD1_CFG | R/W | `0x00` | INT1 function routing (tap, wake-up, free-fall, 6D, embedded functions) |
| `0x5F` | MD2_CFG | R/W | `0x00` | INT2 function routing (tap, wake-up, free-fall, 6D, embedded functions) |
| `0x63` | INTERNAL_FREQ_FINE | R | `0x00` | Internal frequency correction value (read-only calibration) |
| `0x6F` | INT_OIS | R/W | `0x00` | OIS interrupt enable on INT2, OIS DEN polarity, accel OIS self-test |
| `0x70` | CTRL1_OIS | R/W | `0x00` | OIS chain enable, gyro OIS full-scale, SPI2 mode, OIS chain level |
| `0x71` | CTRL2_OIS | R/W | `0x00` | OIS gyro HP filter enable, bandwidth, cutoff frequency |
| `0x72` | CTRL3_OIS | R/W | `0x00` | OIS accel full-scale, LPF filter, gyro self-test |
| `0x73` | X_OFS_USR | R/W | `0x00` | User offset for accelerometer X axis |
| `0x74` | Y_OFS_USR | R/W | `0x00` | User offset for accelerometer Y axis |
| `0x75` | Z_OFS_USR | R/W | `0x00` | User offset for accelerometer Z axis |
| `0x78` | FIFO_DATA_OUT_TAG | R | `0x00` | FIFO data tag: identifies sensor source and parity |
| `0x79` | FIFO_DATA_OUT_X_L | R | `0x00` | FIFO word first axis low byte |
| `0x7A` | FIFO_DATA_OUT_X_H | R | `0x00` | FIFO word first axis high byte |
| `0x7B` | FIFO_DATA_OUT_Y_L | R | `0x00` | FIFO word second axis low byte |
| `0x7C` | FIFO_DATA_OUT_Y_H | R | `0x00` | FIFO word second axis high byte |
| `0x7D` | FIFO_DATA_OUT_Z_L | R | `0x00` | FIFO word third axis low byte |
| `0x7E` | FIFO_DATA_OUT_Z_H | R | `0x00` | FIFO word third axis high byte |

---

## User Interface Registers (Page 0)

### FUNC_CFG_ACCESS (0x01)

R/W | Reset: `0x00`

Controls access to the embedded functions register bank and the sensor hub register bank. Write
this register before accessing registers on pages 1 or 2.

| Bit | Name | Description |
|-----|------|-------------|
| [7] | FUNC_CFG_ACCESS | `1` = enable access to embedded functions registers (page 1) |
| [6] | SHUB_REG_ACCESS | `1` = enable access to sensor hub registers (page 2) |
| [5:0] | — | Reserved, must be 0 |

> After setting either access bit, restore `0x00` before normal operation.

---

### PIN_CTRL (0x02)

R/W | Reset: `0x00`

Controls pull-up on the SDO/SA0 and OIS pins.

| Bit | Name | Description |
|-----|------|-------------|
| [7] | OIS_PU_DIS | `1` = disable pull-up on SDI/SDO/SCLK OIS pins |
| [6] | SDO_PU_EN | `1` = enable pull-up on SDO/SA0 pin |
| [5:0] | — | Reserved, must be 0 |

---

### FIFO_CTRL1 (0x07)

R/W | Reset: `0x00`

FIFO watermark threshold, bits [7:0]. The watermark level is a 9-bit value split across
FIFO_CTRL1 and bit 0 of FIFO_CTRL2.

| Bit | Name | Description |
|-----|------|-------------|
| [7:0] | WTM[7:0] | Watermark threshold low byte — FIFO depth in words (max 512 words) |

---

### FIFO_CTRL2 (0x08)

R/W | Reset: `0x00`

FIFO watermark threshold MSb, compression, and ODR change logging control.

| Bit | Name | Description |
|-----|------|-------------|
| [7] | STOP_ON_WTM | `1` = FIFO stops filling when watermark is reached |
| [6] | FIFO_COMPR_RT_EN | `1` = enable FIFO compression at runtime |
| [5] | — | Reserved |
| [4] | ODRCHG_EN | `1` = enable logging of ODR changes in FIFO |
| [3] | — | Reserved |
| [2:1] | UNCOPTR_RATE[1:0] | Uncompressed data rate: `00`=no compression, `01`=8x, `10`=16x, `11`=32x |
| [0] | WTM[8] | Watermark threshold MSb (bit 8) |

---

### FIFO_CTRL3 (0x09)

R/W | Reset: `0x00`

Batch data rate (BDR) selection for accelerometer and gyroscope data written into the FIFO.

| Bit | Name | Description |
|-----|------|-------------|
| [7:4] | BDR_GY[3:0] | Gyroscope batch data rate (see table below) |
| [3:0] | BDR_XL[3:0] | Accelerometer batch data rate (see table below) |

**BDR values (same encoding for both BDR_GY and BDR_XL):**

| Code | Rate |
|------|------|
| `0000` | Not batched (FIFO off for that sensor) |
| `0001` | 12.5 Hz |
| `0010` | 26 Hz |
| `0011` | 52 Hz |
| `0100` | 104 Hz |
| `0101` | 208 Hz |
| `0110` | 416 Hz |
| `0111` | 833 Hz |
| `1000` | 1666 Hz |
| `1001` | 3333 Hz |
| `1010` | 6667 Hz |
| `1011` | 6.5 Hz (XL only, low-power) |

---

### FIFO_CTRL4 (0x0A)

R/W | Reset: `0x00`

FIFO operating mode, temperature batch rate, and timestamp decimation.

| Bit | Name | Description |
|-----|------|-------------|
| [7:6] | DEC_TS_BATCH[1:0] | Timestamp decimation: `00`=not batched, `01`=dec/1, `10`=dec/8, `11`=dec/32 |
| [5:4] | ODR_T_BATCH[1:0] | Temperature batch rate: `00`=not batched, `01`=1.875 Hz, `10`=15 Hz, `11`=60 Hz |
| [3] | — | Reserved |
| [2:0] | FIFO_MODE[2:0] | FIFO mode (see table below) |

**FIFO_MODE values:**

| Code | Mode |
|------|------|
| `000` | Bypass — FIFO disabled |
| `001` | FIFO mode — stops collecting when full |
| `010` | Reserved |
| `011` | Continuous-to-FIFO — FIFO until trigger, then FIFO mode |
| `100` | Bypass-to-Continuous — bypass until trigger, then continuous |
| `101` | Reserved |
| `110` | Continuous with EIS — continuous with EIS gyro data |
| `111` | Continuous — newest data overwrites oldest when full |

---

### COUNTER_BDR_REG1 (0x0B)

R/W | Reset: `0x00`

BDR counter upper threshold bits and control flags.

| Bit | Name | Description |
|-----|------|-------------|
| [7] | DATAREADY_PULSED | `0`=DRDY latched, `1`=DRDY pulsed (75 µs pulse width) |
| [6] | RST_COUNTER_BDR | Write `1` to reset the BDR counter |
| [5] | TRIG_COUNTER_BDR | `0`=XL triggers counter, `1`=gyro triggers counter |
| [4:3] | — | Reserved |
| [2:0] | CNT_BDR_TH[10:8] | BDR counter threshold, upper 3 bits |

---

### COUNTER_BDR_REG2 (0x0C)

R/W | Reset: `0x00`

BDR counter threshold lower byte.

| Bit | Name | Description |
|-----|------|-------------|
| [7:0] | CNT_BDR_TH[7:0] | BDR counter threshold, lower 8 bits |

---

### INT1_CTRL (0x0D)

R/W | Reset: `0x00`

Routes hardware interrupt sources to the INT1 pin.

| Bit | Name | Description |
|-----|------|-------------|
| [7] | DEN_DRDY_FLAG | `1` = DEN data-ready signal routed to INT1 |
| [6] | INT1_CNT_BDR | `1` = BDR counter threshold reached → INT1 |
| [5] | INT1_FIFO_FULL | `1` = FIFO full → INT1 |
| [4] | INT1_FIFO_OVR | `1` = FIFO overrun → INT1 |
| [3] | INT1_FIFO_TH | `1` = FIFO watermark threshold reached → INT1 |
| [2] | INT1_BOOT | `1` = boot status → INT1 |
| [1] | INT1_DRDY_G | `1` = gyroscope data-ready → INT1 |
| [0] | INT1_DRDY_XL | `1` = accelerometer data-ready → INT1 |

---

### INT2_CTRL (0x0E)

R/W | Reset: `0x00`

Routes hardware interrupt sources to the INT2 pin.

| Bit | Name | Description |
|-----|------|-------------|
| [7] | — | Reserved |
| [6] | INT2_CNT_BDR | `1` = BDR counter threshold reached → INT2 |
| [5] | INT2_FIFO_FULL | `1` = FIFO full → INT2 |
| [4] | INT2_FIFO_OVR | `1` = FIFO overrun → INT2 |
| [3] | INT2_FIFO_TH | `1` = FIFO watermark threshold reached → INT2 |
| [2] | INT2_DRDY_TEMP | `1` = temperature data-ready → INT2 |
| [1] | INT2_DRDY_G | `1` = gyroscope data-ready → INT2 |
| [0] | INT2_DRDY_XL | `1` = accelerometer data-ready → INT2 |

---

### WHO_AM_I (0x0F)

R | Reset: `0x6B` (fixed)

Device identification. Always reads `0x6B`. Use this as a communication sanity check after
power-on.

| Bit | Name | Description |
|-----|------|-------------|
| [7:0] | WHO_AM_I | Fixed value `0x6B` |

---

### CTRL1_XL (0x10)

R/W | Reset: `0x00`

Accelerometer output data rate, full-scale selection, and LPF2 enable.

| Bit | Name | Description |
|-----|------|-------------|
| [7:4] | ODR_XL[3:0] | Accelerometer output data rate and power mode |
| [3:2] | FS_XL[1:0] | Full-scale selection |
| [1] | LPF2_XL_EN | `1` = second low-pass filter (LPF2) stage enabled |
| [0] | — | Reserved |

**ODR_XL values:**

| Bits | ODR | Notes |
|------|-----|-------|
| `0000` | Power-down | Off |
| `0001` | 12.5 Hz | High-performance or low-power |
| `0010` | 26 Hz | High-performance or low-power |
| `0011` | 52 Hz | High-performance or low-power |
| `0100` | 104 Hz | High-performance or normal |
| `0101` | 208 Hz | High-performance or normal |
| `0110` | 416 Hz | High-performance only |
| `0111` | 833 Hz | High-performance only |
| `1000` | 1666 Hz | High-performance only |
| `1001` | 3333 Hz | High-performance only |
| `1010` | 6667 Hz | High-performance only |
| `1011` | 1.6 Hz | Low-power only (XL_HM_MODE=1) |

**FS_XL values:**

| Bits | Full-scale | Sensitivity |
|------|-----------|-------------|
| `00` | ±2 g | 0.061 mg/LSB |
| `10` | ±4 g | 0.122 mg/LSB |
| `11` | ±8 g | 0.244 mg/LSB |
| `01` | ±16 g | 0.488 mg/LSB |

---

### CTRL2_G (0x11)

R/W | Reset: `0x00`

Gyroscope output data rate and full-scale selection.

| Bit | Name | Description |
|-----|------|-------------|
| [7:4] | ODR_G[3:0] | Gyroscope output data rate |
| [3] | FS_4000 | `1` = 4000 dps full-scale (overrides FS_G) |
| [2:1] | FS_G[1:0] | Gyroscope full-scale selection (when FS_4000=0) |
| [0] | FS_125 | `1` = 125 dps full-scale (overrides FS_G) |

**ODR_G values** (same encoding as ODR_XL above, except no 1.6 Hz mode):

| Bits | ODR |
|------|-----|
| `0000` | Power-down |
| `0001` | 12.5 Hz |
| `0010` | 26 Hz |
| `0011` | 52 Hz |
| `0100` | 104 Hz |
| `0101` | 208 Hz |
| `0110` | 416 Hz |
| `0111` | 833 Hz |
| `1000` | 1666 Hz |
| `1001` | 3333 Hz |
| `1010` | 6667 Hz |

**FS_G values (FS_4000=0, FS_125=0):**

| FS_G[1:0] | Full-scale | Sensitivity |
|-----------|-----------|-------------|
| `00` | ±250 dps | 8.75 mdps/LSB |
| `01` | ±500 dps | 17.50 mdps/LSB |
| `10` | ±1000 dps | 35.0 mdps/LSB |
| `11` | ±2000 dps | 70.0 mdps/LSB |

- `FS_125=1`: ±125 dps, 4.375 mdps/LSB
- `FS_4000=1`: ±4000 dps, 140.0 mdps/LSB

---

### CTRL3_C (0x12)

R/W | Reset: `0x04`

General device control. Reset value `0x04` means `IF_INC=1` (register auto-increment ON) by default.

| Bit | Name | Description |
|-----|------|-------------|
| [7] | BOOT | `1` = reboot memory content; self-clears when done |
| [6] | BDU | `1` = block data update: output registers not updated until both MSb and LSb are read |
| [5] | H_LACTIVE | `0` = interrupt active high (default); `1` = interrupt active low |
| [4] | PP_OD | `0` = push-pull (default); `1` = open-drain on INT1 and INT2 |
| [3] | SIM | `0` = SPI 4-wire (default); `1` = SPI 3-wire |
| [2] | IF_INC | `1` = register address auto-increments during multi-byte access (default, set at reset) |
| [1] | — | Reserved |
| [0] | SW_RESET | `1` = software reset; self-clears. Returns all registers to default. Wait ~50 µs after. |

Recommended initialization value: `0x44` (`BDU=1`, `IF_INC=1`).

---

### CTRL4_C (0x13)

R/W | Reset: `0x00`

Interrupt configuration, I2C disable, gyroscope low-power sleep.

| Bit | Name | Description |
|-----|------|-------------|
| [7] | — | Reserved |
| [6] | SLEEP_G | `1` = enable gyroscope sleep mode |
| [5] | INT2_ON_INT1 | `1` = all INT2 signals mirrored on INT1 |
| [4] | — | Reserved |
| [3] | DRDY_MASK | `1` = data-ready masked until filter settling time elapses |
| [2] | I2C_DISABLE | `1` = I2C interface disabled (SPI only) |
| [1] | LPF1_SEL_G | `1` = gyroscope LPF1 output selected for output; `0` = LPF2 |
| [0] | — | Reserved |

---

### CTRL5_C (0x14)

R/W | Reset: `0x00`

Self-test enable for accelerometer and gyroscope, and output data rounding.

| Bit | Name | Description |
|-----|------|-------------|
| [7] | — | Reserved |
| [6:5] | ROUNDING[1:0] | Rounding of output registers: `00`=none, `01`=XL only, `10`=gyro only, `11`=both |
| [4] | — | Reserved |
| [3:2] | ST_G[1:0] | Gyroscope self-test: `00`=off, `01`=positive, `10`=negative, `11`=not allowed |
| [1:0] | ST_XL[1:0] | Accelerometer self-test: `00`=off, `01`=positive, `10`=negative, `11`=not allowed |

---

### CTRL6_C (0x15)

R/W | Reset: `0x00`

DEN (data enable) trigger mode, accelerometer high-performance mode, user offset weight,
and gyroscope LPF1 bandwidth.

| Bit | Name | Description |
|-----|------|-------------|
| [7:5] | DEN_MODE[2:0] | DEN data edge-sensitive/level-sensitive trigger mode |
| [4] | XL_HM_MODE | `1` = accelerometer high-performance mode disabled (low-power) |
| [3] | USR_OFF_W | User offset weight: `0` = 2^-10 g/LSB; `1` = 2^-6 g/LSB |
| [2:0] | FTYPE[2:0] | Gyroscope low-pass filter LPF1 bandwidth selection |

**FTYPE (gyro LPF1 bandwidth at 6667 Hz ODR):**

| FTYPE | Cutoff |
|-------|--------|
| `000` | 245 Hz |
| `001` | 195 Hz |
| `010` | 155 Hz |
| `011` | 293 Hz |
| `100` | 233 Hz |
| `101` | 171 Hz |
| `110` | 281 Hz |
| `111` | 40 Hz |

---

### CTRL7_G (0x16)

R/W | Reset: `0x00`

Gyroscope high-performance mode, high-pass filter, and OIS path control.

| Bit | Name | Description |
|-----|------|-------------|
| [7] | G_HM_MODE | `1` = gyroscope high-performance mode disabled (low-power) |
| [6] | HP_EN_G | `1` = enable gyroscope high-pass filter |
| [5:4] | HPM_G[1:0] | Gyroscope HP filter cutoff: `00`=16 mHz, `01`=65 mHz, `10`=260 mHz, `11`=1.04 Hz |
| [3] | — | Reserved |
| [2] | OIS_ON_EN | `1` = OIS chain enabled from UI (UI CTRL reg takes priority if set) |
| [1] | USR_OFF_ON_OUT | `1` = user offset correction applied to output data |
| [0] | OIS_ON | `1` = OIS chain enabled from UI side |

---

### CTRL8_XL (0x17)

R/W | Reset: `0x00`

Accelerometer composite filter, high-pass filter configuration.

| Bit | Name | Description |
|-----|------|-------------|
| [7:5] | HPCF_XL[2:0] | HP/LPF2 cutoff selection (frequency depends on ODR) |
| [4] | HP_REF_MODE_XL | `1` = HP filter reference mode enabled (tracks DC component) |
| [3] | FASTSETTL_MODE_XL | `1` = fast settling mode enabled for HP filter |
| [2] | HP_SLOPE_XL_EN | `1` = HP or slope filter applied to accel output data |
| [1] | — | Reserved |
| [0] | LOW_PASS_ON_6D | `1` = LPF2 output applied to 6D detection function |

**HPCF_XL ODR/50 cutoff frequencies:**

| HPCF_XL | Cutoff (divisor of ODR) |
|---------|------------------------|
| `000` | ODR/4 (LPF2) or HP at ODR/2 |
| `001` | ODR/100 |
| `010` | ODR/9 |
| `011` | ODR/400 |
| `100` | ODR/20 |
| `101` | ODR/800 |
| `110` | ODR/45 |
| `111` | ODR/ODR (no filter) |

---

### CTRL9_XL (0x18)

R/W | Reset: `0x00`

DEN stamping axis selection and device configuration.

| Bit | Name | Description |
|-----|------|-------------|
| [7] | DEN_X | `1` = DEN value stored in X-axis LSb |
| [6] | DEN_Y | `1` = DEN value stored in Y-axis LSb |
| [5] | DEN_Z | `1` = DEN value stored in Z-axis LSb |
| [4:3] | DEN_XL_G[1:0] | DEN stamping sensor: `00`=gyro, `01`=accel, `10/11`=both |
| [2] | DEN_LH | DEN active level: `0`=low, `1`=high |
| [1] | DEVICE_CONF | `1` = correct operation of device (recommended to set to 1) |
| [0] | — | Reserved |

> Set `DEVICE_CONF=1` after power-on for correct device operation.

---

### CTRL10_C (0x19)

R/W | Reset: `0x00`

Timestamp counter enable.

| Bit | Name | Description |
|-----|------|-------------|
| [7:6] | — | Reserved |
| [5] | TIMESTAMP_EN | `1` = timestamp counter enabled; counter value read from TIMESTAMP0-3 |
| [4:0] | — | Reserved |

> Timestamp counter resolution is 25 µs per LSb. Reset by writing `0xAA` to TIMESTAMP2.

---

### ALL_INT_SRC (0x1A)

R | Reset: `0x00`

Latched source register for all interrupt events. Reading this register clears all latched
interrupt flags (if LIR=1 in TAP_CFG0).

| Bit | Name | Description |
|-----|------|-------------|
| [7] | TIMESTAMP_ENDCOUNT | `1` = timestamp counter overflow occurred |
| [6] | — | Reserved |
| [5] | SLEEP_CHANGE_IA | `1` = activity/inactivity change event detected |
| [4] | D6D_IA | `1` = change in orientation detected (6D/4D) |
| [3] | DOUBLE_TAP | `1` = double-tap event detected |
| [2] | SINGLE_TAP | `1` = single-tap event detected |
| [1] | WU_IA | `1` = wake-up event detected |
| [0] | FF_IA | `1` = free-fall event detected |

---

### WAKE_UP_SRC (0x1B)

R | Reset: `0x00`

Wake-up event source details.

| Bit | Name | Description |
|-----|------|-------------|
| [7] | — | Reserved |
| [6] | SLEEP_CHANGE_IA | `1` = activity/inactivity change detected |
| [5] | FF_IA | `1` = free-fall event |
| [4] | SLEEP_STATE | `1` = device in sleep state |
| [3] | WU_IA | `1` = wake-up event |
| [2] | X_WU | `1` = wake-up on X-axis |
| [1] | Y_WU | `1` = wake-up on Y-axis |
| [0] | Z_WU | `1` = wake-up on Z-axis |

---

### TAP_SRC (0x1C)

R | Reset: `0x00`

Tap and double-tap detection event source.

| Bit | Name | Description |
|-----|------|-------------|
| [7] | — | Reserved |
| [6] | TAP_IA | `1` = tap event detected |
| [5] | SINGLE_TAP | `1` = single-tap detected |
| [4] | DOUBLE_TAP | `1` = double-tap detected |
| [3] | TAP_SIGN | `0` = tap detected on positive axis; `1` = negative axis |
| [2] | X_TAP | `1` = tap on X-axis |
| [1] | Y_TAP | `1` = tap on Y-axis |
| [0] | Z_TAP | `1` = tap on Z-axis |

---

### D6D_SRC (0x1D)

R | Reset: `0x00`

6D orientation detection event source.

| Bit | Name | Description |
|-----|------|-------------|
| [7] | DEN_DRDY | `1` = DEN data-ready signal level |
| [6] | D6D_IA | `1` = orientation change detected |
| [5] | ZH | `1` = Z-axis high orientation detected |
| [4] | ZL | `1` = Z-axis low orientation detected |
| [3] | YH | `1` = Y-axis high orientation detected |
| [2] | YL | `1` = Y-axis low orientation detected |
| [1] | XH | `1` = X-axis high orientation detected |
| [0] | XL | `1` = X-axis low orientation detected |

---

### STATUS_REG (0x1E)

R | Reset: `0x00`

Data-ready status for all sensors. Poll this register before reading output data.

| Bit | Name | Description |
|-----|------|-------------|
| [7:3] | — | Reserved |
| [2] | TDA | `1` = new temperature data available |
| [1] | GDA | `1` = new gyroscope data available |
| [0] | XLDA | `1` = new accelerometer data available |

---

### OUT_TEMP_L (0x20)

R | Reset: `0x00`

Temperature sensor output, low byte. 16-bit two's complement value, little-endian.
Sensitivity: 256 LSB/°C. Offset: 0 = 25 °C.

Formula: `T_degC = OUT_TEMP / 256.0 + 25.0`

---

### OUT_TEMP_H (0x21)

R | Reset: `0x00`

Temperature sensor output, high byte.

---

### OUTX_L_G (0x22)

R | Reset: `0x00`

Gyroscope X-axis angular rate output, low byte. 16-bit two's complement.

---

### OUTX_H_G (0x23)

R | Reset: `0x00`

Gyroscope X-axis angular rate output, high byte.

---

### OUTY_L_G (0x24)

R | Reset: `0x00`

Gyroscope Y-axis angular rate output, low byte.

---

### OUTY_H_G (0x25)

R | Reset: `0x00`

Gyroscope Y-axis angular rate output, high byte.

---

### OUTZ_L_G (0x26)

R | Reset: `0x00`

Gyroscope Z-axis angular rate output, low byte.

---

### OUTZ_H_G (0x27)

R | Reset: `0x00`

Gyroscope Z-axis angular rate output, high byte.

---

### OUTX_L_A (0x28)

R | Reset: `0x00`

Accelerometer X-axis linear acceleration output, low byte. 16-bit two's complement.
Burst-read all 6 accel bytes starting at `0x28`.

---

### OUTX_H_A (0x29)

R | Reset: `0x00`

Accelerometer X-axis linear acceleration output, high byte.

---

### OUTY_L_A (0x2A)

R | Reset: `0x00`

Accelerometer Y-axis linear acceleration output, low byte.

---

### OUTY_H_A (0x2B)

R | Reset: `0x00`

Accelerometer Y-axis linear acceleration output, high byte.

---

### OUTZ_L_A (0x2C)

R | Reset: `0x00`

Accelerometer Z-axis linear acceleration output, low byte.

---

### OUTZ_H_A (0x2D)

R | Reset: `0x00`

Accelerometer Z-axis linear acceleration output, high byte.

---

### EMB_FUNC_STATUS_MAINPAGE (0x35)

R | Reset: `0x00`

Interrupt status for embedded functions (mirrored on main page from embedded functions page).

| Bit | Name | Description |
|-----|------|-------------|
| [7] | — | Reserved |
| [6] | IS_FSM_LC | `1` = FSM long counter timeout event |
| [5] | — | Reserved |
| [4] | IS_SIGMOT | `1` = significant motion event |
| [3] | IS_TILT | `1` = tilt event |
| [2] | IS_STEP_DET | `1` = step detected (pedometer) |
| [1:0] | — | Reserved |

---

### FSM_STATUS_A_MAINPAGE (0x36)

R | Reset: `0x00`

Interrupt status for FSM programs 1–8 (mirrored on main page).

| Bit | Name | Description |
|-----|------|-------------|
| [7] | IS_FSM8 | FSM program 8 interrupt flag |
| [6] | IS_FSM7 | FSM program 7 interrupt flag |
| [5] | IS_FSM6 | FSM program 6 interrupt flag |
| [4] | IS_FSM5 | FSM program 5 interrupt flag |
| [3] | IS_FSM4 | FSM program 4 interrupt flag |
| [2] | IS_FSM3 | FSM program 3 interrupt flag |
| [1] | IS_FSM2 | FSM program 2 interrupt flag |
| [0] | IS_FSM1 | FSM program 1 interrupt flag |

---

### FSM_STATUS_B_MAINPAGE (0x37)

R | Reset: `0x00`

Interrupt status for FSM programs 9–16 (mirrored on main page).

| Bit | Name | Description |
|-----|------|-------------|
| [7] | IS_FSM16 | FSM program 16 interrupt flag |
| [6] | IS_FSM15 | FSM program 15 interrupt flag |
| [5] | IS_FSM14 | FSM program 14 interrupt flag |
| [4] | IS_FSM13 | FSM program 13 interrupt flag |
| [3] | IS_FSM12 | FSM program 12 interrupt flag |
| [2] | IS_FSM11 | FSM program 11 interrupt flag |
| [1] | IS_FSM10 | FSM program 10 interrupt flag |
| [0] | IS_FSM9 | FSM program 9 interrupt flag |

---

### MLC_STATUS_MAINPAGE (0x38)

R | Reset: `0x00`

Machine Learning Core interrupt status (mirrored on main page).

| Bit | Name | Description |
|-----|------|-------------|
| [7:4] | — | Reserved |
| [3] | IS_MLC4 | MLC4 interrupt |
| [2] | IS_MLC3 | MLC3 interrupt |
| [1] | IS_MLC2 | MLC2 interrupt |
| [0] | IS_MLC1 | MLC1 interrupt |

---

### STATUS_MASTER_MAINPAGE (0x39)

R | Reset: `0x00`

Sensor hub master status (mirrored on main page).

| Bit | Name | Description |
|-----|------|-------------|
| [7] | — | Reserved |
| [6] | WR_ONCE_DONE | `1` = write_once operation for slave 0 completed |
| [5] | SLAVE3_NACK | `1` = slave 3 not acknowledged |
| [4] | SLAVE2_NACK | `1` = slave 2 not acknowledged |
| [3] | SLAVE1_NACK | `1` = slave 1 not acknowledged |
| [2] | SLAVE0_NACK | `1` = slave 0 not acknowledged |
| [1] | — | Reserved |
| [0] | SENS_HUB_ENDOP | `1` = all sensor hub operations completed |

---

### FIFO_STATUS1 (0x3A)

R | Reset: `0x00`

FIFO fill level, lower 8 bits.

| Bit | Name | Description |
|-----|------|-------------|
| [7:0] | DIFF_FIFO[7:0] | Number of unread sensor data (tag + 6 bytes) stored in FIFO, bits [7:0] |

---

### FIFO_STATUS2 (0x3B)

R | Reset: `0x00`

FIFO fill level upper bits and FIFO interrupt flags.

| Bit | Name | Description |
|-----|------|-------------|
| [7] | FIFO_WTM_IA | `1` = FIFO watermark level reached |
| [6] | FIFO_OVR_IA | `1` = FIFO overrun occurred |
| [5] | FIFO_FULL_IA | `1` = FIFO is full |
| [4] | COUNTER_BDR_IA | `1` = BDR counter threshold reached |
| [3] | OVER_RUN_LATCHED | `1` = latched FIFO overrun (cleared by reading FIFO_STATUS) |
| [2] | — | Reserved |
| [1:0] | DIFF_FIFO[9:8] | FIFO fill level, upper 2 bits |

---

### TIMESTAMP0 (0x40)

R | Reset: `0x00`

Timestamp counter output register, byte 0 (least significant). 32-bit counter value, resolution 25 µs/LSb.
Burst-read all 4 bytes (0x40–0x43) for the full timestamp.

---

### TIMESTAMP1 (0x41)

R | Reset: `0x00`

Timestamp counter output register, byte 1.

---

### TIMESTAMP2 (0x42)

R | Reset: `0x00`

Timestamp counter output register, byte 2. Writing `0xAA` to this register resets the counter.

---

### TIMESTAMP3 (0x43)

R | Reset: `0x00`

Timestamp counter output register, byte 3 (most significant).

---

### TAP_CFG0 (0x56)

R/W | Reset: `0x00`

Tap detection axis enables, latched interrupt mode, activity/inactivity filter, and interrupt clear-on-read.

| Bit | Name | Description |
|-----|------|-------------|
| [7] | — | Reserved |
| [6] | INT_CLR_ON_READ | `1` = interrupt flags cleared when any output register is read |
| [5] | SLEEP_STATUS_ON_INT | `0` = sleep change on INT; `1` = sleep status on INT |
| [4] | SLOPE_FDS | `1` = HPF applied for activity/inactivity (slope filter used when 0) |
| [3] | TAP_X_EN | `1` = tap detection enabled on X-axis |
| [2] | TAP_Y_EN | `1` = tap detection enabled on Y-axis |
| [1] | TAP_Z_EN | `1` = tap detection enabled on Z-axis |
| [0] | LIR | `1` = latched interrupt (cleared by reading ALL_INT_SRC); `0` = pulsed |

---

### TAP_CFG1 (0x57)

R/W | Reset: `0x00`

Tap X-axis threshold and axis priority.

| Bit | Name | Description |
|-----|------|-------------|
| [7:5] | TAP_PRIORITY[2:0] | Axis priority for tap detection: `000`=X>Y>Z, `001`=Y>X>Z, `010`=X>Z>Y, `011`=Z>Y>X, `100`=X>Y>Z, `101`=Y>Z>X, `110`=Z>X>Y, `111`=Z>Y>X |
| [4:0] | TAP_THS_X[4:0] | Tap recognition threshold for X-axis (1 LSb = FS_XL/2^5) |

---

### TAP_CFG2 (0x58)

R/W | Reset: `0x00`

Tap Y-axis threshold, activity/inactivity enable, and global interrupt enable.

| Bit | Name | Description |
|-----|------|-------------|
| [7] | INTERRUPTS_ENABLE | `1` = enable basic interrupt functions (tap, wake-up, free-fall, 6D) |
| [6:5] | INACT_EN[1:0] | Activity/inactivity function: `00`=disabled, `01`=XL in low-power/gyro off, `10`=XL+gyro in low-power, `11`=XL normal/gyro low-power |
| [4:0] | TAP_THS_Y[4:0] | Tap recognition threshold for Y-axis (1 LSb = FS_XL/2^5) |

---

### TAP_THS_6D (0x59)

R/W | Reset: `0x00`

Tap Z-axis threshold, 6D orientation threshold, and D4D enable.

| Bit | Name | Description |
|-----|------|-------------|
| [7] | D4D_EN | `1` = 4D orientation detection enabled (portrait/landscape on Z-axis disabled) |
| [6:5] | SIXD_THS[1:0] | 6D threshold: `00`=80°, `01`=70°, `10`=60°, `11`=50° |
| [4:0] | TAP_THS_Z[4:0] | Tap recognition threshold for Z-axis (1 LSb = FS_XL/2^5) |

---

### INT_DUR2 (0x5A)

R/W | Reset: `0x00`

Double-tap timing: gap between taps, quiet window, and shock duration.

| Bit | Name | Description |
|-----|------|-------------|
| [7:4] | DUR[3:0] | Double-tap maximum gap duration. 1 LSb = 32 × ODR_XL^-1 (e.g. 4.7 ms at 208 Hz) |
| [3:2] | QUIET[1:0] | Expected quiet time between two consecutive taps. 1 LSb = 4 × ODR_XL^-1 |
| [1:0] | SHOCK[1:0] | Maximum duration of over-threshold event for tap. 1 LSb = 8 × ODR_XL^-1 |

---

### WAKE_UP_THS (0x5B)

R/W | Reset: `0x00`

Wake-up threshold configuration.

| Bit | Name | Description |
|-----|------|-------------|
| [7] | SINGLE_DOUBLE_TAP | `0` = single-tap only; `1` = single and double-tap detection |
| [6] | USR_OFF_ON_WU | `1` = apply user offset to data for wake-up detection |
| [5:0] | WK_THS[5:0] | Wake-up threshold. 1 LSb = FS_XL/2^6 |

---

### WAKE_UP_DUR (0x5C)

R/W | Reset: `0x00`

Wake-up duration, sleep mode duration, and free-fall duration MSb.

| Bit | Name | Description |
|-----|------|-------------|
| [7] | FF_DUR[5] | Free-fall duration MSb (combined with FF_DUR[4:0] in FREE_FALL register) |
| [6:5] | WAKE_DUR[1:0] | Wake-up duration: number of samples above threshold before event fires |
| [4] | WAKE_THS_W | `0` = wake-up threshold at FS_XL/2^6; `1` = wake-up threshold at FS_XL/2^8 |
| [3:0] | SLEEP_DUR[3:0] | Duration to enter sleep mode. 1 LSb = 512/ODR_XL. Default 0 = 16/ODR_XL |

---

### FREE_FALL (0x5D)

R/W | Reset: `0x00`

Free-fall detection threshold and minimum event duration.

| Bit | Name | Description |
|-----|------|-------------|
| [7:3] | FF_DUR[4:0] | Free-fall event minimum duration (lower 5 bits). 1 LSb = 1/ODR_XL |
| [2:0] | FF_THS[2:0] | Free-fall threshold: `000`=156 mg, `001`=219 mg, `010`=250 mg, `011`=312 mg, `100`=344 mg, `101`=406 mg, `110`=469 mg, `111`=500 mg |

---

### MD1_CFG (0x5E)

R/W | Reset: `0x00`

Routes embedded function and basic interrupt events to INT1 pin.

| Bit | Name | Description |
|-----|------|-------------|
| [7] | INT1_SLEEP_CHANGE | `1` = activity/inactivity change event → INT1 |
| [6] | INT1_SINGLE_TAP | `1` = single-tap event → INT1 |
| [5] | INT1_WU | `1` = wake-up event → INT1 |
| [4] | INT1_FF | `1` = free-fall event → INT1 |
| [3] | INT1_DOUBLE_TAP | `1` = double-tap event → INT1 |
| [2] | INT1_6D | `1` = 6D orientation event → INT1 |
| [1] | INT1_EMB_FUNC | `1` = embedded function event → INT1 |
| [0] | INT1_SHUB | `1` = sensor hub end-of-operation → INT1 |

---

### MD2_CFG (0x5F)

R/W | Reset: `0x00`

Routes embedded function and basic interrupt events to INT2 pin.

| Bit | Name | Description |
|-----|------|-------------|
| [7] | INT2_SLEEP_CHANGE | `1` = activity/inactivity change event → INT2 |
| [6] | INT2_SINGLE_TAP | `1` = single-tap event → INT2 |
| [5] | INT2_WU | `1` = wake-up event → INT2 |
| [4] | INT2_FF | `1` = free-fall event → INT2 |
| [3] | INT2_DOUBLE_TAP | `1` = double-tap event → INT2 |
| [2] | INT2_6D | `1` = 6D orientation event → INT2 |
| [1] | INT2_EMB_FUNC | `1` = embedded function event → INT2 |
| [0] | INT2_TIMESTAMP | `1` = timestamp overflow event → INT2 |

---

### INTERNAL_FREQ_FINE (0x63)

R | Reset: `0x00`

Internal oscillator frequency correction value. Used to compute accurate timestamp period.
Signed 8-bit value representing frequency deviation in ppm steps.

`T_step_us = 1 / (40000 * (1 + 0.0015 * INTERNAL_FREQ_FINE))` (approximate)

---

### INT_OIS (0x6F)

R/W | Reset: `0x00`

OIS (Optical Image Stabilization) chain interrupt and self-test control. Accessible via SPI2 or
via UI primary interface with CTRL7_G[OIS_ON_EN]=1.

| Bit | Name | Description |
|-----|------|-------------|
| [7] | INT2_DRDY_OIS | `1` = OIS data-ready signal routed to INT2 |
| [6] | LVL2_OIS | OIS chain level 2 enable |
| [5] | DEN_LH_OIS | DEN active level for OIS: `0`=low, `1`=high |
| [4:2] | — | Reserved |
| [1:0] | ST_XL_OIS[1:0] | Accelerometer OIS self-test: `00`=off, `01`=positive, `10`=negative |

---

### CTRL1_OIS (0x70)

R/W | Reset: `0x00`

OIS chain enable, gyroscope OIS full-scale, and SPI2 mode.

| Bit | Name | Description |
|-----|------|-------------|
| [7] | — | Reserved |
| [6] | LVL1_OIS | OIS chain level 1 enable |
| [5] | SIM_OIS | SPI2 mode: `0`=4-wire, `1`=3-wire |
| [4] | MODE4_EN | `1` = mode 4 enabled for OIS chain |
| [3:2] | FS_G_OIS[1:0] | Gyroscope OIS full-scale: `00`=250 dps, `01`=500 dps, `10`=1000 dps, `11`=2000 dps |
| [1] | FS_125_OIS | `1` = gyroscope OIS full-scale ±125 dps |
| [0] | OIS_EN_SPI2 | `1` = OIS chain enabled via SPI2 |

---

### CTRL2_OIS (0x71)

R/W | Reset: `0x00`

OIS gyroscope high-pass filter configuration.

| Bit | Name | Description |
|-----|------|-------------|
| [7:6] | — | Reserved |
| [5:4] | HPM_OIS[1:0] | OIS gyro HP filter cutoff: `00`=16 mHz, `01`=65 mHz, `10`=260 mHz, `11`=1.04 Hz |
| [3] | — | Reserved |
| [2:1] | FTYPE_OIS[1:0] | OIS gyro LP filter bandwidth selection |
| [0] | HP_EN_OIS | `1` = OIS gyroscope HP filter enabled |

---

### CTRL3_OIS (0x72)

R/W | Reset: `0x00`

OIS accelerometer full-scale, LPF filter, and gyroscope self-test.

| Bit | Name | Description |
|-----|------|-------------|
| [7:6] | FS_XL_OIS[1:0] | Accelerometer OIS full-scale: `00`=±2g, `01`=±16g, `10`=±4g, `11`=±8g |
| [5:3] | FILTER_XL_CONF_OIS[2:0] | Accelerometer OIS LPF bandwidth selection |
| [2:1] | ST_OIS[1:0] | Gyroscope OIS self-test: `00`=off, `01`=positive, `10`=negative |
| [0] | ST_OIS_CLAMPDIS | `1` = clamp disable for OIS self-test |

---

### X_OFS_USR (0x73)

R/W | Reset: `0x00`

User offset correction for accelerometer X-axis. 8-bit two's complement.
Weight selected by `USR_OFF_W` bit in CTRL6_C: `0`=2^-10 g/LSB, `1`=2^-6 g/LSB.

---

### Y_OFS_USR (0x74)

R/W | Reset: `0x00`

User offset correction for accelerometer Y-axis. Same format as X_OFS_USR.

---

### Z_OFS_USR (0x75)

R/W | Reset: `0x00`

User offset correction for accelerometer Z-axis. Same format as X_OFS_USR.
Apply offsets to output with `USR_OFF_ON_OUT=1` in CTRL7_G.

---

### FIFO_DATA_OUT_TAG (0x78)

R | Reset: `0x00`

FIFO data output tag register. Identifies the sensor data source and parity for the next FIFO word.

| Bit | Name | Description |
|-----|------|-------------|
| [7:3] | TAG_SENSOR[4:0] | Data tag identifying the FIFO word source |
| [2] | TAG_CNT[1:0] MSb | 2-bit tag counter MSb (wraps 0–3 per sensor) |
| [1] | TAG_CNT[1:0] LSb | 2-bit tag counter LSb |
| [0] | TAG_PARITY | Even parity of TAG_SENSOR field |

**TAG_SENSOR values:**

| Code | Source |
|------|--------|
| `0x01` | Gyroscope NC (not compressed) |
| `0x02` | Accelerometer NC |
| `0x03` | Temperature |
| `0x04` | Timestamp |
| `0x05` | CFG change |
| `0x06` | Accelerometer NC_T_2 (batched at half ODR) |
| `0x07` | Accelerometer NC_T_1 (batched at full ODR) |
| `0x08` | Accelerometer 2x_C (2x compressed) |
| `0x09` | Accelerometer 3x_C (3x compressed) |
| `0x0A` | Gyroscope NC_T_2 |
| `0x0B` | Gyroscope NC_T_1 |
| `0x0C` | Gyroscope 2x_C |
| `0x0D` | Gyroscope 3x_C |
| `0x0E` | Sensor hub slave 0 |
| `0x0F` | Sensor hub slave 1 |
| `0x10` | Sensor hub slave 2 |
| `0x11` | Sensor hub slave 3 |
| `0x12` | Step counter |
| `0x19` | OIS gyroscope NC |
| `0x1A` | OIS accelerometer |
| `0x1B` | External sensor EIS gyroscope |

---

### FIFO_DATA_OUT_X_L (0x79)

R | Reset: `0x00`

FIFO output word first axis (or data byte 0–1), low byte.

---

### FIFO_DATA_OUT_X_H (0x7A)

R | Reset: `0x00`

FIFO output word first axis, high byte.

---

### FIFO_DATA_OUT_Y_L (0x7B)

R | Reset: `0x00`

FIFO output word second axis (or data byte 2–3), low byte.

---

### FIFO_DATA_OUT_Y_H (0x7C)

R | Reset: `0x00`

FIFO output word second axis, high byte.

---

### FIFO_DATA_OUT_Z_L (0x7D)

R | Reset: `0x00`

FIFO output word third axis (or data byte 4–5), low byte.

---

### FIFO_DATA_OUT_Z_H (0x7E)

R | Reset: `0x00`

FIFO output word third axis, high byte.

---

## Embedded Functions Registers (Page 1)

Access by writing `0x80` to `FUNC_CFG_ACCESS (0x01)` to set `FUNC_CFG_ACCESS[7]=1`. Restore
`0x00` when done.

| Address | Register Name | R/W | Reset | Description |
|---------|---------------|-----|-------|-------------|
| `0x02` | PAGE_SEL | R/W | `0x31` | Embedded functions page selection |
| `0x04` | EMB_FUNC_EN_A | R/W | `0x00` | Enable pedometer, tilt, significant motion, sensor fusion |
| `0x05` | EMB_FUNC_EN_B | R/W | `0x00` | Enable FSM programs and MLC |
| `0x08` | PAGE_ADDRESS | R/W | `0x00` | Target address for embedded page read/write |
| `0x09` | PAGE_VALUE | R/W | `0x00` | Data value for embedded page read/write |
| `0x0A` | EMB_FUNC_INT1 | R/W | `0x00` | Route embedded function events to INT1 |
| `0x0B` | FSM_INT1_A | R/W | `0x00` | Route FSM1–FSM8 interrupts to INT1 |
| `0x0C` | FSM_INT1_B | R/W | `0x00` | Route FSM9–FSM16 interrupts to INT1 |
| `0x0D` | MLC_INT1 | R/W | `0x00` | Route MLC1–MLC4 interrupts to INT1 |
| `0x0E` | EMB_FUNC_INT2 | R/W | `0x00` | Route embedded function events to INT2 |
| `0x0F` | FSM_INT2_A | R/W | `0x00` | Route FSM1–FSM8 interrupts to INT2 |
| `0x10` | FSM_INT2_B | R/W | `0x00` | Route FSM9–FSM16 interrupts to INT2 |
| `0x11` | MLC_INT2 | R/W | `0x00` | Route MLC1–MLC4 interrupts to INT2 |
| `0x12` | EMB_FUNC_STATUS | R | `0x00` | Embedded function interrupt status |
| `0x13` | FSM_STATUS_A | R | `0x00` | FSM1–FSM8 interrupt status |
| `0x14` | FSM_STATUS_B | R | `0x00` | FSM9–FSM16 interrupt status |
| `0x15` | MLC_STATUS | R | `0x00` | MLC1–MLC4 interrupt status |
| `0x44` | EMB_FUNC_FIFO_CFG | R/W | `0x00` | Enable pedometer step counter batching into FIFO |
| `0x46` | FSM_ENABLE_A | R/W | `0x00` | Enable FSM programs 1–8 |
| `0x47` | FSM_ENABLE_B | R/W | `0x00` | Enable FSM programs 9–16 |
| `0x4C`–`0x5B` | FSM_OUTS1–FSM_OUTS16 | R | `0x00` | FSM output state registers (one per FSM program) |
| `0x5F` | EMB_FUNC_ODR_CFG_B | R/W | `0x00` | FSM output data rate selection |
| `0x60` | EMB_FUNC_ODR_CFG_C | R/W | `0x00` | MLC output data rate selection |
| `0x62` | STEP_COUNTER_L | R | `0x00` | Pedometer step count, low byte |
| `0x63` | STEP_COUNTER_H | R | `0x00` | Pedometer step count, high byte |
| `0x64` | EMB_FUNC_SRC | R | `0x00` | Embedded function event source flags |
| `0x66` | EMB_FUNC_INIT_A | R/W | `0x00` | Embedded function initialization (pedometer, tilt, sig. motion) |
| `0x67` | EMB_FUNC_INIT_B | R/W | `0x00` | Embedded function initialization (FSM, MLC) |
| `0x70`–`0x77` | MLC0_SRC–MLC7_SRC | R | `0x00` | Machine Learning Core output values (MLC1–MLC4, decision tree results) |

### EMB_FUNC_EN_A (0x04)

| Bit | Name | Description |
|-----|------|-------------|
| [7] | — | Reserved |
| [6] | — | Reserved |
| [5] | SIGN_MOTION_EN | `1` = significant motion detection enabled |
| [4] | TILT_EN | `1` = tilt detection enabled |
| [3] | PEDO_EN | `1` = pedometer algorithm enabled |
| [2] | — | Reserved |
| [1] | — | Reserved |
| [0] | — | Reserved |

### EMB_FUNC_EN_B (0x05)

| Bit | Name | Description |
|-----|------|-------------|
| [7:5] | — | Reserved |
| [4] | MLC_EN | `1` = Machine Learning Core enabled |
| [3] | FSM_EN | `1` = Finite State Machine programs enabled |
| [2:0] | — | Reserved |

---

## Sensor Hub Registers (Page 2)

Access by writing `0x40` to `FUNC_CFG_ACCESS (0x01)` to set `SHUB_REG_ACCESS[6]=1`. Restore
`0x00` when done. The sensor hub allows the ISM330DHCX to act as I2C master and automatically
read data from up to 4 external slave sensors.

| Address | Register Name | R/W | Reset | Description |
|---------|---------------|-----|-------|-------------|
| `0x02`–`0x09` | SENSOR_HUB_1–SENSOR_HUB_8 | R | `0x00` | External sensor data bytes 1–8 |
| `0x0A`–`0x13` | SENSOR_HUB_9–SENSOR_HUB_18 | R | `0x00` | External sensor data bytes 9–18 |
| `0x14` | MASTER_CONFIG | R/W | `0x00` | Sensor hub master configuration |
| `0x15` | SLV0_ADD | R/W | `0x00` | Slave 0 I2C address and R/W bit |
| `0x16` | SLV0_SUBADD | R/W | `0x00` | Slave 0 register address |
| `0x17` | SLV0_CONFIG | R/W | `0x00` | Slave 0 number of bytes to read, decimation |
| `0x18` | SLV1_ADD | R/W | `0x00` | Slave 1 I2C address and R/W bit |
| `0x19` | SLV1_SUBADD | R/W | `0x00` | Slave 1 register address |
| `0x1A` | SLV1_CONFIG | R/W | `0x00` | Slave 1 number of bytes to read, decimation |
| `0x1B` | SLV2_ADD | R/W | `0x00` | Slave 2 I2C address and R/W bit |
| `0x1C` | SLV2_SUBADD | R/W | `0x00` | Slave 2 register address |
| `0x1D` | SLV2_CONFIG | R/W | `0x00` | Slave 2 number of bytes to read, decimation |
| `0x1E` | SLV3_ADD | R/W | `0x00` | Slave 3 I2C address and R/W bit |
| `0x1F` | SLV3_SUBADD | R/W | `0x00` | Slave 3 register address |
| `0x20` | SLV3_CONFIG | R/W | `0x00` | Slave 3 number of bytes to read, decimation |
| `0x21` | DATAWRITE_SLV0 | R/W | `0x00` | Data byte to write to slave 0 |
| `0x22` | STATUS_MASTER | R | `0x00` | Sensor hub master operation status |

### MASTER_CONFIG (0x14)

| Bit | Name | Description |
|-----|------|-------------|
| [7] | RST_MASTER_REGS | `1` = reset sensor hub slave configuration registers |
| [6] | WRITE_ONCE | `1` = perform single write to slave 0 then switch to read-only mode |
| [5] | — | Reserved |
| [4] | PASS_THROUGH_MODE | `1` = I2C master directly passes through to SDx/SCx pins |
| [3] | START_CONFIG | `1` = trigger mode: sensor hub starts at XL data-ready |
| [2] | PULL_UP_EN | `1` = internal pull-up on SDx pin enabled |
| [1] | AUX_SENS_ON[1:0] MSb | Number of slaves connected (2-bit field): `00`=slave 0 only |
| [0] | AUX_SENS_ON[1:0] LSb | `01`=slaves 0+1, `10`=slaves 0+1+2, `11`=slaves 0+1+2+3 |

### SLV0_ADD (0x15) / SLV1_ADD (0x18) / SLV2_ADD (0x1B) / SLV3_ADD (0x1E)

| Bit | Name | Description |
|-----|------|-------------|
| [7:1] | SLAVEx_ADD[6:0] | Slave I2C 7-bit address |
| [0] | Rw_0 | `0` = write; `1` = read |

### SLV0_CONFIG (0x17) / SLV1_CONFIG (0x1A) / SLV2_CONFIG (0x1D) / SLV3_CONFIG (0x20)

| Bit | Name | Description |
|-----|------|-------------|
| [7:6] | SLAVEx_ODR (SLV0 only) | `00`=same as ODR_XL, decimation factors for others |
| [5] | SHUB_ODR_OUT_x (SLV1–3) | Slave sensor data output rate decimation |
| [4:3] | BATCH_EXT_SENS_x_EN | Enable batching of this slave's data into FIFO |
| [2:0] | SLAVEx_NUMOP[2:0] | Number of bytes to read from slave (0 = 1 byte, max 7 = 8 bytes) |

---

*Sources:*
- *ST Datasheet DS12441 Rev 8 — ISM330DHCX*
- *ST PID Driver: [STMicroelectronics/ism330dhcx-pid](https://github.com/STMicroelectronics/ism330dhcx-pid)*
- *ST Application Note AN5398*
