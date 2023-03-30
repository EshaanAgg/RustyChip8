// SCREEN SIZE CONSTANTS
pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

// ARCHITECTURE CONSTANTS
const RAM_SIZE: usize = 4096;
const NUM_REGS: usize = 16;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;

// IMPLEMENTATION CONSTANTS
const START_ADDR: u16 = 0x200;

// DEFINING THE FONTSET
// 80 = 16 most commonly used characters * 5 elements to display 5 rows
const FONTSET_SIZE: usize = 80;

// Each row contains 8 bits
const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

// The actual emulator class
pub struct CPU {
    pc: u16,                                      // Program Counter
    ram: [u8; RAM_SIZE],                          // RAM
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT], // Screen Pixels
    v_reg: [u8; NUM_REGS],                        // V Registers
    i_reg: u16,                                   // Instruction Register
    sp: u16,                                      // Stack Pointer
    stack: [u16; STACK_SIZE],                     // Stack
    keys: [bool; NUM_KEYS],                       // Keys
    dt: u8,                                       // Delay Timer
    st: u8,                                       // Stack Timer
}

// Deals with the most basic fucntionality that involves with instanstiating an emulator
impl CPU {
    // Constructor method to initialize a new instance
    pub fn new() -> Self {
        let mut new_cpu = Self {
            pc: START_ADDR,
            ram: [0; RAM_SIZE],
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            v_reg: [0; NUM_REGS],
            i_reg: 0,
            sp: 0,
            stack: [0; STACK_SIZE],
            keys: [false; NUM_KEYS],
            dt: 0,
            st: 0,
        };

        // Copy the FONTSET into the RAM of the CPU
        new_cpu.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);

        new_cpu
    }

    // Reset the CPU to the original state
    pub fn reset(&mut self) {
        self.pc = START_ADDR;
        self.ram = [0; RAM_SIZE];
        self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
        self.v_reg = [0; NUM_REGS];
        self.i_reg = 0;
        self.sp = 0;
        self.stack = [0; STACK_SIZE];
        self.keys = [false; NUM_KEYS];
        self.dt = 0;
        self.st = 0;
        // Load the FONTSET into the inital addresses of the RAM
        self.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }
}

// This block deals all the STACK operations
// TODO: No error handling is added for the underflow and overflow of stack. The same must be implemented
impl CPU {
    fn push(&mut self, val: u16) {
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }
}

// Deals with each CYCLE of the emualator life
impl CPU {
    // Simulates one clock cycle
    pub fn tick(&mut self) {
        let op = self.fetch();
        // Decode
        // Execute
    }

    // Fetch the instruction from the program (which will be loaded into RAM) at the memory address stored in the Program Counter
    fn fetch(&mut self) -> u16 {
        // Each OP Code is just 2 bytes in size
        let higher_byte = self.ram[self.pc as usize] as u16;
        let lower_byte = self.ram[(self.pc + 1) as usize] as u16;
        let op = (higher_byte << 8) | lower_byte;

        // Increment the program counter by two bytes
        self.pc += 2;

        op
    }

    // Work with the two timer flags
    pub fn tick_timers(&mut self) {
        // Delay timer is decremented by 1 in each cycle until it reaches 0 and is ready to be executed
        if self.dt > 0 {
            self.dt -= 1;
        }

        // The sound timer beeps when the same is 1, and is decremented by 1 in each cycle
        if self.st > 0 {
            if self.st == 1 {
                // BEEP! Simulated by a print line statement
                println!("BEEP!");
            }
            self.st -= 1;
        }
    }
}

// Deals with all of the OP Code Execution
impl CPU {
    // Execute the instruction corresponding to a particular hex code
    fn execute(&mut self, op: u16) {
        let d1 = (op & 0xF000) >> 12;
        let d2 = (op & 0x0F00) >> 8;
        let d3 = (op & 0x00F0) >> 4;
        let d4 = op & 0x000F;

        match (d1, d2, d3, d4) {
            // NOP Instruction
            // Do nothing, move onto the next instruction
            (0, 0, 0, 0) => return,

            // CLS Instruction
            // Used to clear the screen, and set all the pixels to 0
            (0, 0, 0xE, 0) => {
                self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
            }

            // Return from Subroutine Instruction
            // When entering a subroutine, we push the address onto the stack and then run the routine's code
            // To return, we pop that value off our stack and execute from that point again
            (0, 0, 0xE, 0xE) => {
                let ret_addr = self.pop();
                self.pc = ret_addr;
            }

            // JMP Instruction
            // Used to jump to a particular instruction
            // Only the most significant digit needs to be set, the rest are used as operand (specify with instruction to jump to)
            (1, _, _, _) => {
                let nnn = op & 0xFFF;
                self.pc = nnn;
            }

            // CALL Instruction
            // Used to enter a subroutine
            // The current value is stored in the stack, and the jump to the adress provided by the last 3 digits is made
            (2, _, _, _) => {
                let nnn = op & 0xFFF;
                self.push(self.pc);
                self.pc = nnn;
            }

            // Match all the left cases that have not been handled yet
            (_, _, _, _) => unimplemented!("Unimplemented opcode: {}", op),
        }
    }
}
