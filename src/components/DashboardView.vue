<template>
  <div class="dashboard-view">
    <div v-if="isBudgetWarning" class="budget-warning-banner">
      <span class="warning-icon">⚠️</span>
      <span class="warning-text">本月已消耗预算的 80%，请关注。</span>
    </div>
    <div class="stats-row">
      <div class="stat-card">
        <div class="stat-header">
          <span class="stat-icon">📅</span>
          <span class="stat-label">今日花费</span>
        </div>
        <div class="stat-value-row">
          <span class="stat-value">{{ formatCurrency(dailyCost) }}</span>
          <span v-if="dailyChange !== null" :class="['stat-change', dailyChange >= 0 ? 'up' : 'down']">
            {{ dailyChange >= 0 ? '↑' : '↓' }} {{ Math.abs(dailyChange).toFixed(1) }}%
          </span>
        </div>
        <div class="stat-progress">
          <div class="progress-bar" :style="{ width: dailyProgress + '%' }"></div>
        </div>
        <div class="stat-hint">已用 {{ dailyProgress.toFixed(1) }}% / 日限 {{ formatCurrency(totalDailyLimit) }}</div>
      </div>
      <div :class="['stat-card', { 'budget-exhausted': budgetExhausted }]">
        <div class="stat-header">
          <span class="stat-icon">📊</span>
          <span class="stat-label">本月花费</span>
          <button @click="$emit('open-budget-modal')" class="stat-settings-btn" title="预算设置">⚙️</button>
        </div>
        <div class="stat-value-row">
          <span class="stat-value">{{ formatCurrency(monthlyCost) }}</span>
          <span v-if="monthlyChange !== null" :class="['stat-change', monthlyChange >= 0 ? 'up' : 'down']">
            {{ monthlyChange >= 0 ? '↑' : '↓' }} {{ Math.abs(monthlyChange).toFixed(1) }}%
          </span>
        </div>
        <div class="stat-progress">
          <div :class="['progress-bar', { 'budget-warning': isBudgetWarning, 'budget-exhausted': budgetExhausted }]" :style="{ width: monthlyProgress + '%' }"></div>
        </div>
        <div class="stat-hint">
          <span>已用 {{ monthlyProgress.toFixed(1) }}%</span>
          <span v-if="currentProject?.monthly_budget && currentProject.monthly_budget > 0"> / 预算 {{ formatCurrency(currentProject.monthly_budget) }}</span>
          <span v-else> / 月限 {{ formatCurrency(totalMonthlyLimit) }}</span>
        </div>
      </div>
      <div class="stat-card keys-stat-card">
        <div class="stat-header">
          <span class="stat-icon">🔑</span>
          <span class="stat-label">可用 Keys</span>
        </div>
        <div class="keys-main-value">
          <span class="keys-available">{{ activeKeysCount }}</span>
          <span class="keys-divider">/</span>
          <span class="keys-total">{{ keys.length }}</span>
        </div>
        <div class="keys-status-dots">
          <span 
            v-for="key in keys" 
            :key="key.id" 
            :class="['key-status-dot', key.status]"
            :title="key.remark || key.provider"
          ></span>
        </div>
        <div class="keys-details">
          <div class="keys-detail">
            <span class="detail-label">额度使用</span>
            <template v-if="keyUtilization < 0">
              <span class="detail-value infinite">∞ 无限</span>
            </template>
            <template v-else>
              <div class="detail-bar">
                <div :class="['detail-fill', { empty: keyUtilization === 0 }]" :style="{ width: keyUtilization + '%', background: keyUtilizationColor }"></div>
              </div>
              <span class="detail-value">{{ keyUtilization.toFixed(0) }}%</span>
            </template>
          </div>
          <div class="keys-detail">
            <span class="detail-label">已暂停</span>
            <span class="detail-value paused">{{ pausedKeysCount }} 个</span>
          </div>
        </div>
      </div>
    </div>

    <transition name="slide-fade">
      <div v-if="routeStrategy === 'smart'" class="routing-rules-section">
        <div class="section-header">
          <h3>智能路由规则</h3>
          <div class="section-actions">
            <span class="rules-hint">匹配优先级从上到下</span>
          </div>
        </div>
        <div class="routing-rules-list">
          <div v-for="(rule, index) in smartRoutingRules" :key="rule.id" class="routing-rule-card">
            <div class="rule-order">{{ index + 1 }}</div>
            <div class="rule-content">
              <div class="rule-header">
                <span class="rule-name">{{ rule.name }}</span>
                <span :class="['rule-match-type', rule.match_type]">{{ rule.match_type === 'keyword' ? '关键词匹配' : '正则匹配' }}</span>
              </div>
              <div class="rule-detail">
                <span class="rule-match-label">匹配条件:</span>
                <span class="rule-match-content">{{ rule.match_content }}</span>
              </div>
              <div class="rule-detail">
                <span class="rule-target-label">路由至:</span>
                <span class="rule-target-provider">{{ rule.target_provider }}</span>
                <span class="rule-target-model">{{ rule.target_model }}</span>
              </div>
            </div>
            <div class="rule-actions">
              <button class="rule-action-btn" title="编辑">✏️</button>
              <button class="rule-action-btn" title="删除">🗑️</button>
            </div>
          </div>
        </div>
      </div>
    </transition>

    <div class="chart-section">
      <div class="section-header">
        <h3>月度用量趋势</h3>
        <div class="section-actions">
          <div class="time-range">
            <button @click="$emit('update-time-range', '7d')" :class="['range-btn', { active: timeRange === '7d' }]">7天</button>
            <button @click="$emit('update-time-range', '30d')" :class="['range-btn', { active: timeRange === '30d' }]">30天</button>
          </div>
          <div class="export-dropdown">
            <button class="btn-export">📥 导出报表</button>
            <button @click="$emit('toggle-export-dropdown')" class="btn-export-arrow">▼</button>
            <div v-if="toggleExportDropdown" class="export-dropdown-content">
              <button @click="$emit('export-csv')" class="export-dropdown-item">📊 导出 CSV</button>
              <button @click="$emit('export-pdf')" class="export-dropdown-item">📄 导出 PDF</button>
            </div>
          </div>
        </div>
      </div>
      <div class="chart-container">
        <canvas v-if="chartData.length > 0" ref="chartCanvas" class="main-chart"></canvas>
        <div v-else class="chart-empty-state">
          <div class="chart-skeleton">
            <div class="skeleton-line"></div>
            <div class="skeleton-line"></div>
            <div class="skeleton-line"></div>
            <div class="skeleton-line"></div>
            <div class="skeleton-line"></div>
          </div>
          <span class="chart-empty-text">暂无用量数据</span>
          <span class="chart-empty-hint">添加 API Key 并开始调用后，数据将在此显示</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted, nextTick } from 'vue';
