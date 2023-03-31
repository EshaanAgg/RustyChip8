# Rusty Chip-8

The `Hello World` of Emulation, made in your favourite memory-safe Rust.

### Installation

You need only `SDL2` package and `Cargo` to run the project!

Run `sudo apt-get install libsdl2-dev` to install `SDL2` and [follow this guide](https://doc.rust-lang.org/cargo/getting-started/installation.html) to install Cargo!

After the above setup, you can start playing the games with the following commands:

```
cd desktop
cargo run ../roms/[GAME_NAME]
```

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

### About the Project

The project has two main directories.

1. `chip8`: This is a library package with holds all the code for the `CHIP8 Emulator`
2. `desktop`: This is a binary application package which would be using the `chip8` emulator to run games!

### Scopes of Improvement

- [ ] Add better comments for all the OP Codes
- [ ] Rename classes and variables to standard CPU names
- [ ] Add more ROMS
- [ ] Add WASM integration
- [ ] Add tests for each of the OP Codes and methods
- [ ] Add frontend to play the games
