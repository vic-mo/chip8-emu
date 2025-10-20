<template>
  <div class="display-container">
    <canvas
      ref="canvas"
      :width="width * scale"
      :height="height * scale"
      class="chip8-display"
    ></canvas>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted, watch } from 'vue'

const props = defineProps({
  displayBuffer: Array,
  width: {
    type: Number,
    default: 64
  },
  height: {
    type: Number,
    default: 32
  },
  scale: {
    type: Number,
    default: 10
  },
  isRunning: Boolean
})

const canvas = ref(null)
let ctx = null
let animationFrameId = null

const draw = () => {
  if (!ctx || !props.displayBuffer) return

  // Clear canvas
  ctx.fillStyle = '#000000'
  ctx.fillRect(0, 0, props.width * props.scale, props.height * props.scale)

  // Draw pixels
  ctx.fillStyle = '#00ff00'
  for (let i = 0; i < props.displayBuffer.length; i++) {
    if (props.displayBuffer[i]) {
      const x = (i % props.width) * props.scale
      const y = Math.floor(i / props.width) * props.scale
      ctx.fillRect(x, y, props.scale, props.scale)
    }
  }
}

const renderLoop = () => {
  draw()
  if (props.isRunning) {
    animationFrameId = requestAnimationFrame(renderLoop)
  }
}

onMounted(() => {
  if (canvas.value) {
    ctx = canvas.value.getContext('2d')
    draw()
  }
})

onUnmounted(() => {
  if (animationFrameId) {
    cancelAnimationFrame(animationFrameId)
  }
})

// Watch for running state changes
watch(() => props.isRunning, (running) => {
  if (running) {
    renderLoop()
  } else {
    if (animationFrameId) {
      cancelAnimationFrame(animationFrameId)
      animationFrameId = null
    }
    // Draw one more time when paused to show current state
    draw()
  }
})

// Also redraw when displayBuffer changes and not running (for initial state)
watch(() => props.displayBuffer, () => {
  if (!props.isRunning) {
    draw()
  }
}, { deep: true })
</script>

<style scoped>
.display-container {
  background: #2a2a2a;
  padding: 1.5rem;
  border-radius: 8px;
  margin-bottom: 1.5rem;
  display: flex;
  justify-content: center;
  align-items: center;
}

.chip8-display {
  border: 2px solid #444;
  border-radius: 4px;
  image-rendering: pixelated;
  image-rendering: crisp-edges;
  max-width: 100%;
  height: auto;
}

@media (max-width: 768px) {
  .display-container {
    padding: 1rem;
  }
}

@media (max-width: 480px) {
  .display-container {
    padding: 0.75rem;
  }
}
</style>
