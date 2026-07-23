<template>
  <div v-if="show" class="command-palette-overlay" @click.self="$emit('close')">
    <div class="command-palette">
      <div class="cmd-input-container">
        <span class="cmd-icon">🔍</span>
        <input v-model="searchQuery" type="text" class="cmd-input" placeholder="搜索 Key、执行命令..." />
        <span class="cmd-shortcut">ESC</span>
      </div>
      <div class="cmd-list">
        <div v-for="item in filteredItems" :key="item.id" :class="['cmd-item', { active: selectedIndex === item.index }]" @click="$emit('execute', item)">
          <span class="cmd-item-icon">{{ item.icon }}</span>
          <div class="cmd-item-content">
            <span class="cmd-item-title">{{ item.title }}</span>
            <span v-if="item.subtitle" class="cmd-item-subtitle">{{ item.subtitle }}</span>
          </div>
          <span v-if="item.shortcut" class="cmd-item-shortcut">{{ item.shortcut }}</span>
        </div>
        <div v-if="filteredItems.length === 0" class="cmd-empty">
          <span>未找到匹配结果</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, nextTick, onMounted, onUnmounted } from 'vue';

interface CommandItem {
  id: string;
  index: number;
  icon: string;
  title: string;
  subtitle?: string;
  shortcut?: string;
  action: string;
  payload?: any;
}

const props = defineProps<{
  show: boolean;
  items: CommandItem[];
}>();

const emit = defineEmits(['close', 'execute']);

const searchQuery = ref('');
const selectedIndex = ref(0);

const filteredItems = computed(() => {
  if (!searchQuery.value) return props.items;
  const query = searchQuery.value.toLowerCase();
  return props.items.filter(item => 
    item.title.toLowerCase().includes(query) || 
    (item.subtitle && item.subtitle.toLowerCase().includes(query))
  );
});

watch(() => props.show, (newVal) => {
  if (newVal) {
    searchQuery.value = '';
    selectedIndex.value = 0;
    nextTick(() => {
      const input = document.querySelector('.cmd-input') as HTMLInputElement;
      if (input) input.focus();
    });
  }
});

watch(filteredItems, () => {
  selectedIndex.value = 0;
});

const handleKeydown = (e: KeyboardEvent) => {
  if (!props.show) return;
  
  if (e.key === 'ArrowDown') {
    e.preventDefault();
    selectedIndex.value = Math.min(selectedIndex.value + 1, filteredItems.value.length - 1);
  } else if (e.key === 'ArrowUp') {
    e.preventDefault();
    selectedIndex.value = Math.max(selectedIndex.value - 1, 0);
  } else if (e.key === 'Enter' && filteredItems.value.length > 0) {
    e.preventDefault();
    emit('execute', filteredItems.value[selectedIndex.value]);
  } else if (e.key === 'Escape') {
    emit('close');
  }
};

onMounted(() => {
  window.addEventListener('keydown', handleKeydown);
});

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown);
});
</script>

<style scoped>
.command-palette-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.6);
  backdrop-filter: blur(8px);
  display: flex;
  align-items: flex-start;
  justify-content: center;
  padding-top: 15vh;
  z-index: 2000;
}

.command-palette {
  width: 600px;
  max-width: 90vw;
  background: rgba(13, 17, 23, 0.95);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 16px;
  overflow: hidden;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5);
}

.cmd-input-container {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 16px 20px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.08);
}

.cmd-icon { font-size: 1.1rem; }

.cmd-input {
  flex: 1;
  background: transparent;
  border: none;
  color: var(--text-primary);
  font-size: 1rem;
  outline: none;
}

.cmd-input::placeholder { color: #484f58; }

.cmd-shortcut {
  font-size: 0.75rem;
  color: #484f58;
  padding: 4px 8px;
  background: rgba(255, 255, 255, 0.05);
  border-radius: 4px;
}

.cmd-list {
  max-height: 300px;
  overflow-y: auto;
}

.cmd-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 16px;
  cursor: pointer;
  transition: all 0.15s ease;
}

.cmd-item:hover { background: rgba(88, 166, 255, 0.1); }
.cmd-item.active { background: rgba(88, 166, 255, 0.15); }

.cmd-item-icon { font-size: 1rem; }

.cmd-item-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.cmd-item-title { font-size: 0.9rem; color: var(--text-primary); }
.cmd-item-subtitle { font-size: 0.75rem; color: #8b949e; }

.cmd-item-shortcut {
  font-size: 0.7rem;
  color: #484f58;
  padding: 4px 8px;
  background: rgba(255, 255, 255, 0.05);
  border-radius: 4px;
}

.cmd-empty {
  padding: 24px;
  text-align: center;
  color: #484f58;
  font-size: 0.9rem;
}

::-webkit-scrollbar { width: 4px; }
::-webkit-scrollbar-track { background: transparent; }
::-webkit-scrollbar-thumb { background: rgba(255, 255, 255, 0.1); border-radius: 2px; }
</style>