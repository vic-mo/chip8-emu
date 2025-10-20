<template>
  <div class="rom-selector">
    <div class="mode-toggle">
      <button
        @click="mode = 'library'"
        :class="['toggle-btn', { active: mode === 'library' }]"
      >
        ROM Library
      </button>
      <button
        @click="mode = 'upload'"
        :class="['toggle-btn', { active: mode === 'upload' }]"
      >
        Upload File
      </button>
    </div>

    <div v-if="mode === 'upload'" class="upload-section">
      <h2>Upload ROM</h2>
      <div class="file-input-wrapper">
        <label for="rom-file" class="file-label">
          <span v-if="!fileName">Choose a ROM file (.ch8)</span>
          <span v-else>{{ fileName }}</span>
        </label>
        <input
          id="rom-file"
          type="file"
          accept=".ch8,.bin"
          @change="handleFileSelect"
          ref="fileInput"
        />
      </div>
      <div v-if="error" class="error">{{ error }}</div>
    </div>

    <div v-else>
      <slot name="library"></slot>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'

const emit = defineEmits(['rom-loaded'])

const mode = ref('library')
const fileInput = ref(null)
const fileName = ref('')
const error = ref('')

const handleFileSelect = async (event) => {
  const file = event.target.files[0]
  if (!file) return

  error.value = ''
  fileName.value = file.name

  try {
    emit('rom-loaded', file)
  } catch (err) {
    error.value = `Failed to load ROM: ${err.message}`
    fileName.value = ''
  }
}
</script>

<style scoped>
.rom-selector {
  background: #2a2a2a;
  padding: 1.5rem;
  border-radius: 8px;
  margin-bottom: 1.5rem;
  width: 100%;
}

.mode-toggle {
  display: flex;
  gap: 0.5rem;
  margin-bottom: 1.5rem;
  background: #1a1a1a;
  padding: 0.25rem;
  border-radius: 6px;
}

.toggle-btn {
  flex: 1;
  padding: 0.75rem;
  background: transparent;
  border: none;
  color: #aaa;
  border-radius: 4px;
  cursor: pointer;
  font-weight: 500;
  transition: all 0.2s;
  min-height: 44px;
  font-size: 1rem;
}

.toggle-btn:hover {
  color: #e0e0e0;
}

.toggle-btn.active {
  background: #4a9eff;
  color: white;
}

.upload-section h2 {
  font-size: 1.2rem;
  margin-bottom: 1rem;
  color: #fff;
}

.file-input-wrapper {
  position: relative;
}

input[type="file"] {
  position: absolute;
  opacity: 0;
  width: 0;
  height: 0;
}

.file-label {
  display: inline-block;
  padding: 0.75rem 1.5rem;
  background: #4a9eff;
  color: white;
  border-radius: 6px;
  cursor: pointer;
  font-weight: 500;
  transition: background 0.2s;
}

.file-label:hover {
  background: #3a8eef;
}

.error {
  margin-top: 1rem;
  padding: 0.75rem;
  background: #ff4444;
  color: white;
  border-radius: 6px;
  font-size: 0.9rem;
}

@media (max-width: 768px) {
  .rom-selector {
    padding: 1rem;
  }

  .upload-section h2 {
    font-size: 1.1rem;
  }

  .file-label {
    width: 100%;
    text-align: center;
  }
}

@media (max-width: 480px) {
  .rom-selector {
    padding: 0.75rem;
  }

  .mode-toggle {
    margin-bottom: 1rem;
  }

  .toggle-btn {
    padding: 0.625rem;
    font-size: 0.95rem;
  }

  .upload-section h2 {
    font-size: 1rem;
    margin-bottom: 0.875rem;
  }

  .file-label {
    padding: 0.875rem 1.25rem;
    font-size: 0.95rem;
  }

  .error {
    font-size: 0.85rem;
    padding: 0.625rem;
  }
}
</style>
