use rand::random;

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
        self.execute(op);
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
                // TODO: Implement actual sound of a beep here
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
            /*
               0000
               NOP Instruction
               Do nothing, move onto the next instruction
            */
            (0, 0, 0, 0) => return,

            /*
                00EO
                CLS Instruction
                Used to clear the screen, and set all the pixels to 0
            */
            (0, 0, 0xE, 0) => {
                self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
            }

            /*
                00EE
                Return from Subroutine Instruction
                When entering a subroutine, we push the address onto the stack and then run the routine's code
                To return, we pop that value off our stack and execute from that point again
            */
            (0, 0, 0xE, 0xE) => {
                let ret_addr = self.pop();
                self.pc = ret_addr;
            }

            /*
                1NNN
                JMP Instruction
                Used to jump to a particular instruction
                Only the most significant digit needs to be set, the rest are used as operand (specify with instruction to jump to)
            */
            (1, _, _, _) => {
                let nnn = op & 0xFFF;
                self.pc = nnn;
            }

            /*
                2NNN
                CALL Instruction
                Used to enter a subroutine
                The current value is stored in the stack, and the jump to the adress provided by the last 3 digits is made
            */
            (2, _, _, _) => {
                let nnn = op & 0xFFF;
                self.push(self.pc);
                self.pc = nnn;
            }

            /*
                3XNN
                SKIP if Equal Instruction
                Conditional instruction
                Used to skip one instruction (2 bytes in PC) if the register V[d2] has the value designated by d3 and d4
            */
            (3, _, _, _) => {
                let x = d2 as usize;
                let nn = (op & 0xFF) as u8;
                if self.v_reg[x] == nn {
                    self.pc += 2;
                }
            }

            /*
                4XNN
                SKIP if Not Equal Instruction
                Conditional instruction
                Used to skip one instruction (2 bytes in PC) if the register V[d2] does not have the value equal designated by d3 and d4
            */
            (4, _, _, _) => {
                let x = d2 as usize;
                let nn = (op & 0xFF) as u8;
                if self.v_reg[x] != nn {
                    self.pc += 2;
                }
            }

            /*
                5XY0
                SKIP if Registers are Equal Instruction
                Conditional instruction
                Used to skip one instruction (2 bytes in PC) if register V[d2] == register V[d3]
            */
            (5, _, _, 0) => {
                let x = d2 as usize;
                let y = d3 as usize;
                if self.v_reg[x] == self.v_reg[y] {
                    self.pc += 2;
                }
            }

            /*
                6XNN
                Set Register Instruction
                Set register V[d2] equal to the value desginated by d3 and d4
            */
            (6, _, _, _) => {
                let x = d2 as usize;
                let nn = (op & 0xFF) as u8;
                self.v_reg[x] = nn;
            }

            /*
                7XNN
                Increment Register Instruction
                Increment register V[d2] by the value desginated by d3 and d4
            */
            (7, _, _, _) => {
                let x = d2 as usize;
                let nn = (op & 0xFF) as u8;
                // The wrapping add method is used to prevent Rust from panicking if an overflow occurs, and to wrap around
                // No overflow flag is used by this instruction
                self.v_reg[x] = self.v_reg[x].wrapping_add(nn);
            }

            /*
                8XY0
                Set Register Instruction
                Set register V[d2] = register V[d3]
            */
            (8, _, _, 0) => {
                let x = d2 as usize;
                let y = d3 as usize;
                self.v_reg[x] = self.v_reg[y];
            }

            /*
                8XY1, 8XY2, 8XY3
                Bitwise OR
                Set V[d2] = V[d2] | V[d3]
            */
            (8, _, _, 1) | (8, _, _, 2) | (8, _, _, 3) => {
                let x = d2 as usize;
                let y = d3 as usize;
                self.v_reg[x] |= self.v_reg[y];
            }

            /*
                8XY4
                Add Registers with Overflow Instruction
                Sets V[d2] = V[d2] + V[d3]
                The last register (V[15]) is used the flag bit, while the others are used as general purpose registers
            */
            (8, _, _, 4) => {
                let x = d2 as usize;
                let y = d3 as usize;

                let (sum, carry) = self.v_reg[x].overflowing_add(self.v_reg[y]);
                let carry = if carry { 1 } else { 0 };

                self.v_reg[x] = sum;
                self.v_reg[0xF] = carry;
            }

            /*
                8XY5
                Subtract Registers with Overflow Instruction
                Sets V[d2] = V[d2] - V[d3]
                The last register (V[15]) is used the flag bit
            */
            (8, _, _, 5) => {
                let x = d2 as usize;
                let y = d3 as usize;

                let (diff, borrow) = self.v_reg[x].overflowing_add(self.v_reg[y]);
                let borrow = if borrow { 1 } else { 0 };

                self.v_reg[x] = diff;
                self.v_reg[0xF] = borrow;
            }

            /*
                8XY6
                Right Shift in Register Instruction
                Sets V[d2] = V[d2] >> 1
                The last register (V[15]) is used the flag bit, which is used to store the dropped off bit
            */
            (8, _, _, 6) => {
                let x = d2 as usize;
                let lsb = self.v_reg[x] & 1;
                self.v_reg[x] >>= 1;
                self.v_reg[0xF] = lsb;
            }

            /*
                8XY7
                Subtract Registers with Overflow Instruction
                Sets V[d2] = V[d3] - V[d2]
                The last register (V[15]) is used the flag bit
            */
            (8, _, _, 7) => {
                let x = d2 as usize;
                let y = d3 as usize;

                let (diff, borrow) = self.v_reg[y].overflowing_sub(self.v_reg[x]);
                let borrow = if borrow { 0 } else { 1 };

                self.v_reg[x] = diff;
                self.v_reg[0xF] = borrow;
            }

            /*
                8XYE
                Right Shift in Register Instruction
                Sets V[d2] = V[d2] >> 1
                The last register (V[15]) is used the flag bit, which is used to store if there was an overflow
            */
            (8, _, _, 0xE) => {
                let x = d2 as usize;
                let msb = (self.v_reg[x] >> 7) & 1;
                self.v_reg[x] <<= 1;
                self.v_reg[0xF] = msb;
            }

            /*
                9XYO
                Skip If Not Equal Instruction
                Skips the next instruction if register V[d2] != register V[d3]
            */
            (9, _, _, 0) => {
                let x = d2 as usize;
                let y = d3 as usize;
                if self.v_reg[x] != self.v_reg[y] {
                    self.pc += 2;
                }
            }

            /*
                ANNN
                Set IR Instruction
                Used to set the value of the Instruction Register === nnn, which will act as a memory pointer to RAM
            */
            (0xA, _, _, _) => {
                let nnn = op & 0xFFF;
                self.i_reg = nnn;
            }

            /*
                BNNN
                Increment Program Counter Instruction
                Sets value of the program counter = Register V[0] + the value corresponding to d2,d3,d4 in the opcode
            */
            (0xB, _, _, _) => {
                let nnn = op & 0xFFF;
                self.pc = (self.v_reg[0] as u16) + nnn;
            }

            /*
                CXNN
                Set Register to Random Value Instrcution (with some bits unset)
                Sets register V[X] = A random value & NN
            */
            (0xC, _, _, _) => {
                let x = d2 as usize;
                let nn = (op & 0xFF) as u8;
                let rng: u8 = random();
                self.v_reg[x] = rng & nn;
            }

            // Match all the left cases that have not been handled yet
            (_, _, _, _) => unimplemented!("Unimplemented OP Code: {}", op),
        }
    }
}

// Implementations to deal with the interaction with the frontend
impl CPU {
    // Send the screen to the frontend
    pub fn get_display(&self) -> &[bool] {
        &self.screen
    }

    // Handle key-presses and store them in the CPU
    // The frontend would map the actual key-presses to the CPU keys
    pub fn keypress(&mut self, idx: usize, pressed: bool) {
        self.keys[idx] = pressed;
    }

    // Copy the contents sent to the RAM of the CPU
    pub fn load(&mut self, data: &[u8]) {
        let start = START_ADDR as usize;
        let end = (START_ADDR as usize) + data.len();
        self.ram[start..end].copy_from_slice(data);
    }
}
