<template>
  <div class="keys-view">
    <div class="keys-header">
      <div class="keys-title">
        <h3>API Keys 管理</h3>
        <span class="keys-count">共 {{ keys.length }} 个</span>
      </div>
      <button @click="$emit('open-add-key')" class="btn-primary">
        <span class="btn-icon">➕</span>
        添加 Key
      </button>
    </div>

    <div v-if="keys.length === 0" class="empty-state">
      <div class="empty-icon">🔑</div>
      <h4>暂无 API Key</h4>
      <p>添加一个 API Key 开始使用</p>
      <button @click="$emit('open-add-key')" class="btn-primary">添加 Key</button>
    </div>

    <div v-else class="keys-grid">
      <div v-for="key in keys" :key="key.id" class="key-card">
        <div class="key-card-header">
          <div class="key-provider">
            <span class="provider-icon">{{ getProviderIcon(key.provider) }}</span>
            <span class="provider-name">{{ key.provider }}</span>
          </div>
          <div :class="['key-status', key.status]">
            <span class="status-dot"></span>
            {{ key.status === 'active' ? '可用' : '已停用' }}
          </div>
        </div>
        
        <div class="key-info">
          <div class="key-masked">{{ maskKey(key.key_encrypted) }}</div>
          <div v-if="key.remark" class="key-remark">{{ key.remark }}</div>
          <div class="key-meta">添加于 {{ formatDate(key.created_at) }}</div>
        </div>

        <div class="key-limits">
          <div class="limit-item">
            <span class="limit-label">日限</span>
            <span class="limit-value">{{ formatCurrency(key.daily_limit) }}</span>
            <div class="limit-bar">
              <div class="limit-fill" :style="{ width: getDailyUsagePercent(key) + '%' }"></div>
            </div>
          </div>
          <div class="limit-item">
            <span class="limit-label">月限</span>
            <span class="limit-value">{{ formatCurrency(key.monthly_limit) }}</span>
            <div class="limit-bar">
              <div class="limit-fill" :style="{ width: getMonthlyUsagePercent(key) + '%' }"></div>
            </div>
          </div>
        </div>

        <div class="key-actions">
          <button @click="$emit('edit-key', key)" class="key-action-btn" title="编辑">✏️</button>
          <button @click="$emit('test-key', key)" class="key-action-btn" title="测试">🔄</button>
          <button @click="$emit('toggle-key-status', key)" class="key-action-btn" :class="{ disabled: key.status !== 'active' }" title="停用/启用">
            {{ key.status === 'active' ? '⏸' : '▶' }}
          </button>
          <button @click="$emit('delete-key', key.id)" class="key-action-btn key-action-btn-danger" title="删除">🗑️</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { } from 'vue';

interface ApiKey {
  id: number;
  provider: string;
  key_encrypted: string;
  remark: string;
  status: string;
  daily_limit: number;
  monthly_limit: number;
  created_at: string;
}

const props = defineProps<{
  keys: ApiKey[];
  exchangeRate: number;
  keyUsages: Record<number, { daily: number; monthly: number }>;
}>();

defineEmits(['open-add-key', 'edit-key', 'test-key', 'toggle-key-status', 'delete-key']);

const formatCurrency = (usdAmount: number): string => {
  const cnyAmount = usdAmount * props.exchangeRate;
  return `¥${cnyAmount.toFixed(2)} ($${usdAmount.toFixed(2)})`;
};

const maskKey = (encryptedKey: string) => {
  const len = encryptedKey.length;
  if (len <= 8) return '******';
  return encryptedKey.substring(0, 4) + '********' + encryptedKey.substring(len - 4);
};

const getProviderIcon = (provider: string) => {
  const icons: Record<string, string> = {
    'OpenAI': '🤖', 'DeepSeek': '🔵', 'Doubao': '🐼', 'Qwen': '🦄',
    'Claude': '🤖', 'Gemini': '🌟', 'Groq': '⚡', 'Ollama': '🐑',
  };
  return icons[provider] || '🔑';
};

const formatDate = (dateStr: string) => {
  try {
    const date = new Date(dateStr);
    return date.toLocaleDateString('zh-CN', { month: 'short', day: 'numeric' });
  } catch {
    return dateStr;
  }
};

