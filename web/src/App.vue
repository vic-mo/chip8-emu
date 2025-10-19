<template>
  <div class="app">
    <header>
      <h1>CHIP-8 Emulator</h1>
      <p class="subtitle">CHIP-8 Emulator on the Web</p>
    </header>

    <main class="container">
      <div v-if="!isLoaded" class="loading">
        <div class="spinner"></div>
        <p>Loading emulator...</p>
      </div>

      <template v-else>
        <RomSelector @rom-loaded="handleRomLoad">
          <template #library>
            <RomLibrary @rom-selected="handleLibraryRomSelect" />
          </template>
        </RomSelector>

        <Display
          :displayBuffer="displayBuffer"
          :width="screenWidth"
          :height="screenHeight"
          :scale="10"
          :isRunning="isRunning"
        />

        <Controls
          :romLoaded="romLoaded"
          :isRunning="isRunning"
          :speed="ticksPerFrame"
          @start="start"
          @pause="pause"
          @reset="reset"
          @update:speed="ticksPerFrame = $event"
        />

        <Keypad :pressedKeys="pressedKeys" />
      </template>
    </main>

    <footer>
      <p>
        Built with Vue.js + WebAssembly
      </p>
    </footer>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue'
import { useEmulator } from './composables/useEmulator'
import RomSelector from './components/RomSelector.vue'
import RomLibrary from './components/RomLibrary.vue'
import Display from './components/Display.vue'
import Controls from './components/Controls.vue'
import Keypad from './components/Keypad.vue'

const {
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
} = useEmulator()

const displayBuffer = ref([])
const screenWidth = ref(64)
const screenHeight = ref(32)
const pressedKeys = ref(new Set())

// Keyboard mapping (keyboard key to CHIP-8 hex key)
const keyMap = {
  '1': 0x1, '2': 0x2, '3': 0x3, '4': 0xC,
  'q': 0x4, 'w': 0x5, 'e': 0x6, 'r': 0xD,
  'a': 0x7, 's': 0x8, 'd': 0x9, 'f': 0xE,
  'z': 0xA, 'x': 0x0, 'c': 0xB, 'v': 0xF
}

// Handle keyboard input
const handleKeyDown = (event) => {
  const key = event.key.toLowerCase()
  if (key in keyMap) {
    event.preventDefault()
    const chip8Key = keyMap[key]
    keyPress(chip8Key, true)
    pressedKeys.value.add(chip8Key)
  }
}

const handleKeyUp = (event) => {
  const key = event.key.toLowerCase()
  if (key in keyMap) {
    event.preventDefault()
    const chip8Key = keyMap[key]
    keyPress(chip8Key, false)
    pressedKeys.value.delete(chip8Key)
  }
}

// Load ROM handler (for file uploads)
const handleRomLoad = async (file) => {
  try {
    await loadRom(file)
    updateDisplay()
  } catch (error) {
    console.error('Failed to load ROM:', error)
    alert(`Failed to load ROM: ${error.message}`)
  }
}

// Load ROM from library
const handleLibraryRomSelect = async (rom) => {
  try {
    // Fetch the ROM file from the public directory
    const response = await fetch(`/roms/${rom.file}`)
    if (!response.ok) {
      throw new Error(`Failed to fetch ROM: ${response.statusText}`)
    }

    const arrayBuffer = await response.arrayBuffer()
    const blob = new Blob([arrayBuffer])
    const file = new File([blob], rom.name, { type: 'application/octet-stream' })

    await loadRom(file)
    updateDisplay()
    start()
    console.log(`Loaded ROM: ${rom.name}`)
  } catch (error) {
    console.error('Failed to load ROM from library:', error)
    alert(`Failed to load ROM: ${error.message}`)
  }
}

// Update display buffer from emulator
const updateDisplay = () => {
  const buffer = getDisplay()
  if (buffer) {
    displayBuffer.value = Array.from(buffer)
  }
}

// Display update loop
let displayUpdateInterval = null

const startDisplayUpdate = () => {
  displayUpdateInterval = setInterval(() => {
    if (isRunning.value) {
      updateDisplay()
    }
  }, 16) // ~60 FPS
}

const stopDisplayUpdate = () => {
  if (displayUpdateInterval) {
    clearInterval(displayUpdateInterval)
    displayUpdateInterval = null
  }
}

// Initialize on mount
onMounted(async () => {
  try {
    await init()
    screenWidth.value = getScreenWidth()
    screenHeight.value = getScreenHeight()
    // Initialize display buffer
    displayBuffer.value = new Array(screenWidth.value * screenHeight.value).fill(0)

    // Add keyboard event listeners
    window.addEventListener('keydown', handleKeyDown)
    window.addEventListener('keyup', handleKeyUp)

    // Start display update loop
    startDisplayUpdate()
  } catch (error) {
    console.error('Failed to initialize emulator:', error)
    alert('Failed to initialize emulator. Please refresh the page.')
  }
})

// Cleanup on unmount
onUnmounted(() => {
  window.removeEventListener('keydown', handleKeyDown)
  window.removeEventListener('keyup', handleKeyUp)
  stopDisplayUpdate()
})
</script>

<style>
.app {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}

header {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  padding: 2rem 1rem;
  text-align: center;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.3);
}

h1 {
  font-size: 2.5rem;
  margin: 0;
  color: white;
  text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.3);
}

.subtitle {
  margin: 0.5rem 0 0;
  color: rgba(255, 255, 255, 0.9);
  font-size: 1.1rem;
}

main {
  flex: 1;
  padding: 2rem 1rem;
}

.container {
  max-width: 900px;
  margin: 0 auto;
}

.loading {
  text-align: center;
  padding: 4rem 2rem;
}

.spinner {
  width: 50px;
  height: 50px;
  border: 4px solid #444;
  border-top-color: #4a9eff;
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin: 0 auto 1rem;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

footer {
  background: #2a2a2a;
  padding: 1.5rem;
  text-align: center;
  color: #888;
  font-size: 0.9rem;
  border-top: 1px solid #333;
}

footer a {
  color: #4a9eff;
  text-decoration: none;
}

footer a:hover {
  text-decoration: underline;
}

@media (max-width: 768px) {
  h1 {
    font-size: 1.8rem;
  }

  .subtitle {
    font-size: 0.9rem;
  }
}
</style>
