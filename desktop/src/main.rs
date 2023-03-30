use chip8::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::env;
use std::fs::File;
use std::io::Read;

// STYLISTIC PREFERENCES so that the game appears properly on the Emulation
const SCALE: u32 = 15;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;
const TICKS_PER_FRAME: usize = 10;

// Main code
fn main() {
    let args: Vec<_> = env::args().collect();
    /*
        Just accept 1 argument
        The actual path to the source file containing the code of the game
    */
    if args.len() != 2 {
        println!("Usage: cargo run path/to/game");
        return;
    }

    // Create the emulator
    let mut chip8 = CPU::new();
    print!("{}", &args[1]);

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
                // If the window is closed or the ESCAPE button is pressed, exit out of the game by ending the gameloop
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'gameloop;
                }

                // If a key is pressed, set the same to be pressed in the CPU
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    if let Some(k) = key2btn(key) {
                        chip8.keypress(k, true);
                    }
                }

                // If a key is released, set the same to be released in the CPU
                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    if let Some(k) = key2btn(key) {
                        chip8.keypress(k, false);
                    }
                }

                // Handle all the rest of the events, and do nothing
                _ => (),
            }
        }

        // Execute a clock-cycle
        for _ in 0..TICKS_PER_FRAME {
            chip8.tick();
        }
        chip8.tick_timers();

        // Draw the screen
        draw_screen(&chip8, &mut canvas);
    }
}

// Draw's the canvas
fn draw_screen(chip8: &CPU, canvas: &mut Canvas<Window>) {
    // Clear canvas as black
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let screen_buf = chip8.get_display();
    // Now set draw color to white, iterate through each point and see if it should be drawn
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for (i, pixel) in screen_buf.iter().enumerate() {
        if *pixel {
            // Convert our 1D array's index into a 2D (x,y) position
            let x = (i % SCREEN_WIDTH) as u32;
            let y = (i / SCREEN_WIDTH) as u32;

            // Draw a rectangle at (x,y), scaled up by our SCALE value
            let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
            canvas.fill_rect(rect).unwrap();
        }
    }
    canvas.present();
}

/*
    Converts a keypress on the actual device to a byte KEY for the emulator

    Keyboard                    Chip-8
    +---+---+---+---+           +---+---+---+---+
    | 1 | 2 | 3 | 4 |           | 1 | 2 | 3 | C |
    +---+---+---+---+           +---+---+---+---+
    | Q | W | E | R |           | 4 | 5 | 6 | D |
    +---+---+---+---+     =>    +---+---+---+---+
    | A | S | D | F |           | 7 | 8 | 9 | E |
    +---+---+---+---+           +---+---+---+---+
    | Z | X | C | V |           | A | 0 | B | F |
    +---+---+---+---+           +---+---+---+---+
*/
fn key2btn(key: Keycode) -> Option<usize> {
    match key {
        Keycode::Num1 => Some(0x1),
        Keycode::Num2 => Some(0x2),
        Keycode::Num3 => Some(0x3),
        Keycode::Num4 => Some(0xC),
        Keycode::Q => Some(0x4),
        Keycode::W => Some(0x5),
        Keycode::E => Some(0x6),
        Keycode::R => Some(0xD),
        Keycode::A => Some(0x7),
        Keycode::S => Some(0x8),
        Keycode::D => Some(0x9),
        Keycode::F => Some(0xE),
        Keycode::Z => Some(0xA),
        Keycode::X => Some(0x0),
        Keycode::C => Some(0xB),
        Keycode::V => Some(0xF),
        _ => None,
    }
}
