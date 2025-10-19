# CHIP-8 Web Emulator

A web-based CHIP-8 emulator built with Vue.js and WebAssembly.

## Prerequisites

- [Node.js](https://nodejs.org/) (v16 or higher)
- [Rust](https://rustup.rs/) and Cargo
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

## Installation

1. Install wasm-pack if you haven't already:
```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

2. Install Node.js dependencies:
```bash
npm install
```

3. Build the WASM module:
```bash
npm run wasm:build
```

4. (Optional) Set up built-in ROM library:
```bash
./setup-roms.sh
```

This will copy CHIP-8 ROMs from the adjacent `chip8-roms` repository and create a ROM catalog. If you don't have the chip8-roms repository, you can still use the emulator by uploading your own ROM files.

## Development

Start the development server:
```bash
npm run dev
```

Then open your browser to the URL shown in the terminal (usually `http://localhost:5173`).

## Building for Production

1. Build the WASM module:
```bash
npm run wasm:build
```

2. Build the web app:
```bash
npm run build
```

The production files will be in the `dist/` directory.

## Usage

### Playing ROMs from the Library

If you've run `setup-roms.sh`, you'll have access to a built-in library of 99+ CHIP-8 ROMs:

1. The **ROM Library** tab is selected by default
2. Browse through **Games**, **Demos**, or **Programs** tabs
3. Use the search bar to find specific ROMs
4. Click on any ROM to load and play it instantly
5. Click "Start" to begin emulation
6. Use your keyboard to play:
   - `1234` / `QWER` / `ASDF` / `ZXCV` map to the CHIP-8 hex keypad
7. Adjust speed with the slider if needed
8. Use "Pause" to pause and "Reset" to reset the emulator

### Uploading Your Own ROMs

1. Click the **Upload File** tab
2. Click "Choose a ROM file" to load a CHIP-8 ROM (.ch8 file)
3. Follow steps 5-8 above

## Finding More CHIP-8 ROMs

You can find additional public domain CHIP-8 ROMs at:
- [CHIP-8 Games Pack](https://www.zophar.net/pdroms/chip8.html)
- [CHIP-8 Archive](https://johnearnest.github.io/chip8Archive/)

## Deployment

This project is ready to deploy to platforms like Vercel, Netlify, or any static hosting service.

### Quick Deploy to Vercel

```bash
npm i -g vercel
vercel
```

Or connect your GitHub repository to [Vercel](https://vercel.com) for automatic deployments.

### Quick Deploy to Netlify

```bash
npm i -g netlify-cli
netlify deploy --prod
```

Or connect your GitHub repository to [Netlify](https://netlify.com) for automatic deployments.

**See [DEPLOYMENT.md](DEPLOYMENT.md) for detailed deployment instructions.**

### What's Included

The repository includes pre-built files for easy deployment:
- ✅ WASM binaries (~56KB)
- ✅ ROM library (~408KB, 99 ROMs)
- ✅ Platform configs (`vercel.json`, `netlify.toml`)

No Rust toolchain required for deployment!

## Architecture

- **WASM Module** (`wasm_frontend/`): Rust code compiled to WebAssembly using wasm-bindgen
- **Vue.js App** (`web/`): Modern reactive UI built with Vue 3 and Vite
- **Core Emulator** (`chip_core/`): Shared Rust emulator logic used by both web and desktop versions
