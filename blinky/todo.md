## Improvements

1. Use of a critical section mutex is a bit overkill since only a single u64 type is modified. Additionally, it rejects button presses during read/write of it. Better to use an Atomic type instead, that way MCU will retry failed read/writes once it's able to. Will also save like 1 cycle. Massive savings.

## Possible bugfixes

1. Look into making reset button function when debug probe is running.
