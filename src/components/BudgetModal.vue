<template>
  <div v-if="show" class="modal-overlay" @click.self="$emit('close')">
    <div class="modal-content">
      <div class="modal-header">
        <h3>预算管理</h3>
        <button @click="$emit('close')" class="modal-close">✕</button>
      </div>
      <div class="modal-body">
        <div class="budget-summary">
          <div class="budget-item">
            <span class="budget-label">本月已花费</span>
            <span class="budget-value">{{ formatCurrency(monthlyCost) }}</span>
          </div>
          <div class="budget-item">
            <span class="budget-label">预算使用</span>
            <span :class="['budget-value', { warning: isOverBudget }]">{{ monthlyProgress.toFixed(1) }}%</span>
          </div>
        </div>
        <div class="form-group">
          <label>月度预算限额 (USD)</label>
          <input v-model.number="budgetInput" type="number" step="0.01" class="form-input" />
          <p class="form-hint">设置为 0 表示不限制</p>
        </div>
      </div>
      <div class="modal-footer">
        <button @click="$emit('close')" class="btn-secondary">取消</button>
        <button @click="saveBudget" class="btn-primary">保存</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';

const props = defineProps<{
  show: boolean;
  monthlyCost: number;
  monthlyLimit: number;
  exchangeRate: number;
}>();

const emit = defineEmits(['close', 'save']);

const budgetInput = ref(props.monthlyLimit);

watch(() => props.monthlyLimit, (newVal) => {
  budgetInput.value = newVal;
});

const monthlyProgress = computed(() => {
  if (props.monthlyLimit <= 0) return 0;
  return Math.min(100, (props.monthlyCost / props.monthlyLimit) * 100);
});

const isOverBudget = computed(() => monthlyProgress.value >= 80);

const formatCurrency = (usdAmount: number): string => {
  const cnyAmount = usdAmount * props.exchangeRate;
  return `¥${cnyAmount.toFixed(2)} ($${usdAmount.toFixed(2)})`;
};

const saveBudget = () => {
  emit('save', budgetInput.value);
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
  width: 420px;
  max-width: 90vw;
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

.modal-body { padding: 20px; }

.budget-summary {
  display: flex;
  justify-content: space-between;
  margin-bottom: 20px;
  padding: 16px;
  background: rgba(255, 255, 255, 0.05);
  border-radius: 12px;
}

.budget-item { display: flex; flex-direction: column; gap: 4px; }
.budget-label { font-size: 0.8rem; color: #8b949e; }
.budget-value { font-size: 1.2rem; font-weight: 600; color: var(--text-primary); }
.budget-value.warning { color: #faa31a; }

.form-group { margin-bottom: 20px; }

.form-group label {
  display: block;
  font-size: 0.85rem;
  color: #8b949e;
  margin-bottom: 8px;
}

.form-input {
  width: 100%;
  padding: 12px;
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 10px;
  color: var(--text-primary);
  font-size: 1rem;
  outline: none;
  transition: all 0.2s ease;
}

.form-input:focus { border-color: rgba(88, 166, 255, 0.5); }

.form-hint {
  font-size: 0.75rem;
  color: #484f58;
  margin: 6px 0 0 0;
}

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

.btn-primary:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(88, 166, 255, 0.4);
}
</style>