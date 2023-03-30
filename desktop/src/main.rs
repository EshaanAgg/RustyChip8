use chip8::*;
use sdl2::event::Event;
use std::env;
use std::fs::File;
use std::io::Read;

// SCALING PREFERENCES so that the game appears properly on the Emulation
const SCALE: u32 = 15;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;

fn main() {
    let args: Vec<_> = env::args().collect();
    // Just accept 1 argument
    // The actual path to the source file containing the code of the game
    if args.len() != 2 {
        println!("Usage: cargo run path/to/game");
        return;
    }

    // Create the emulator
    let mut chip8 = CPU::new();

    // Try to open the file and then load it into the chip8's RAM
    let mut rom = File::open(&args[1]).expect("Unable to open file");
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer).unwrap();
    chip8.load(&buffer);

    // Setup SDL (Boilerplate Code)
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Chip-8 Emulator", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    // Create a canvas to draw on and reset the same to blank
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.clear();
    canvas.present();

    // Using SDL's EventPump to listen for Events
    let mut event_pump = sdl_context.event_pump().unwrap();
    'gameloop: loop {
        // Listen for all possible events
        for evt in event_pump.poll_iter() {
            match evt {
                // If the window is closed, exit out of the game by ending the gameloop
                Event::Quit { .. } => {
                    break 'gameloop;
                }
                _ => (),
            }
        }

        // Execute a clock-cycle
        chip8.tick();
    }
}
