# Rusty Chip-8

The `Hello World` of Emulation, made in your favourite memory-safe Rust.

### Chip 8 Specifications

- A `64x32 monochrome display`, drawn to via sprites that are always `8 pixels wide` and `between 1 and 16 pixels tall`
- Sixteen `18-bit general purpose registers`, referred to as `V0` through `VF`. (`VF` also doubles as the flag register for overflow operations)
- `16-bit program counter`
- Single `16-bit register` used as a pointer for memory access, called the `I Register`
- An unstandardised amount of `RAM` (most emulators allocate `4 KB`)
- `16-bit stack` used for calling and returning from subroutines
- `16-key keyboard` input

Two special registers which decrease each frame and trigger upon reaching zero are also present:

1. `Delay timer`: Used for time-based game events
2. `Sound timer`: Used to trigger the audio beep
