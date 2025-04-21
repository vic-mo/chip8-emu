// Standard sprites
const FONTSET_SIZE: usize = 80;
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

// Emulator definition
const RAM_SIZE: usize = 4096;
const NUMBER_OF_REGISTERS: usize = 16;
const NUMBER_OF_KEYS: usize = 16;
const STACK_SIZE: usize = 16;
pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

pub struct Emulator {
    program_counter: u16,
    ram: [u8; RAM_SIZE],
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    registers: [u8; NUMBER_OF_REGISTERS],
    i_register: u16,
    stack_pointer: u16,
    stack: [u16; STACK_SIZE],
    pressed_keys: [bool; NUMBER_OF_KEYS],
    delay_timer: u8,
    sound_timer: u8,
}

// Emulator implementation

const START_ADDR: u16 = 0x200;

impl Emulator {
    pub fn get_display(&self) -> &[bool] {
        &self.screen
    }
    pub fn key_pressed(&mut self, key: usize, pressed: bool) {
        if key < NUMBER_OF_KEYS {
            self.pressed_keys[key] = pressed;
        }
    }

    pub fn new() -> Self {
        let mut new_emulator = Self {
            program_counter: START_ADDR,
            ram: [0; RAM_SIZE],
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            registers: [0; NUMBER_OF_REGISTERS],
            i_register: 0,
            stack_pointer: 0,
            stack: [0; STACK_SIZE],
            pressed_keys: [false; NUMBER_OF_KEYS],
            delay_timer: 0,
            sound_timer: 0,
        };
        new_emulator.load_fontset();
        new_emulator
    }

    fn load_fontset(&mut self) {
        self.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }

    fn push(&mut self, value: u16) {
        if self.stack_pointer < STACK_SIZE as u16 {
            self.stack[self.stack_pointer as usize] = value;
            self.stack_pointer += 1;
        } else {
            panic!("Stack overflow");
        }
    }

    fn pop(&mut self) -> u16 {
        if self.stack_pointer > 0 {
            self.stack_pointer -= 1;
            self.stack[self.stack_pointer as usize]
        } else {
            panic!("Stack underflow");
        }
    }

    pub fn tick(&mut self) {
        // Fetch, decode, and execute the instruction
        let opcode = self.fetch_opcode();
        self.execute_opcode(opcode);
    }

