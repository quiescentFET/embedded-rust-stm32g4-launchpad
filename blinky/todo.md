## Improvements

## Done/Dismissed Improvements
1. ~~Use of a critical section mutex is a bit overkill since only a single u64 type is modified. Additionally, it rejects button presses during read/write of it. Better to use an Atomic type instead, that way MCU will retry failed read/writes once it's able to. Will also save like 1 cycle. Massive savings.~~ DONE
2. ~~Use u32 or smaller type for delay values. At present u64 is used as that is what the timer function was expecting, but that type is not native to a 32-bit architecture. Incurring unnecessary overhead. Perhaps checktimer driver for alternatives~~ Time driver is writtenf or u64, too much risk to change to u32. DISMISSED.
3. ~~Needs confirmation, but assuming using HSI, the internal 16MHz oscillator. ~~Should use LSE, the more stable external 32.768KHz crystal (X2).~~ Turns out the LSE is for RTC and non sysclk use. For accuracy, use HSE, the external 24MHz crystal (X3).~~ DONE

## Possible bugfixes

1. Look into making reset button function when debug probe is running.