import { Chart, registerables } from 'chart.js';

Chart.register(...registerables);

interface ApiKey {
  id: number;
  provider: string;
  remark: string;
  status: string;
  daily_limit: number;
  monthly_limit: number;
}

interface Project {
  id: number;
  name: string;
  monthly_budget: number;
}

interface RoutingRule {
  id: number;
  name: string;
  match_type: string;
  match_content: string;
  target_provider: string;
  target_model: string;
}

const props = defineProps<{
  keys: ApiKey[];
  dailyCost: number;
  monthlyCost: number;
  dailyChange: number | null;
  monthlyChange: number | null;
  chartLabels: string[];
  chartData: number[];
  timeRange: string;
  toggleExportDropdown: boolean;
  routeStrategy: string;
  smartRoutingRules: RoutingRule[];
  exchangeRate: number;
  currentProject: Project | null;
  budgetExhausted: boolean;
}>();

defineEmits(['open-budget-modal', 'update-time-range', 'toggle-export-dropdown', 'export-csv', 'export-pdf']);

const chartCanvas = ref<HTMLCanvasElement | null>(null);
let chartInstance: any = null;

const formatCurrency = (usdAmount: number): string => {
  const cnyAmount = usdAmount * props.exchangeRate;
  return `¥${cnyAmount.toFixed(2)} ($${usdAmount.toFixed(2)})`;
};

const activeKeysCount = computed(() => props.keys.filter(k => k.status === 'active').length);
const pausedKeysCount = computed(() => props.keys.filter(k => k.status !== 'active').length);
const totalDailyLimit = computed(() => props.keys.reduce((sum, k) => sum + k.daily_limit, 0));
const totalMonthlyLimit = computed(() => props.keys.reduce((sum, k) => sum + k.monthly_limit, 0));

const dailyProgress = computed(() => {
  if (totalDailyLimit.value === 0) return 0;
  return Math.min(100, (props.dailyCost / totalDailyLimit.value) * 100);
});

const monthlyProgress = computed(() => {
  if (totalMonthlyLimit.value === 0) return 0;
  return Math.min(100, (props.monthlyCost / totalMonthlyLimit.value) * 100);
});