    fn execute_opcode(&mut self, opcode: u16) {
        // Decode and execute the opcode
        let digit1 = (opcode & 0xF000) >> 12;
        let digit2 = (opcode & 0x0F00) >> 8;
        let digit3 = (opcode & 0x00F0) >> 4;
        let digit4 = opcode & 0x000F;
        match (digit1, digit2, digit3, digit4) {
            // NOP
            (0, 0, 0, 0) => return,
            // Clear screeen
            (0, 0, 0xE, 0) => {
                self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
            }
            // Return from subroutine
            (0, 0, 0xE, 0xE) => {
                self.program_counter = self.pop();
            }
            // Jump to address NNN
            (1, _, _, _) => {
                let address_nnn = opcode & 0x0FFF;
                self.program_counter = address_nnn;
            }
            // Call subroutine at NNN
            (2, _, _, _) => {
                let address_nnn = opcode & 0x0FFF;
                self.push(self.program_counter);
                self.program_counter = address_nnn;
            }
            // Skip next instruction if Vx == kk
            (3, _, _, _) => {
                let register_x = digit2 as usize;
                let value_kk = (opcode & 0x00FF) as u8;
                if self.registers[register_x] == value_kk {
                    self.program_counter += 2;
                }
            }
            // Skip next instruction if Vx != kk
            (4, _, _, _) => {
                let register_x = digit2 as usize;
                let value_kk = (opcode & 0x00FF) as u8;
                if self.registers[register_x] != value_kk {
                    self.program_counter += 2;
                }
            }
            // Skip next instruction if Vx == Vy
            (5, _, _, 0) => {
                let register_x = digit2 as usize;
                let register_y = digit3 as usize;
                if self.registers[register_x] == self.registers[register_y] {
                    self.program_counter += 2;
                }
            }
            // Set Vx = kk
            (6, _, _, _) => {
                let register_x = digit2 as usize;
                let value_kk = (opcode & 0x00FF) as u8;
                self.registers[register_x] = value_kk;
            }
            // Add kk to Vx
            (7, _, _, _) => {
                let register_x = digit2 as usize;
                let value_kk = (opcode & 0x00FF) as u8;
                self.registers[register_x] = self.registers[register_x].wrapping_add(value_kk);
            }
            // Set Vx = Vy
            (8, _, _, 0) => {
                let register_x = digit2 as usize;
                let register_y = digit3 as usize;
                self.registers[register_x] = self.registers[register_y];
            }
            // Set Vx = Vx OR Vy
            (8, _, _, 1) => {
                let register_x = digit2 as usize;
                let register_y = digit3 as usize;
                self.registers[register_x] |= self.registers[register_y];
            }
            // Set Vx = Vx AND Vy
            (8, _, _, 2) => {
                let register_x = digit2 as usize;
                let register_y = digit3 as usize;
                self.registers[register_x] &= self.registers[register_y];
            }
            // Set Vx = Vx XOR Vy
            (8, _, _, 3) => {
                let register_x = digit2 as usize;
                let register_y = digit3 as usize;
                self.registers[register_x] ^= self.registers[register_y];
            }
            // Set Vx = Vx + Vy, set VF = carry
            (8, _, _, 4) => {
                let register_x = digit2 as usize;
                let register_y = digit3 as usize;
                let (result, carry) =
                    self.registers[register_x].overflowing_add(self.registers[register_y]);
                self.registers[register_x] = result;
                self.registers[0xF] = if carry { 1 } else { 0 };
            }
            // Set Vx = Vx - Vy, set VF = NOT borrow
            (8, _, _, 5) => {
                let register_x = digit2 as usize;
                let register_y = digit3 as usize;
                let (result, borrow) =
                    self.registers[register_x].overflowing_sub(self.registers[register_y]);
                self.registers[register_x] = result;
                self.registers[0xF] = if borrow { 0 } else { 1 };
            }
            // VX >>= 1
            (8, _, _, 6) => {
                let x = digit2 as usize;
                let lsb = self.registers[x] & 1;
                self.registers[x] >>= 1;
                self.registers[0xF] = lsb;
            }
            // Set Vx = Vy - Vx, set VF = NOT borrow
            (8, _, _, 7) => {
                let register_x = digit2 as usize;
                let register_y = digit3 as usize;
                let (result, borrow) =
                    self.registers[register_y].overflowing_sub(self.registers[register_x]);
                self.registers[register_x] = result;
                self.registers[0xF] = if borrow { 0 } else { 1 };
            }
            // VX <<= 1
            (8, _, _, 0xE) => {
                let x = digit2 as usize;
                let msb = (self.registers[x] >> 7) & 1;
                self.registers[x] <<= 1;
                self.registers[0xF] = msb;
            }
            (9, _, _, 0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                if self.registers[x] != self.registers[y] {
                    self.program_counter += 2;
                }
            }
            // Set I = NNN
            (0xA, _, _, _) => {
                let address_nnn = opcode & 0x0FFF;
                self.i_register = address_nnn;
            }
            // Jump to V0 + NNN
            (0xB, _, _, _) => {
                let address_nnn = opcode & 0x0FFF;
                self.program_counter = self.registers[0] as u16 + address_nnn;
            }
            // Set Vx = random byte AND kk
            (0xC, _, _, _) => {
                let register_x = digit2 as usize;
                let value_kk = (opcode & 0x00FF) as u8;
                self.registers[register_x] = rand::random::<u8>() & value_kk;
            }
            // Display N pixels starting at coordinate (Vx, Vy)
            (0xD, _, _, _) => {
                let register_x = digit2 as usize;
                let register_y = digit3 as usize;
                // how many rows high our sprite is
                let num_rows = digit4;
                let x_coord = self.registers[register_x] as u16;
                let y_coord = self.registers[register_y] as u16;
                // Keep track if any pixels were flipped
                let mut flipped = false;

                for y_line in 0..num_rows {
                    let sprite_row = self.ram[(self.i_register + y_line as u16) as usize];
                    for x_line in 0..8 {
                        // Use a mask to fetch current pixel's bit. Only flip if a 1
                        if (sprite_row & (0x80 >> x_line)) != 0 {
                            let x = (x_coord + x_line) as usize % SCREEN_WIDTH;
                            let y = (y_coord + y_line) as usize % SCREEN_HEIGHT;
                            // Get pixel's index for the 1D screen array
                            let pixel_idx = x + SCREEN_WIDTH * y;
                            // Check if we're about to flip the pixel and set
                            flipped |= self.screen[pixel_idx];
                            self.screen[pixel_idx] ^= true;
                        }
                    }
                }
                // if any pixel is flipped from white to black or vice versa, the VF is set (and cleared otherwise)
                if flipped {
                    self.registers[0xF] = 1;
                } else {
                    self.registers[0xF] = 0;
                }
            }
            // Skip next instruction if key with value Vx is pressed
            (0xE, _, 0x9, 0xE) => {
                let register_x = digit2 as usize;
                let key = self.registers[register_x];
                if self.pressed_keys[key as usize] {
                    self.program_counter += 2;
                }
            }
            // Skip next instruction if key with value Vx is not pressed
            (0xE, _, 0xA, 0x1) => {
                let register_x = digit2 as usize;
                let key = self.registers[register_x];
                if !self.pressed_keys[key as usize] {
                    self.program_counter += 2;
                }
            }
            // Set Vx = delay timer value
            (0xF, _, 0x0, 0x7) => {
                let register_x = digit2 as usize;
                self.registers[register_x] = self.delay_timer;
            }
            // Wait for a key press, store the value of the key in Vx
            (0xF, _, 0x0, 0xA) => {
                let register_x = digit2 as usize;
                let mut pressed = false;
                for (i, &key) in self.pressed_keys.iter().enumerate() {
                    if key {
                        self.registers[register_x] = i as u8;
                        pressed = true;
                        break;
                    }
                }
                if !pressed {
                    // Redo opcode
                    self.program_counter -= 2;
                }
            }
            // Set delay timer = Vx
            (0xF, _, 0x1, 0x5) => {
                let register_x = digit2 as usize;
                self.delay_timer = self.registers[register_x];
            }
            // Set sound timer = Vx
            (0xF, _, 0x1, 0x8) => {
                let register_x = digit2 as usize;
                self.sound_timer = self.registers[register_x];
            }
            // Add Vx to I
            (0xF, _, 0x1, 0xE) => {
                let x = digit2 as usize;
                let vx = self.registers[x] as u16;
                self.i_register = self.i_register.wrapping_add(vx);
            }
            // Set I = location of sprite for digit Vx
            (0xF, _, 0x2, 0x9) => {
                let register_x = digit2 as usize;
                // As all of our font sprites take up five bytes each, their RAM address is their value times 5
                self.i_register = (self.registers[register_x] as u16) * 5;
            }
            // Store BCD representation of Vx in memory locations I, I+1, and I+2
            (0xF, _, 0x3, 0x3) => {
                let register_x = digit2 as usize;
                let value = self.registers[register_x];
                self.ram[self.i_register as usize] = value / 100;
                self.ram[(self.i_register + 1) as usize] = (value / 10) % 10;
                self.ram[(self.i_register + 2) as usize] = value % 10;
            }
            // Store registers V0 to Vx in memory starting at address I
            (0xF, _, 0x5, 0x5) => {
                let register_x = digit2 as usize;
                for i in 0..=register_x {
                    self.ram[self.i_register as usize + i] = self.registers[i];
                }
            }
            // Read registers V0 to Vx from memory starting at address I
            (0xF, _, 0x6, 0x5) => {
                let register_x = digit2 as usize;
                for i in 0..=register_x {
                    self.registers[i] = self.ram[self.i_register as usize + i];
                }
            }
            (_, _, _, _) => unimplemented!("Unimplemented opcode: {:#X}", opcode),
        }
    }

    fn fetch_opcode(&mut self) -> u16 {
        // all chip-8 opcodes are 2 bytes
        let high_byte = self.ram[self.program_counter as usize];
        let low_byte = self.ram[(self.program_counter + 1) as usize];
        let op = (high_byte as u16) << 8 | (low_byte as u16);
        self.program_counter += 2; // Move to the next instruction
        op
    }

    pub fn tick_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                // Play sound
            }
            self.sound_timer -= 1;
        }
    }

    pub fn reset(&mut self) {
        self.program_counter = START_ADDR;
        self.ram = [0; RAM_SIZE];
        self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
        self.registers = [0; NUMBER_OF_REGISTERS];
        self.i_register = 0;
        self.stack_pointer = 0;
        self.stack = [0; STACK_SIZE];
        self.pressed_keys = [false; NUMBER_OF_KEYS];
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.load_fontset();
    }
    /// Load a game into the emulator's memory
    pub fn load_game(&mut self, data: &[u8]) {
        for (i, &byte) in data.iter().enumerate() {
            self.ram[START_ADDR as usize + i] = byte;
        }
    }
}
