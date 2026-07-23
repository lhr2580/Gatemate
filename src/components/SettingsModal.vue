<template>
  <div v-if="show" class="modal-overlay" @click.self="$emit('close')">
    <div class="modal-content settings-modal">
      <div class="modal-header">
        <h3>设置</h3>
        <button @click="$emit('close')" class="modal-close">✕</button>
      </div>
      <div class="settings-tabs">
        <button @click="settingsTab = 'appearance'" :class="['settings-tab', { active: settingsTab === 'appearance' }]">外观</button>
      </div>
      <div class="modal-body">
        <div v-if="settingsTab === 'appearance'">
          <div class="form-group">
            <label>汇率 (USD → CNY)</label>
            <input v-model.number="localExchangeRate" type="number" step="0.1" class="form-input" />
            <p class="form-hint">当前: 1 USD = {{ localExchangeRate }} CNY</p>
          </div>
          <div class="form-group">
            <label>外观主题</label>
            <div class="theme-options">
              <button @click="$emit('change-theme', 'deep-space')" :class="['theme-btn', { active: currentTheme === 'deep-space' }]">
                <span class="theme-icon">🌙</span>
                <span class="theme-name">深空</span>
              </button>
              <button @click="$emit('change-theme', 'bright-moon')" :class="['theme-btn', { active: currentTheme === 'bright-moon' }]">
                <span class="theme-icon">☀️</span>
                <span class="theme-name">皓月</span>
              </button>
              <button @click="$emit('change-theme', 'aurora')" :class="['theme-btn', { active: currentTheme === 'aurora' }, { locked: !isProUser }]">
                <span class="theme-icon">🌌</span>
                <span class="theme-name">极光</span>
              </button>
            </div>
          </div>
          <div class="form-group">
            <label>服务端口</label>
            <input :value="serverPort" type="number" class="form-input" disabled />
            <p class="form-hint">HTTP API 服务端口: {{ serverPort }}</p>
          </div>
        </div>
      </div>
      <div class="modal-footer">
        <button @click="saveSettings" class="btn-primary">保存设置</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';

const props = defineProps<{
  show: boolean;
  exchangeRate: number;
  currentTheme: string;
  serverPort: number;
  isProUser: boolean;
}>();

const emit = defineEmits(['close', 'change-theme', 'save-settings']);

const settingsTab = ref('appearance');
const localExchangeRate = ref(props.exchangeRate);

watch(() => props.exchangeRate, (newVal) => {
  localExchangeRate.value = newVal;
});

const saveSettings = () => {
  emit('save-settings', { exchangeRate: localExchangeRate.value });
};
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.7);
  backdrop-filter: blur(8px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  background: var(--bg-card);
  border: 1px solid var(--border-color);
  border-radius: 16px;
  min-width: 480px;
  max-width: 90vw;
  max-height: 90vh;
  overflow: hidden;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  border-bottom: 1px solid var(--border-color);
}

.modal-header h3 { font-size: 1.1rem; margin: 0; color: var(--text-primary); }

.modal-close {
  width: 32px; height: 32px;
  display: flex; align-items: center; justify-content: center;
  border: none; background: transparent;
  color: #8b949e;
  cursor: pointer; font-size: 1rem;
  border-radius: 6px;
  transition: all 0.2s ease;
}

.modal-close:hover { background: rgba(255, 255, 255, 0.05); color: var(--text-primary); }

.settings-modal { min-width: 520px; }

.settings-tabs {
  display: flex;
  padding: 0 20px;
  border-bottom: 1px solid var(--border-color);
}

.settings-tab {
  padding: 12px 20px;
  border: none;
  background: transparent;
  color: #8b949e;
  font-size: 0.9rem;
  cursor: pointer;
  border-bottom: 2px solid transparent;
  transition: all 0.2s ease;
}

.settings-tab:hover { color: var(--text-primary); }
.settings-tab.active { color: #58a6ff; border-bottom-color: #58a6ff; }

.modal-body { padding: 20px; }

.form-group { margin-bottom: 20px; }

.form-group label {
  display: block;
  font-size: 0.85rem;
  color: #8b949e;
  margin-bottom: 8px;
}

.form-input {
  width: 100%;
  padding: 10px 12px;
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 8px;
  color: var(--text-primary);
  font-size: 0.9rem;
  outline: none;
  transition: all 0.2s ease;
}

.form-input:focus { border-color: rgba(88, 166, 255, 0.5); }
.form-input:disabled { opacity: 0.5; cursor: not-allowed; }

.form-hint {
  font-size: 0.75rem;
  color: #484f58;
  margin: 6px 0 0 0;
}

.theme-options {
  display: flex;
  gap: 12px;
}

.theme-btn {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 16px;
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 12px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.theme-btn:hover { background: rgba(88, 166, 255, 0.1); }
.theme-btn.active { background: rgba(88, 166, 255, 0.15); border-color: rgba(88, 166, 255, 0.3); }
.theme-btn.locked { opacity: 0.5; cursor: not-allowed; }

.theme-icon { font-size: 1.5rem; }
.theme-name { font-size: 0.85rem; color: var(--text-primary); }

.modal-footer {
  display: flex;
  justify-content: flex-end;
  padding: 16px 20px;
  border-top: 1px solid var(--border-color);
}

.btn-primary {
  padding: 10px 24px;
  background: linear-gradient(135deg, #58a6ff, #79c0ff);
  border: none;
  border-radius: 10px;
  color: #fff;
  font-size: 0.9rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.btn-primary:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(88, 166, 255, 0.4);
}
</style>