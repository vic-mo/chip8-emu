use chip_core::{Emulator, SCREEN_HEIGHT, SCREEN_WIDTH};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmEmulator {
    emulator: Emulator,
}

#[wasm_bindgen]
impl WasmEmulator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        WasmEmulator {
            emulator: Emulator::new(),
        }
    }

    /// Load a ROM from a byte array
    #[wasm_bindgen]
    pub fn load_rom(&mut self, rom_data: &[u8]) {
        self.emulator.reset();
        self.emulator.load_game(rom_data);
    }

    /// Execute one instruction cycle
    #[wasm_bindgen]
    pub fn tick(&mut self) {
        self.emulator.tick();
    }

    /// Tick the delay and sound timers
    #[wasm_bindgen]
    pub fn tick_timers(&mut self) {
        self.emulator.tick_timers();
    }

    /// Reset the emulator
    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.emulator.reset();
    }

    /// Set key pressed state
    /// key: 0-15 (hex keypad)
    /// pressed: true if pressed, false if released
    #[wasm_bindgen]
    pub fn key_press(&mut self, key: usize, pressed: bool) {
        self.emulator.key_pressed(key, pressed);
    }

    /// Get the display buffer as a flat array
    /// Returns array of 0s and 1s (64*32 = 2048 elements)
    #[wasm_bindgen]
    pub fn get_display(&self) -> Vec<u8> {
        self.emulator
            .get_display()
            .iter()
            .map(|&pixel| if pixel { 1 } else { 0 })
            .collect()
    }

    /// Get screen width
    #[wasm_bindgen]
    pub fn screen_width(&self) -> usize {
        SCREEN_WIDTH
    }

    /// Get screen height
    #[wasm_bindgen]
    pub fn screen_height(&self) -> usize {
        SCREEN_HEIGHT
    }
}
