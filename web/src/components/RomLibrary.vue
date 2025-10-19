<template>
  <div class="rom-library">
    <h2>ROM Library</h2>

    <div v-if="loading" class="loading">Loading ROMs...</div>
    <div v-else-if="error" class="error">{{ error }}</div>

    <template v-else>
      <div v-if="selectedRom" class="selected-rom-info">
        <span class="label">Selected:</span>
        <span class="rom-title">{{ selectedRom.name }}</span>
      </div>

      <div class="search-bar">
        <input
          v-model="searchQuery"
          type="text"
          placeholder="Search ROMs..."
          class="search-input"
        />
      </div>

      <div class="tabs">
        <button
          v-for="category in categories"
          :key="category.id"
          @click="activeCategory = category.id"
          :class="['tab', { active: activeCategory === category.id }]"
        >
          {{ category.label }} ({{ filteredRomsByCategory(category.id).length }})
        </button>
      </div>

      <div class="rom-list">
        <div
          v-for="rom in filteredRoms"
          :key="rom.file"
          @click="selectRom(rom)"
          :class="['rom-item', { selected: selectedRom && selectedRom.file === rom.file }]"
        >
          <div class="rom-name">{{ rom.name }}</div>
          <div class="rom-category">{{ rom.category }}</div>
        </div>
        <div v-if="filteredRoms.length === 0" class="no-results">
          No ROMs found
        </div>
      </div>
    </template>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'

const emit = defineEmits(['rom-selected'])

const catalog = ref(null)
const loading = ref(true)
const error = ref('')
const searchQuery = ref('')
const activeCategory = ref('games')
const selectedRom = ref(null)

const categories = [
  { id: 'games', label: 'Games' },
  { id: 'demos', label: 'Demos' },
  { id: 'programs', label: 'Programs' }
]

const filteredRomsByCategory = (categoryId) => {
  if (!catalog.value) return []
  const roms = catalog.value[categoryId] || []

  if (!searchQuery.value) return roms

  const query = searchQuery.value.toLowerCase()
  return roms.filter(rom =>
    rom.name.toLowerCase().includes(query)
  )
}

const filteredRoms = computed(() => {
  return filteredRomsByCategory(activeCategory.value)
})

const selectRom = (rom) => {
  selectedRom.value = rom
  emit('rom-selected', rom)
}

onMounted(async () => {
  try {
    const response = await fetch('/roms/catalog.json')
    if (!response.ok) {
      throw new Error('Failed to load ROM catalog')
    }
    catalog.value = await response.json()
    loading.value = false
  } catch (err) {
    error.value = err.message
    loading.value = false
  }
})
</script>

<style scoped>
.rom-library {
  background: #2a2a2a;
  padding: 1.5rem;
  border-radius: 8px;
  margin-bottom: 1.5rem;
}

h2 {
  font-size: 1.2rem;
  margin-bottom: 1rem;
  color: #fff;
}

.loading,
.error {
  padding: 2rem;
  text-align: center;
  color: #aaa;
}

.error {
  color: #ff4444;
}

.selected-rom-info {
  background: #3a3a3a;
  padding: 0.75rem 1rem;
  border-radius: 6px;
  margin-bottom: 1rem;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.selected-rom-info .label {
  color: #888;
  font-size: 0.9rem;
  font-weight: 500;
}

.selected-rom-info .rom-title {
  color: #4a9eff;
  font-weight: 600;
  font-size: 1rem;
}

.search-bar {
  margin-bottom: 1rem;
}

.search-input {
  width: 100%;
  padding: 0.75rem;
  background: #1a1a1a;
  border: 1px solid #444;
  border-radius: 6px;
  color: #e0e0e0;
  font-size: 1rem;
}

.search-input:focus {
  outline: none;
  border-color: #4a9eff;
}

.tabs {
  display: flex;
  gap: 0.5rem;
  margin-bottom: 1rem;
  border-bottom: 2px solid #444;
}

.tab {
  padding: 0.75rem 1.5rem;
  background: transparent;
  border: none;
  color: #aaa;
  cursor: pointer;
  font-size: 1rem;
  font-weight: 500;
  border-bottom: 2px solid transparent;
  margin-bottom: -2px;
  transition: all 0.2s;
}

.tab:hover {
  color: #e0e0e0;
}

.tab.active {
  color: #4a9eff;
  border-bottom-color: #4a9eff;
}

.rom-list {
  max-height: 400px;
  overflow-y: auto;
  border: 1px solid #444;
  border-radius: 6px;
}

.rom-item {
  padding: 0.75rem 1rem;
  border-bottom: 1px solid #333;
  cursor: pointer;
  transition: background 0.2s;
}

.rom-item:last-child {
  border-bottom: none;
}

.rom-item:hover {
  background: #3a3a3a;
}

.rom-item.selected {
  background: #4a4a5e;
  border-left: 3px solid #4a9eff;
  padding-left: calc(1rem - 3px);
}

.rom-name {
  color: #e0e0e0;
  font-weight: 500;
  margin-bottom: 0.25rem;
}

.rom-category {
  color: #888;
  font-size: 0.85rem;
  text-transform: capitalize;
}

.no-results {
  padding: 2rem;
  text-align: center;
  color: #888;
}

/* Scrollbar styling */
.rom-list::-webkit-scrollbar {
  width: 8px;
}

.rom-list::-webkit-scrollbar-track {
  background: #1a1a1a;
}

.rom-list::-webkit-scrollbar-thumb {
  background: #555;
  border-radius: 4px;
}

.rom-list::-webkit-scrollbar-thumb:hover {
  background: #666;
}
</style>
