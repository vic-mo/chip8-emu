import { ref, onUnmounted } from 'vue'

export function useEmulator() {
  const emulator = ref(null)
  const isLoaded = ref(false)
  const isRunning = ref(false)
  const romLoaded = ref(false)
  const ticksPerFrame = ref(10)
  let animationFrameId = null
  let lastFrameTime = 0

  // Initialize the WASM emulator
  const init = async () => {
    try {
      // Import the WASM module
      const wasmModule = await import('../wasm/wasm_frontend.js')

      // Initialize the WASM binary
      await wasmModule.default()

      // Create emulator instance
      emulator.value = new wasmModule.WasmEmulator()
      isLoaded.value = true
      console.log('CHIP-8 emulator initialized')
    } catch (error) {
      console.error('Failed to load WASM module:', error)
      throw error
    }
  }

  // Load a ROM file
  const loadRom = async (file) => {
    if (!isLoaded.value) {
      throw new Error('Emulator not initialized')
    }

    return new Promise((resolve, reject) => {
      const reader = new FileReader()

      reader.onload = (e) => {
        try {
          const romData = new Uint8Array(e.target.result)
          emulator.value.load_rom(romData)
          romLoaded.value = true
          console.log(`ROM loaded: ${file.name} (${romData.length} bytes)`)
          resolve()
        } catch (error) {
          reject(error)
        }
      }

      reader.onerror = () => reject(reader.error)
      reader.readAsArrayBuffer(file)
    })
  }

  // Game loop
  const gameLoop = (currentTime) => {
    if (!isRunning.value) return

    // Run at 60 FPS
    const deltaTime = currentTime - lastFrameTime
    if (deltaTime >= 16.67) { // ~60 FPS
      // Execute multiple ticks per frame for speed
      for (let i = 0; i < ticksPerFrame.value; i++) {
        emulator.value.tick()
      }
      emulator.value.tick_timers()
      lastFrameTime = currentTime
    }

    animationFrameId = requestAnimationFrame(gameLoop)
  }

  // Start emulation
  const start = () => {
    if (!romLoaded.value) {
      console.warn('No ROM loaded')
      return
    }
    isRunning.value = true
    lastFrameTime = performance.now()
    animationFrameId = requestAnimationFrame(gameLoop)
  }

  // Pause emulation
  const pause = () => {
    isRunning.value = false
    if (animationFrameId) {
      cancelAnimationFrame(animationFrameId)
      animationFrameId = null
    }
  }

  // Reset emulator
  const reset = () => {
    pause()
    if (emulator.value) {
      emulator.value.reset()
    }
    romLoaded.value = false
  }

  // Handle keyboard input
  const keyPress = (key, pressed) => {
    if (emulator.value) {
      emulator.value.key_press(key, pressed)
    }
  }

  // Get display buffer
  const getDisplay = () => {
    if (emulator.value) {
      return emulator.value.get_display()
    }
    return null
  }

  // Get screen dimensions
  const getScreenWidth = () => {
    return emulator.value ? emulator.value.screen_width() : 64
  }

  const getScreenHeight = () => {
    return emulator.value ? emulator.value.screen_height() : 32
  }

  // Cleanup on unmount
  onUnmounted(() => {
    pause()
  })

  return {
    emulator,
    isLoaded,
    isRunning,
    romLoaded,
    ticksPerFrame,
    init,
    loadRom,
    start,
    pause,
    reset,
    keyPress,
    getDisplay,
    getScreenWidth,
    getScreenHeight
  }
}
