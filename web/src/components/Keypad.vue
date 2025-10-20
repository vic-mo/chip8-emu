<template>
  <div class="keypad">
    <h3>CHIP-8 Keypad</h3>
    <div class="keypad-info">
      Use your keyboard (1234/QWER/ASDF/ZXCV)
    </div>
    <div class="keypad-grid">
      <div
        v-for="key in keypadLayout"
        :key="key.chip8"
        class="key"
        :class="{ active: pressedKeys.has(key.chip8) }"
      >
        <div class="chip8-key">{{ key.label }}</div>
        <div class="keyboard-key">{{ key.keyboard }}</div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'

defineProps({
  pressedKeys: {
    type: Set,
    default: () => new Set()
  }
})

// CHIP-8 keypad layout (hex) mapped to keyboard keys
const keypadLayout = [
  { chip8: 0x1, label: '1', keyboard: '1' },
  { chip8: 0x2, label: '2', keyboard: '2' },
  { chip8: 0x3, label: '3', keyboard: '3' },
  { chip8: 0xC, label: 'C', keyboard: '4' },
  { chip8: 0x4, label: '4', keyboard: 'Q' },
  { chip8: 0x5, label: '5', keyboard: 'W' },
  { chip8: 0x6, label: '6', keyboard: 'E' },
  { chip8: 0xD, label: 'D', keyboard: 'R' },
  { chip8: 0x7, label: '7', keyboard: 'A' },
  { chip8: 0x8, label: '8', keyboard: 'S' },
  { chip8: 0x9, label: '9', keyboard: 'D' },
  { chip8: 0xE, label: 'E', keyboard: 'F' },
  { chip8: 0xA, label: 'A', keyboard: 'Z' },
  { chip8: 0x0, label: '0', keyboard: 'X' },
  { chip8: 0xB, label: 'B', keyboard: 'C' },
  { chip8: 0xF, label: 'F', keyboard: 'V' }
]
</script>

<style scoped>
.keypad {
  background: #2a2a2a;
  padding: 1.5rem;
  border-radius: 8px;
  width: 100%;
}

h3 {
  font-size: 1.1rem;
  margin-bottom: 0.5rem;
  color: #fff;
}

.keypad-info {
  font-size: 0.9rem;
  color: #aaa;
  margin-bottom: 1rem;
}

.keypad-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 0.5rem;
  max-width: 320px;
  margin: 0 auto;
}

.key {
  background: #3a3a3a;
  border: 2px solid #4a4a4a;
  border-radius: 6px;
  padding: 0.75rem;
  text-align: center;
  transition: all 0.15s;
  cursor: default;
  min-height: 60px;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}

.key.active {
  background: #4a9eff;
  border-color: #5aaaff;
  box-shadow: 0 0 12px rgba(74, 158, 255, 0.5);
  transform: scale(0.95);
}

.chip8-key {
  font-size: 1.2rem;
  font-weight: 700;
  color: #fff;
  margin-bottom: 0.25rem;
}

.keyboard-key {
  font-size: 0.75rem;
  color: #888;
  font-weight: 500;
}

.key.active .keyboard-key {
  color: #fff;
}

@media (max-width: 768px) {
  .keypad {
    padding: 1rem;
  }

  .keypad-grid {
    max-width: 100%;
  }
}

@media (max-width: 480px) {
  .keypad {
    padding: 0.75rem;
  }

  h3 {
    font-size: 1rem;
  }

  .keypad-info {
    font-size: 0.85rem;
  }

  .keypad-grid {
    gap: 0.375rem;
  }

  .key {
    padding: 0.5rem;
    min-height: 50px;
  }

  .chip8-key {
    font-size: 1rem;
  }

  .keyboard-key {
    font-size: 0.7rem;
  }
}
</style>
