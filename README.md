# CHIP-8 Emulator

A CHIP-8 emulator with both desktop and web frontends.

## Project Structure

- **chip_core/** - Core CHIP-8 emulator logic (Rust)
- **desktop/** - Desktop frontend using SDL2 (Rust)
- **wasm_frontend/** - WebAssembly bindings for web (Rust)
- **web/** - Web frontend using Vue.js (JavaScript)

## Desktop Version

To run the desktop version:

```bash
cd desktop
cargo run PATH_TO_YOUR_ROM
```

Example:
```bash
cargo run roms/pong.ch8
```

## Web Version

The web version runs in your browser using WebAssembly.

**Quick Start:**

1. Install prerequisites (Rust, wasm-pack, Node.js)
2. Build the WASM module:
   ```bash
   cd wasm_frontend
   wasm-pack build --target web
   ```
3. Start the web server:
   ```bash
   cd ../web
   npm install
   npm run dev
   ```
4. Open your browser to the URL shown (usually `http://localhost:5173`)

For detailed setup instructions, see [WEB_SETUP.md](WEB_SETUP.md)

## Controls

Both versions use the same keyboard layout:

```
Keyboard:          CHIP-8 Keypad:
1 2 3 4            1 2 3 C
Q W E R     →      4 5 6 D
A S D F            7 8 9 E
Z X C V            A 0 B F
```

## Finding ROMs

You can find CHIP-8 ROMs at:
- [CHIP-8 Archive](https://johnearnest.github.io/chip8Archive/)
- [Zophar's Domain](https://www.zophar.net/pdroms/chip8.html)