const keyUtilization = computed(() => {
  if (totalDailyLimit.value === 0) return -1;
  if (props.dailyCost === 0) return 0;
  return Math.min(100, (props.dailyCost / totalDailyLimit.value) * 100);
});

const keyUtilizationColor = computed(() => {
  if (keyUtilization.value < 0) return 'transparent';
  if (keyUtilization.value >= 70) return '#f85149';
  if (keyUtilization.value >= 40) return '#faa31a';
  return '#2ea043';
});

const isBudgetWarning = computed(() => {
  if (!props.currentProject || props.currentProject.monthly_budget <= 0) return false;
  const budgetPercent = (props.monthlyCost / props.currentProject.monthly_budget) * 100;
  return budgetPercent >= 80 && budgetPercent < 100;
});

const renderChart = () => {
  if (!chartCanvas.value) return;
  
  if (chartInstance) {
    chartInstance.destroy();
  }
  
  const ctx = chartCanvas.value.getContext('2d');
  if (!ctx) return;

  const gradient = ctx.createLinearGradient(0, 0, 0, 250);
  gradient.addColorStop(0, 'rgba(88, 166, 255, 0.5)');
  gradient.addColorStop(1, 'rgba(88, 166, 255, 0.05)');

  chartInstance = new Chart(ctx, {
    type: 'line',
    data: {
      labels: props.chartLabels.length > 0 ? props.chartLabels.slice(-7) : ['暂无数据'],
      datasets: [{
        label: '花费 (CNY)',
        data: props.chartData.length > 0 ? props.chartData.slice(-7).map(v => v * props.exchangeRate) : [0],
        borderColor: '#58a6ff',
        backgroundColor: gradient,
        fill: true,
        tension: 0.4,
        pointBackgroundColor: '#58a6ff',
        pointBorderColor: '#fff',
        pointBorderWidth: 2,
        pointRadius: 4,
        pointHoverRadius: 6,
      }]
    },
    options: {
      responsive: true,
      maintainAspectRatio: false,
      interaction: { intersect: false, mode: 'index' },
      plugins: {
        legend: { display: false },
        tooltip: {
          backgroundColor: 'rgba(13, 17, 23, 0.95)',
          titleColor: '#e6edf3',
          bodyColor: '#8b949e',
          borderColor: 'rgba(88, 166, 255, 0.3)',
          borderWidth: 1,
          padding: 12,
          cornerRadius: 10,
          callbacks: {
            label: (context: any) => {
              const cnyValue = context.parsed.y;
              const usdValue = cnyValue / props.exchangeRate;
              return `¥${cnyValue.toFixed(2)} ($${usdValue.toFixed(2)})`;
            }
          }
        }
      },
      scales: {
        x: { grid: { display: false }, ticks: { color: '#8b949e', font: { size: 11 } } },
        y: { 
          grid: { color: 'rgba(255, 255, 255, 0.05)' }, 
          ticks: { color: '#8b949e', font: { size: 11 }, callback: (v: string | number) => `¥${Number(v).toFixed(0)}` } 
        }
      },
      animation: { duration: 800, easing: 'easeOutQuart' }
    }
  });
};

const handleResize = () => {
  if (chartInstance) chartInstance.resize();
};

watch(() => [props.chartLabels, props.chartData], () => {
  nextTick(renderChart);
});

onMounted(() => {
  nextTick(renderChart);
  window.addEventListener('resize', handleResize);
});

onUnmounted(() => {
  window.removeEventListener('resize', handleResize);
  if (chartInstance) chartInstance.destroy();
});
</script>

<style scoped>
.dashboard-view { display: flex; flex-direction: column; gap: 20px; height: 100%; }

.budget-warning-banner {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 12px 20px;
  background: rgba(250, 173, 20, 0.1);
  border: 1px solid rgba(250, 173, 20, 0.2);
  border-radius: 12px;
}

