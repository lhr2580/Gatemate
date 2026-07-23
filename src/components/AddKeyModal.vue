<template>
  <div v-if="show" class="modal-overlay" @click.self="$emit('close')">
    <div class="modal-content">
      <div class="modal-header">
        <h3>{{ isEdit ? '编辑 API Key' : '添加 API Key' }}</h3>
        <button @click="$emit('close')" class="modal-close">✕</button>
      </div>
      <div class="modal-body">
        <div class="form-group">
          <label>供应商</label>
          <select v-model="form.provider" class="form-select" @change="handleProviderChange">
            <option value="" disabled>选择供应商</option>
            <option v-for="p in providers" :key="p" :value="p">{{ p }}</option>
            <option value="other">Other (自定义)</option>
          </select>
          <input 
            v-if="form.provider === 'other'" 
            v-model="form.customProvider" 
            type="text" 
            class="form-input" 
            placeholder="输入自定义供应商名称" 
          />
        </div>
        <div class="form-group">
          <label>
            模型 
            <button @click="$emit('open-model-manager')" class="model-manager-btn">管理</button>
          </label>
          <select v-model="form.model" class="form-select">
            <option value="">选择模型（可选）</option>
            <optgroup v-for="(modelList, provider) in groupedModels" :key="provider" :label="provider">
              <option v-for="model in modelList" :key="model.id" :value="model.name">{{ model.name }}</option>
            </optgroup>
          </select>
        </div>
        <div class="form-group">
          <label>API Key</label>
          <input v-model="form.key" type="password" class="form-input" placeholder="sk-..." />
        </div>
        <div class="form-group">
          <label>备注 (可选)</label>
          <input v-model="form.remark" type="text" class="form-input" placeholder="例如：个人账号" />
        </div>
        <div class="form-row">
          <div class="form-group flex-1">
            <label>日限额 (USD)</label>
            <input v-model.number="form.daily_limit" type="number" step="0.01" class="form-input" />
            <p class="form-hint">设置为 0 表示不限制</p>
          </div>
          <div class="form-group flex-1">
            <label>月限额 (USD)</label>
            <input v-model.number="form.monthly_limit" type="number" step="0.01" class="form-input" />
            <p class="form-hint">设置为 0 表示不限制</p>
          </div>
        </div>
      </div>
      <div class="modal-footer">
        <button @click="$emit('close')" class="btn-secondary">取消</button>
        <button @click="submit" :disabled="!isValid" class="btn-primary">
          {{ isEdit ? '保存' : '添加' }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';

interface ApiKey {
  id: number;
  provider: string;
  remark: string;
  daily_limit: number;
  monthly_limit: number;
}

interface Model {
  id: number;
  name: string;
  provider: string;
  is_enabled: number;
}

const props = defineProps<{
  show: boolean;
  isEdit: boolean;
  editKey: ApiKey | null;
  models: Model[];
}>();

const emit = defineEmits(['close', 'submit', 'open-model-manager']);

const providers = ['OpenAI', 'DeepSeek', 'Doubao', 'Qwen', 'Claude', 'Gemini', 'Groq', 'Ollama'];

const form = ref({
  provider: '',
  customProvider: '',
  model: '',
  key: '',
  remark: '',
  daily_limit: 0,
  monthly_limit: 0,
});

const groupedModels = computed(() => {
  const enabledModels = props.models.filter(m => m.is_enabled === 1);
  const grouped: Record<string, Model[]> = {};
  enabledModels.forEach(model => {
    const provider = model.provider || '其他';
    if (!grouped[provider]) {
      grouped[provider] = [];
    }
    grouped[provider].push(model);
  });
  return grouped;
});

const isValid = computed(() => {
  const provider = form.value.provider === 'other' ? form.value.customProvider : form.value.provider;
  return provider && form.value.key;
});

const handleProviderChange = () => {
  if (form.value.provider !== 'other') {
    form.value.customProvider = '';
  }
};

watch(() => props.show, (newVal) => {
  if (newVal && props.isEdit && props.editKey) {
    form.value = {
      provider: props.editKey.provider,
      customProvider: '',
      model: '',
      key: '****',
      remark: props.editKey.remark,
      daily_limit: props.editKey.daily_limit,
      monthly_limit: props.editKey.monthly_limit,
    };
  } else if (newVal) {
    form.value = {
      provider: '',
      customProvider: '',
      model: '',
      key: '',
      remark: '',
      daily_limit: 0,
      monthly_limit: 0,
    };
  }
});

const submit = () => {
  if (!isValid.value) return;
  const provider = form.value.provider === 'other' ? form.value.customProvider : form.value.provider;
  emit('submit', { ...form.value, provider });
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
  width: 520px;
  max-width: 90vw;
  max-height: 90vh;
  overflow-y: auto;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  border-bottom: 1px solid var(--border-color);
  position: sticky; top: 0; background: var(--bg-card);
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

.modal-body { padding: 20px; }

.form-group { margin-bottom: 16px; }

.form-group label {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 0.85rem;
  color: #8b949e;
  margin-bottom: 8px;
}

.model-manager-btn {
  font-size: 0.75rem;
  padding: 2px 8px;
  background: rgba(88, 166, 255, 0.15);
  border: 1px solid rgba(88, 166, 255, 0.3);
  border-radius: 4px;
  color: #58a6ff;
  cursor: pointer;
  transition: all 0.2s ease;
}

.model-manager-btn:hover {
  background: rgba(88, 166, 255, 0.25);
}

.form-input, .form-select {
  width: 100%;
  padding: 12px;
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 10px;
  color: var(--text-primary);
  font-size: 0.9rem;
  outline: none;
  transition: all 0.2s ease;
}

.form-input:focus, .form-select:focus { border-color: rgba(88, 166, 255, 0.5); }

.form-hint {
  font-size: 0.75rem;
  color: #484f58;
  margin: 6px 0 0 0;
}

.form-row { display: flex; gap: 12px; }
.form-row .form-group { flex: 1; }

.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding: 16px 20px;
  border-top: 1px solid var(--border-color);
}

.btn-secondary {
  padding: 10px 24px;
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 10px;
  color: #8b949e;
  font-size: 0.9rem;
  cursor: pointer;
  transition: all 0.2s ease;
}

.btn-secondary:hover { background: rgba(255, 255, 255, 0.1); color: var(--text-primary); }

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

.btn-primary:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(88, 166, 255, 0.4);
}

.btn-primary:disabled { opacity: 0.5; cursor: not-allowed; }

::-webkit-scrollbar { width: 4px; }
::-webkit-scrollbar-track { background: transparent; }
::-webkit-scrollbar-thumb { background: rgba(255, 255, 255, 0.1); border-radius: 2px; }
</style>