const getDailyUsagePercent = (key: ApiKey) => {
  const usage = props.keyUsages[key.id]?.daily || 0;
  if (key.daily_limit <= 0) return 0;
  return Math.min(100, (usage / key.daily_limit) * 100);
};

const getMonthlyUsagePercent = (key: ApiKey) => {
  const usage = props.keyUsages[key.id]?.monthly || 0;
  if (key.monthly_limit <= 0) return 0;
  return Math.min(100, (usage / key.monthly_limit) * 100);
};
</script>

<style scoped>
.keys-view {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.keys-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
  flex-shrink: 0;
}

.keys-title { display: flex; align-items: center; gap: 12px; }
.keys-title h3 { font-size: 1.15rem; margin: 0; color: var(--text-primary); }
.keys-count { font-size: 0.85rem; color: #8b949e; }

.btn-primary {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 20px;
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

.btn-icon { font-size: 1rem; }

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  gap: 12px;
}

.empty-icon { font-size: 4rem; }
.empty-state h4 { font-size: 1.1rem; margin: 0; color: var(--text-primary); }
.empty-state p { font-size: 0.9rem; color: #8b949e; }

.keys-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
  gap: 16px;
  overflow-y: auto;
  flex: 1;
}

.key-card {
  background: var(--bg-card);
  backdrop-filter: blur(20px);
  border: 1px solid var(--border-color);
  border-radius: 16px;
  padding: 20px;
  transition: all 0.2s ease;
}

.key-card:hover {
  border-color: rgba(88, 166, 255, 0.3);
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.15);
}

.key-card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.key-provider {
  display: flex;
  align-items: center;
  gap: 8px;
}

.provider-icon { font-size: 1.2rem; }
.provider-name { font-weight: 600; color: var(--text-primary); }

.key-status {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 4px 10px;
  border-radius: 6px;
  font-size: 0.75rem;
}

.key-status.active { background: rgba(46, 160, 67, 0.15); color: #2ea043; }
.key-status.paused { background: rgba(250, 173, 20, 0.15); color: #faa31a; }

.key-status .status-dot {
  width: 6px; height: 6px; border-radius: 50%;
}

.key-status.active .status-dot { background: #2ea043; }
.key-status.paused .status-dot { background: #faa31a; }

.key-info { margin-bottom: 16px; }

.key-masked {
  font-family: 'Courier New', monospace;
  font-size: 0.85rem;
  color: #8b949e;
  letter-spacing: 1px;
  margin-bottom: 6px;
}

.key-remark {
  font-size: 0.9rem;
  color: var(--text-primary);
  margin-bottom: 4px;
}

.key-meta {
  font-size: 0.75rem;
  color: #484f58;
}

.key-limits {
  display: flex;
  flex-direction: column;
  gap: 12px;
  margin-bottom: 16px;
}

.limit-item {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.limit-label {
  font-size: 0.75rem;
  color: #8b949e;
}

.limit-value {
  font-size: 0.85rem;
  color: var(--text-primary);
  font-weight: 500;
}

.limit-bar {
  height: 4px;
  background: rgba(255, 255, 255, 0.06);
  border-radius: 2px;
  overflow: hidden;
}

.limit-fill {
  height: 100%;
  background: linear-gradient(90deg, #58a6ff, #79c0ff);
  border-radius: 2px;
  transition: width 0.6s ease;
}

.key-actions {
  display: flex;
  gap: 8px;
  justify-content: flex-end;
}

.key-action-btn {
  width: 36px; height: 36px;
  display: flex; align-items: center; justify-content: center;
  border: none; background: rgba(255, 255, 255, 0.05);
  color: #8b949e; border-radius: 8px;
  cursor: pointer; font-size: 0.9rem;
  transition: all 0.2s ease;
}

.key-action-btn:hover { background: rgba(88, 166, 255, 0.15); color: #58a6ff; }
.key-action-btn.disabled { opacity: 0.4; cursor: not-allowed; }
.key-action-btn-danger:hover { background: rgba(248, 81, 73, 0.15); color: #f85149; }

::-webkit-scrollbar { width: 6px; }
::-webkit-scrollbar-track { background: rgba(255, 255, 255, 0.03); }
::-webkit-scrollbar-thumb { background: rgba(255, 255, 255, 0.1); border-radius: 3px; }
::-webkit-scrollbar-thumb:hover { background: rgba(255, 255, 255, 0.2); }
</style>