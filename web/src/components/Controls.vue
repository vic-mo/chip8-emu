<template>
  <div class="controls">
    <div class="button-group">
      <button
        @click="emit('start')"
        :disabled="!romLoaded || isRunning"
        class="btn btn-success"
      >
        Start
      </button>
      <button
        @click="emit('pause')"
        :disabled="!isRunning"
        class="btn btn-warning"
      >
        Pause
      </button>
      <button
        @click="emit('reset')"
        class="btn btn-danger"
      >
        Reset
      </button>
    </div>

    <div class="speed-control">
      <label for="speed">Speed (Ticks/Frame): {{ speed }}</label>
      <input
        id="speed"
        type="range"
        min="1"
        max="30"
        :value="speed"
        @input="emit('update:speed', parseInt($event.target.value))"
      />
    </div>

    <div class="status">
      <span class="status-indicator" :class="{ active: isRunning }"></span>
      <span>{{ statusText }}</span>
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue'

const props = defineProps({
  romLoaded: Boolean,
  isRunning: Boolean,
  speed: Number
})

const emit = defineEmits(['start', 'pause', 'reset', 'update:speed'])

const statusText = computed(() => {
  if (!props.romLoaded) return 'No ROM loaded'
  if (props.isRunning) return 'Running'
  return 'Paused'
})
</script>

<style scoped>
.controls {
  background: #2a2a2a;
  padding: 1.5rem;
  border-radius: 8px;
  margin-bottom: 1.5rem;
}

.button-group {
  display: flex;
  gap: 0.75rem;
  margin-bottom: 1.5rem;
  flex-wrap: wrap;
}

.btn {
  padding: 0.75rem 1.5rem;
  border: none;
  border-radius: 6px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
  font-size: 1rem;
  min-height: 44px;
  min-width: 80px;
}

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-success {
  background: #4caf50;
  color: white;
}

.btn-success:hover:not(:disabled) {
  background: #45a049;
}

.btn-warning {
  background: #ff9800;
  color: white;
}

.btn-warning:hover:not(:disabled) {
  background: #e68900;
}

.btn-danger {
  background: #f44336;
  color: white;
}

.btn-danger:hover:not(:disabled) {
  background: #da190b;
}

.speed-control {
  margin-bottom: 1.5rem;
}

.speed-control label {
  display: block;
  margin-bottom: 0.5rem;
  color: #e0e0e0;
  font-weight: 500;
}

.speed-control input[type="range"] {
  width: 100%;
  max-width: 300px;
  height: 6px;
  border-radius: 3px;
  background: #444;
  outline: none;
}

.speed-control input[type="range"]::-webkit-slider-thumb {
  appearance: none;
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background: #4a9eff;
  cursor: pointer;
}

.speed-control input[type="range"]::-moz-range-thumb {
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background: #4a9eff;
  cursor: pointer;
  border: none;
}

.status {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  color: #e0e0e0;
  font-weight: 500;
}

.status-indicator {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  background: #666;
  transition: background 0.3s;
}

.status-indicator.active {
  background: #4caf50;
  box-shadow: 0 0 8px #4caf50;
}

@media (max-width: 768px) {
  .controls {
    padding: 1rem;
  }

  .button-group {
    gap: 0.5rem;
  }

  .btn {
    flex: 1;
    min-width: 0;
  }
}

@media (max-width: 480px) {
  .controls {
    padding: 0.75rem;
  }

  .button-group {
    flex-direction: column;
  }

  .btn {
    width: 100%;
    padding: 0.875rem 1rem;
  }

  .speed-control {
    margin-bottom: 1rem;
  }

  .speed-control input[type="range"] {
    max-width: 100%;
  }
}
</style>
