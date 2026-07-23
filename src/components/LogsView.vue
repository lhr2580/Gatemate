<template>
  <div class="logs-view">
    <div class="section-header">
      <h3>调用日志</h3>
      <div class="section-actions">
        <div class="filters">
          <select v-model="localFilter.provider" @change="handleFilterChange" class="filter-select">
            <option value="">全部供应商</option>
            <option v-for="p in providers" :key="p" :value="p">{{ p }}</option>
          </select>
          <select v-model="localFilter.status" @change="handleFilterChange" class="filter-select">
            <option value="">全部状态</option>
            <option value="success">成功</option>
            <option value="failed">失败</option>
          </select>
        </div>
        <button @click="$emit('clear-logs')" class="btn-danger-sm">🗑️ 清空</button>
      </div>
    </div>
    <div class="logs-container">
      <div v-if="callLogs.length === 0" class="empty-state">
        <div class="empty-icon">📝</div>
        <h4>暂无调用记录</h4>
        <p>发送请求后将在此显示</p>
      </div>
      <div v-else class="logs-list">
        <div v-for="log in callLogs" :key="log.id" class="log-item">
          <div class="log-time">{{ formatLogTime(log.created_at) }}</div>
          <div class="log-provider">{{ log.provider }}</div>
          <div class="log-remark">{{ log.remark || '-' }}</div>
          <div class="log-model">{{ log.model }}</div>
          <div class="log-tokens">{{ log.prompt_tokens }} / {{ log.completion_tokens }}</div>
          <div class="log-cost">{{ formatCurrency(log.cost) }}</div>
          <div :class="['log-status', log.status]">
            {{ log.status === 'success' ? '✓' : '✗' }}
          </div>
          <div v-if="log.error_message" class="log-error">{{ log.error_message }}</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';

interface CallLog {
  id: number;
  provider: string;
  remark: string;
  model: string;
  prompt_tokens: number;
  completion_tokens: number;
  cost: number;
  status: string;
  error_message: string | null;
  created_at: string;
}

interface ApiKey {
  provider: string;
}

const props = defineProps<{
  callLogs: CallLog[];
  keys: ApiKey[];
  exchangeRate: number;
  filter: { provider: string; status: string };
}>();

const emit = defineEmits(['filter-change', 'clear-logs']);

const localFilter = ref({ provider: '', status: '' });

watch(() => props.filter, (newVal) => {
  localFilter.value = { ...newVal };
}, { immediate: true, deep: true });

const handleFilterChange = () => {
  emit('filter-change', { ...localFilter.value });
};

const providers = computed(() => {
  const set = new Set(props.keys.map(k => k.provider));
  return Array.from(set);
});

const formatCurrency = (usdAmount: number): string => {
  const cnyAmount = usdAmount * props.exchangeRate;
  return `¥${cnyAmount.toFixed(2)} ($${usdAmount.toFixed(2)})`;
};

const formatLogTime = (dateStr: string) => {
  try {
    const date = new Date(dateStr);
    return date.toLocaleString('zh-CN', { month: 'short', day: 'numeric', hour: '2-digit', minute: '2-digit', second: '2-digit' });
  } catch {
    return dateStr;
  }
};
</script>

<style scoped>
.logs-view {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
  flex-shrink: 0;
}

.section-header h3 { font-size: 1.15rem; margin: 0; color: var(--text-primary); }

.section-actions { display: flex; gap: 14px; align-items: center; }

.filters { display: flex; gap: 8px; }

.filter-select {
  padding: 8px 12px;
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 8px;
  color: #e6edf3;
  font-size: 0.85rem;
  cursor: pointer;
  outline: none;
}

.btn-danger-sm {
  padding: 8px 16px;
  background: rgba(248, 81, 73, 0.1);
  border: 1px solid rgba(248, 81, 73, 0.2);
  border-radius: 8px;
  color: #f85149;
  font-size: 0.85rem;
  cursor: pointer;
  transition: all 0.2s ease;
}

.btn-danger-sm:hover {
  background: rgba(248, 81, 73, 0.2);
  border-color: rgba(248, 81, 73, 0.4);
}

.logs-container {
  flex: 1;
  overflow-y: auto;
  background: var(--bg-card);
  border-radius: 16px;
  border: 1px solid var(--border-color);
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  gap: 12px;
}

.empty-icon { font-size: 3rem; }
.empty-state h4 { font-size: 1rem; margin: 0; color: var(--text-primary); }
.empty-state p { font-size: 0.85rem; color: #8b949e; }

.logs-list {
  padding: 12px;
}

.log-item {
  display: grid;
  grid-template-columns: 120px 80px 1fr 120px 100px 100px 60px;
  gap: 12px;
  padding: 12px 16px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.05);
  font-size: 0.8rem;
  align-items: center;
}

.log-item:last-child { border-bottom: none; }

.log-time { color: #484f58; }
.log-provider { color: #58a6ff; font-weight: 500; }
.log-remark { color: var(--text-primary); white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
.log-model { color: #8b949e; }
.log-tokens { color: #8b949e; font-family: 'Courier New', monospace; }
.log-cost { color: var(--text-primary); font-weight: 500; }

.log-status {
  text-align: center;
  font-weight: 700;
  font-size: 0.9rem;
}

.log-status.success { color: #2ea043; }
.log-status.failed { color: #f85149; }

.log-error {
  grid-column: 1 / -1;
  color: #f85149;
  font-size: 0.75rem;
  margin-top: 4px;
  padding-top: 4px;
  border-top: 1px solid rgba(248, 81, 73, 0.1);
  white-space: pre-wrap;
}

::-webkit-scrollbar { width: 6px; }
::-webkit-scrollbar-track { background: rgba(255, 255, 255, 0.03); }
::-webkit-scrollbar-thumb { background: rgba(255, 255, 255, 0.1); border-radius: 3px; }
::-webkit-scrollbar-thumb:hover { background: rgba(255, 255, 255, 0.2); }

@media (max-width: 1000px) {
  .log-item {
    grid-template-columns: 1fr 80px;
    grid-template-areas:
      "time provider"
      "remark model"
      "tokens cost"
      "status status";
    gap: 6px;
  }
  
  .log-time { grid-area: time; }
  .log-provider { grid-area: provider; text-align: right; }
  .log-remark { grid-area: remark; }
  .log-model { grid-area: model; text-align: right; }
  .log-tokens { grid-area: tokens; }
  .log-cost { grid-area: cost; text-align: right; }
  .log-status { grid-area: status; }
}
</style>