.warning-icon { font-size: 1.2rem; }
.warning-text { font-size: 0.9rem; color: #faa31a; }

.stats-row { display: grid; grid-template-columns: repeat(3, 1fr); gap: 16px; }

.stat-card {
  background: var(--bg-card);
  backdrop-filter: blur(20px);
  border: 1px solid var(--border-color);
  border-radius: 20px;
  padding: 24px;
  position: relative;
  overflow: hidden;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
}

.stat-card::before {
  content: '';
  position: absolute; top: -50%; right: -50%;
  width: 100%; height: 100%;
  background: radial-gradient(circle, rgba(88, 166, 255, 0.05) 0%, transparent 70%);
}

.stat-header { display: flex; align-items: center; gap: 10px; margin-bottom: 16px; }
.stat-icon { font-size: 1.3rem; }
.stat-label { font-size: 0.78rem; color: var(--text-secondary); text-transform: uppercase; letter-spacing: 0.8px; font-weight: 500; }
.stat-value { font-size: 2rem; font-weight: 700; color: var(--accent-color); }
.stat-unit { font-size: 1rem; font-weight: 400; color: var(--text-secondary); }
.stat-settings-btn {
  margin-left: auto;
  width: 28px; height: 28px;
  display: flex; align-items: center; justify-content: center;
  border: none; background: rgba(255, 255, 255, 0.05);
  color: #8b949e; border-radius: 6px;
  cursor: pointer; font-size: 0.8rem;
  transition: all 0.2s ease;
}
.stat-settings-btn:hover { background: rgba(88, 166, 255, 0.15); color: #58a6ff; }

.stat-progress { height: 6px; background: var(--border-color); border-radius: 3px; overflow: hidden; margin-bottom: 10px; }
.progress-bar { height: 100%; background: linear-gradient(90deg, var(--accent-color), #79c0ff); border-radius: 3px; transition: width 0.6s ease; }
.progress-bar.budget-warning { background: linear-gradient(90deg, var(--warning-color), #ffc03d); }
.progress-bar.budget-exhausted { background: linear-gradient(90deg, var(--danger-color), #ff6b6b); }
.stat-hint { font-size: 0.8rem; color: var(--text-secondary); }
.stat-card.budget-exhausted { border-color: rgba(218, 54, 51, 0.3); background: rgba(218, 54, 51, 0.05); }

.stat-value-row {
  display: flex;
  align-items: baseline;
  gap: 10px;
  margin-bottom: 14px;
}

.stat-change {
  font-size: 0.85rem;
  font-weight: 600;
  padding: 4px 8px;
  border-radius: 6px;
}

.stat-change.up {
  color: #f85149;
  background: rgba(218, 54, 51, 0.1);
}

.stat-change.down {
  color: #2ea043;
  background: rgba(46, 160, 67, 0.1);
}

.keys-stat-card {
  display: flex;
  flex-direction: column;
}

.keys-main-value {
  display: flex;
  align-items: baseline;
  gap: 8px;
  margin-bottom: 10px;
}

.keys-available { font-size: 2.8rem; font-weight: 700; color: #2ea043; }
.keys-divider { font-size: 1.2rem; color: #8b949e; }
.keys-total { font-size: 1.2rem; color: #8b949e; }

.keys-status-dots {
  display: flex;
  gap: 6px;
  margin-bottom: 16px;
  flex-wrap: wrap;
}

.key-status-dot {
  width: 10px; height: 10px; border-radius: 50%;
  background: #484f58; cursor: pointer;
  transition: transform 0.2s ease;
}
.key-status-dot:hover { transform: scale(1.3); }
.key-status-dot.active { background: #2ea043; box-shadow: 0 0 6px rgba(46, 160, 67, 0.4); }
.key-status-dot.inactive { background: #faa31a; box-shadow: 0 0 6px rgba(250, 173, 20, 0.4); }

.keys-details {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.keys-detail {
  display: flex;
  align-items: center;
  gap: 10px;
}

.detail-label {
  font-size: 0.78rem;
  color: #8b949e;
  width: 70px;
}

.detail-bar {
  flex: 1;
  height: 6px;
  background: rgba(255, 255, 255, 0.06);
  border-radius: 3px;
  overflow: hidden;
}

.detail-fill {
  height: 100%;
  border-radius: 3px;
  transition: width 0.6s ease;
}

.detail-fill.empty { background: var(--border-color) !important; }
.detail-value.infinite { color: var(--text-secondary); font-style: italic; }
.detail-value { font-size: 0.85rem; color: #e6edf3; width: 50px; text-align: right; }
.detail-value.paused { color: #faa31a; }

.chart-section {
  background: var(--bg-card);
  backdrop-filter: blur(20px);
  border: 1px solid var(--border-color);
  border-radius: 20px;
  padding: 24px;
  flex: 1; min-height: 0;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
}

.section-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px; }
.section-header h3 { font-size: 1.15rem; margin: 0; color: var(--text-primary); font-weight: 600; }

.section-actions { display: flex; gap: 14px; align-items: center; }

.rules-hint { font-size: 0.75rem; color: var(--text-secondary); }

.routing-rules-section {
  margin-top: 24px;
  padding: 20px;
  background: var(--bg-card);
  border-radius: 12px;
  border: 1px solid var(--border-color);
}

.routing-rules-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.routing-rule-card {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 14px;
  background: rgba(0, 0, 0, 0.15);
  border-radius: 8px;
  border: 1px solid var(--border-color);
}

.rule-order {
  width: 24px; height: 24px;
  display: flex; align-items: center; justify-content: center;
  background: rgba(88, 166, 255, 0.15);
  color: #58a6ff;
  border-radius: 4px;
  font-size: 0.75rem; font-weight: 600;
}

.rule-content { flex: 1; }

.rule-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 6px;
}

.rule-name { font-weight: 500; font-size: 0.9rem; }

.rule-match-type {
  font-size: 0.65rem;
  padding: 2px 6px;
  border-radius: 3px;
}

.rule-match-type.keyword { background: rgba(88, 166, 255, 0.15); color: #58a6ff; }
.rule-match-type.regex { background: rgba(250, 173, 20, 0.15); color: #faa31a; }

.rule-detail { display: flex; align-items: center; gap: 6px; font-size: 0.8rem; color: #8b949e; }
.rule-match-label, .rule-target-label { color: #484f58; }
.rule-match-content { color: #e6edf3; }
.rule-target-provider { color: #58a6ff; }
.rule-target-model { color: #e6edf3; }

.rule-actions { display: flex; gap: 8px; }

.rule-action-btn {
  width: 32px; height: 32px;
  display: flex; align-items: center; justify-content: center;
  border: none; background: rgba(255, 255, 255, 0.05);
  color: #8b949e; border-radius: 6px;
  cursor: pointer; font-size: 0.9rem;
  transition: all 0.2s ease;
}

.rule-action-btn:hover { background: rgba(88, 166, 255, 0.15); color: #58a6ff; }

.time-range { display: flex; gap: 4px; }

.range-btn {
  padding: 6px 14px;
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 6px;
  background: rgba(255, 255, 255, 0.05);
  color: #8b949e;
  font-size: 0.8rem;
  cursor: pointer;
  transition: all 0.2s ease;
}

.range-btn:hover { background: rgba(88, 166, 255, 0.1); }
.range-btn.active { background: rgba(88, 166, 255, 0.2); color: #58a6ff; border-color: rgba(88, 166, 255, 0.3); }

.export-dropdown {
  display: flex;
  align-items: center;
  background: rgba(88, 166, 255, 0.1);
  border-radius: 8px;
  overflow: hidden;
}

.btn-export {
  padding: 8px 14px;
  border: none;
  background: transparent;
  color: #58a6ff;
  font-size: 0.85rem;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 6px;
}

.btn-export-arrow {
  padding: 8px 10px;
  border: none;
  background: rgba(88, 166, 255, 0.1);
  color: #58a6ff;
  cursor: pointer;
}

.export-dropdown-content {
  position: absolute;
  top: 100%;
  right: 0;
  margin-top: 8px;
  background: rgba(13, 17, 23, 0.95);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 10px;
  padding: 6px;
  min-width: 160px;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.3);
  z-index: 100;
}

.export-dropdown-item {
  width: 100%;
  padding: 10px 12px;
  border: none;
  background: transparent;
  color: #e6edf3;
  font-size: 0.85rem;
  cursor: pointer;
  border-radius: 6px;
  display: flex;
  align-items: center;
  gap: 8px;
  transition: all 0.2s ease;
}

.export-dropdown-item:hover { background: rgba(88, 166, 255, 0.15); }

.chart-container {
  height: calc(100% - 50px);
  position: relative;
}

.main-chart {
  width: 100%;
  height: 100%;
}

.chart-empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  gap: 12px;
}

.chart-skeleton {
  display: flex;
  flex-direction: column;
  gap: 8px;
  width: 80%;
}

.skeleton-line {
  height: 4px;
  background: linear-gradient(90deg, rgba(88, 166, 255, 0.2), rgba(88, 166, 255, 0.05));
  border-radius: 2px;
  animation: skeletonPulse 1.5s infinite;
}

@keyframes skeletonPulse {
  0%, 100% { opacity: 0.4; }
  50% { opacity: 0.8; }
}

.chart-empty-text { font-size: 0.95rem; color: #8b949e; }
.chart-empty-hint { font-size: 0.78rem; color: #484f58; }

.slide-fade-enter-active {
  transition: all 0.3s ease-out;
}

.slide-fade-leave-active {
  transition: all 0.3s ease-in;
}

.slide-fade-enter-from {
  transform: translateY(-20px);
  opacity: 0;
}

.slide-fade-leave-to {
  transform: translateY(-20px);
  opacity: 0;
}
</style>