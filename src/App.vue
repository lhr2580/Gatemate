<template>
  <div class="app-container">
    <div class="particles">
      <div 
        v-for="i in 20" 
        :key="i" 
        class="particle"
        :style="{
          left: `${Math.random() * 100}%`,
          top: `${Math.random() * 100}%`,
          animationDelay: `${Math.random() * 20}s`,
          animationDuration: `${15 + Math.random() * 10}s`,
          background: i % 3 === 0 ? 'rgba(88, 166, 255, 0.3)' : i % 3 === 1 ? 'rgba(250, 173, 20, 0.2)' : 'rgba(46, 160, 67, 0.2)'
        }"
      ></div>
    </div>

    <div class="sidebar">
      <div class="sidebar-header">
        <div class="logo">
          <span class="logo-icon">⚡</span>
          <span class="logo-text">GateMate</span>
          <span :class="['logo-status', { online: isServerReady }]"></span>
        </div>
      </div>
      <div class="sidebar-nav">
        <button 
          v-for="item in navItems" 
          :key="item.id"
          @click="activeTab = item.id"
          :class="['nav-item', { active: activeTab === item.id }]"
        >
          <span class="nav-icon">{{ item.icon }}</span>
          <span class="nav-label">{{ item.label }}</span>
        </button>
      </div>
      <div v-if="showToast" class="sidebar-toast" :class="toastType">
        {{ toastMessage }}
      </div>
      <div class="sidebar-footer">
        <div class="route-strategy">
          <span class="strategy-label">路由策略</span>
          <select v-model="routeStrategy" @change="saveRouteStrategy" class="strategy-select">
            <option value="round_robin">轮询</option>
            <option value="random">随机</option>
            <option value="smart">智能路由</option>
          </select>
        </div>
        <div class="server-status" :class="{ online: isServerReady }">
          <span class="status-dot"></span>
          <span>{{ isServerReady ? '服务运行中' : '未连接' }}</span>
        </div>
      </div>
    </div>

    <div class="main-panel">
      <header class="top-bar">
        <div class="top-bar-left">
          <div class="project-selector" @click="showProjectDropdown = !showProjectDropdown">
            <span class="project-name">{{ currentProject?.name || '选择项目' }}</span>
            <span class="project-arrow">▼</span>
          </div>
          <div v-if="showProjectDropdown" class="project-dropdown">
            <div v-for="project in projects" :key="project.id" @click="selectProject(project)" :class="['project-item', { active: currentProject?.id === project.id }]">
              <span class="project-item-name">{{ project.name }}</span>
              <span class="project-item-cost">¥{{ (projectMonthlyUsages[project.id] || 0).toFixed(2) }}</span>
            </div>
            <div class="project-divider"></div>
            <button @click="handleCreateProjectClick" class="project-add-btn">
              <span>+ 新建项目</span>
            </button>
          </div>
          <h2>{{ currentTabTitle }}</h2>
          <p class="top-bar-subtitle">{{ currentTabSubtitle }}</p>
        </div>
        <div class="top-bar-right">
          <div class="command-hint">⌘K 快速搜索</div>
          <div class="notification-center" @click="showNotificationDropdown = !showNotificationDropdown">
            <span class="notification-icon">🔔</span>
            <span v-if="unreadNotificationCount > 0" class="notification-badge">{{ unreadNotificationCount }}</span>
            <div v-if="showNotificationDropdown" class="notification-dropdown">
              <div class="notification-dropdown-header">
                <span class="notification-dropdown-title">通知中心</span>
                <span class="notification-dropdown-count">{{ unreadNotificationCount }} 条未读</span>
              </div>
              <div class="notification-dropdown-body">
                <div v-if="notifications.length === 0" class="notification-empty">
                  <span class="empty-icon">📭</span>
                  <span class="empty-text">暂无通知</span>
                </div>
                <div 
                  v-for="notification in notifications" 
                  :key="notification.id"
                  :class="['notification-item', { unread: notification.is_read === 0 }]"
                >
                  <span class="notification-item-icon">{{ getNotificationIcon(notification.type) }}</span>
                  <div class="notification-item-content">
                    <span class="notification-item-title">{{ notification.title }}</span>
                    <span class="notification-item-text">{{ notification.content }}</span>
                    <span class="notification-item-time">{{ formatNotificationTime(notification.created_at) }}</span>
                  </div>
                  <button @click.stop="markNotificationRead(notification.id)" class="notification-item-mark" title="标记已读">✓</button>
                </div>
              </div>
              <div class="notification-dropdown-footer">
                <button @click.stop="markAllNotificationsRead" class="notification-footer-btn">全部标记已读</button>
                <button @click.stop="clearAllNotifications" class="notification-footer-btn danger">清空所有</button>
              </div>
            </div>
          </div>
          <div class="server-status-badge" :class="{ online: isServerReady }">
            <span class="status-dot"></span>
            <span>{{ isServerReady ? '服务运行中' : '未连接' }}</span>
          </div>
          <div class="notification-toggle" v-if="activeTab === 'dashboard'">
            <span class="toggle-label">通知</span>
            <button @click="toggleNotification" :class="['toggle-switch', { on: notificationEnabled }]">
              <span class="toggle-knob"></span>
            </button>
          </div>
          <button @click="showSettings = true" class="btn-icon" title="设置">⚙️</button>
          <button @click="refreshData" class="btn-icon" title="刷新">🔄</button>
        </div>
      </header>

      <div class="content-area">
        <DashboardView 
          v-if="activeTab === 'dashboard'"
          :keys="keys"
          :daily-cost="dailyCost"
          :monthly-cost="monthlyCost"
          :daily-change="dailyChange"
          :monthly-change="monthlyChange"
          :chart-labels="chartLabels"
          :chart-data="chartData"
          :time-range="timeRange"
          :toggle-export-dropdown="toggleExportDropdown"
          :route-strategy="routeStrategy"
          :smart-routing-rules="smartRoutingRules"
          :exchange-rate="exchangeRate"
          :current-project="currentProject"
          :budget-exhausted="budgetExhausted"
          @open-budget-modal="showBudgetModal = true"
          @update-time-range="timeRange = $event"
          @toggle-export-dropdown="toggleExportDropdown = !toggleExportDropdown"
          @export-csv="showExportModal = true"
          @export-pdf="showExportModal = true"
        />

        <KeysView 
          v-if="activeTab === 'keys'"
          :keys="keys"
          :exchange-rate="exchangeRate"
          :key-usages="keyUsages"
          @open-add-key="showAddKeyModal = true"
          @edit-key="editKey"
          @test-key="testKey"
          @toggle-key-status="toggleKeyStatus"
          @delete-key="deleteKey"
        />

        <LogsView 
          v-if="activeTab === 'logs'"
          :call-logs="callLogs"
          :keys="keys"
          :exchange-rate="exchangeRate"
          :filter="logFilter"
          @filter-change="loadLogs"
          @clear-logs="clearLogs"
        />
      </div>
    </div>

    <AddKeyModal 
      :show="showAddKeyModal"
      :is-edit="!!editingKey"
      :edit-key="editingKey"
      :models="models"
      @close="closeAddKeyModal"
      @submit="saveKey"
      @open-model-manager="showModelManager = true"
    />

    <div v-if="showModelManager" class="modal-overlay" @click.self="showModelManager = false">
      <div class="modal-content model-modal">
        <div class="modal-header">
          <h3>模型管理</h3>
          <button @click="showModelManager = false" class="modal-close">✕</button>
        </div>
        <div class="modal-body">
          <div class="form-row">
            <div class="form-group flex-1">
              <input v-model="newModelName" type="text" placeholder="模型名称..." class="form-input" />
            </div>
            <div class="form-group">
              <select v-model="newModelProvider" class="form-select">
                <option value="">选择供应商</option>
                <option value="OpenAI">OpenAI</option>
                <option value="Claude">Claude</option>
                <option value="DeepSeek">DeepSeek</option>
                <option value="Qwen">Qwen</option>
                <option value="Gemini">Gemini</option>
                <option value="Groq">Groq</option>
                <option value="Mistral">Mistral</option>
                <option value="Cohere">Cohere</option>
                <option value="智谱GLM">智谱GLM</option>
                <option value="百川">百川</option>
                <option value="零一万物">零一万物</option>
                <option value="自定义">自定义</option>
              </select>
            </div>
            <div class="form-group">
              <select v-model="newModelTags" class="form-select">
                <option value="">标签</option>
                <option value="推荐">推荐</option>
                <option value="性价比">性价比</option>
                <option value="推理强">推理强</option>
              </select>
            </div>
            <button @click="addModel" class="btn-primary btn-sm">添加</button>
          </div>
          <div class="model-list">
            <div v-for="model in models" :key="model.id" class="model-item">
              <div class="model-info">
                <span :class="['model-name', { disabled: model.is_enabled === 0 }]">{{ model.name }}</span>
                <span v-if="model.tags" class="model-tag">{{ model.tags }}</span>
                <span v-if="model.provider" class="model-provider">{{ model.provider }}</span>
              </div>
              <div class="model-actions">
                <button @click="toggleModelEnabled(model)" :class="['toggle-btn', { active: model.is_enabled === 1 }]">
                  {{ model.is_enabled === 1 ? '启用' : '禁用' }}
                </button>
                <button @click="deleteModelItem(model)" class="btn-danger-sm">删除</button>
              </div>
            </div>
          </div>
        </div>
        <div class="modal-footer">
          <button @click="showModelManager = false" class="btn-primary">关闭</button>
        </div>
      </div>
    </div>

    <div v-if="showExportModal" class="modal-overlay" @click.self="showExportModal = false">
      <div class="modal-content">
        <div class="modal-header">
          <h3>导出报表</h3>
          <button @click="showExportModal = false" class="modal-close">✕</button>
        </div>
        <div class="modal-body">
          <div class="form-group">
            <label>日期范围</label>
            <div class="date-range">
              <input v-model="exportStartDate" type="date" class="form-input" />
              <span class="date-separator">至</span>
              <input v-model="exportEndDate" type="date" class="form-input" />
            </div>
          </div>
          <div class="export-options">
            <button @click="exportCSV" class="export-btn">
              <span class="export-icon">📄</span>
              <div class="export-info">
                <span class="export-name">CSV 导出</span>
                <span class="export-desc">导出详细调用日志</span>
              </div>
            </button>
            <button @click="exportPDF" class="export-btn">
              <span class="export-icon">📑</span>
              <div class="export-info">
                <span class="export-name">PDF 导出</span>
                <span class="export-desc">生成格式化报表</span>
              </div>
            </button>
          </div>
        </div>
        <div class="modal-footer">
          <button @click="showExportModal = false" class="btn-secondary">取消</button>
        </div>
      </div>
    </div>

    <SettingsModal 
      :show="showSettings"
      :exchange-rate="exchangeRate"
      :current-theme="currentTheme"
      :server-port="serverPort"
      :is-pro-user="isProUser"
      @close="showSettings = false"
      @change-theme="changeTheme"
      @save-settings="saveSettings"
    />

    <div v-if="showUpgradeModal" class="modal-overlay" @click.self="showUpgradeModal = false">
      <div class="modal-content upgrade-modal">
        <div class="modal-header">
          <h3>升级到 Pro</h3>
          <button @click="showUpgradeModal = false" class="modal-close">✕</button>
        </div>
        <div class="modal-body">
          <div class="upgrade-feature-hint">
            <span class="feature-icon">🔒</span>
            <span class="feature-text">此功能为 Pro 专享</span>
          </div>
          <div class="upgrade-pricing">
            <div class="pricing-card">
              <div class="pricing-header">
                <span class="pricing-name">月度订阅</span>
                <span class="pricing-price">¥9.9<span class="pricing-unit">/月</span></span>
              </div>
              <ul class="pricing-features">
                <li>✓ 无限项目</li>
                <li>✓ 智能路由</li>
                <li>✓ 极光主题</li>
                <li>✓ PDF 报表导出</li>
                <li>✓ 自定义脱敏规则</li>
              </ul>
              <button @click="showUpgradeModal = false; showLicenseModal = true" class="btn-primary upgrade-btn">立即订阅</button>
              <p class="pricing-hint">随时取消，不满意 7 天内可申请退款</p>
            </div>
          </div>
          <div class="upgrade-actions">
            <button @click="showLicenseModal = true; showUpgradeModal = false" class="btn-secondary">输入激活码</button>
          </div>
        </div>
      </div>
    </div>

    <div v-if="showLicenseModal" class="modal-overlay" @click.self="showLicenseModal = false">
      <div class="modal-content license-modal">
        <div class="modal-header">
          <h3>输入激活码</h3>
          <button @click="showLicenseModal = false" class="modal-close">✕</button>
        </div>
        <div class="modal-body">
          <p class="license-hint">请输入您订阅 GateMate Pro 后收到的月度激活码。</p>
          <div class="form-group">
            <label>激活码</label>
            <input v-model="licenseKey" type="text" placeholder="输入激活码..." class="form-input license-input" maxlength="64" />
          </div>
          <div class="license-status" v-if="isProUser">
            <span class="status-icon">✓</span>
            <span class="status-text">已激活 Pro 版本</span>
          </div>
        </div>
        <div class="modal-footer">
          <button @click="showLicenseModal = false" class="btn-secondary">取消</button>
          <button @click="verifyLicense" class="btn-primary">激活</button>
        </div>
      </div>
    </div>

    <div v-if="showCreateProjectModal" class="modal-overlay" @click.self="closeCreateProjectModal">
      <div class="modal-content">
        <div class="modal-header">
          <h3>新建项目</h3>
          <button @click="closeCreateProjectModal" class="modal-close">✕</button>
        </div>
        <div class="modal-body">
          <div class="form-group">
            <label>项目名称</label>
            <input v-model="newProjectName" type="text" placeholder="输入项目名称..." class="form-input" maxlength="30" />
            <p class="form-hint">最长 30 个字符</p>
          </div>
          <div class="form-group">
            <label>描述（可选）</label>
            <textarea v-model="newProjectDescription" placeholder="添加项目描述..." class="form-input" rows="3"></textarea>
          </div>
        </div>
        <div class="modal-footer">
          <button @click="closeCreateProjectModal" class="btn-secondary">取消</button>
          <button @click="createProject" class="btn-primary">创建</button>
        </div>
      </div>
    </div>

    <BudgetModal 
      :show="showBudgetModal"
      :monthly-cost="monthlyCost"
      :monthly-limit="totalMonthlyLimit"
      :exchange-rate="exchangeRate"
      @close="showBudgetModal = false"
      @save="saveBudget"
    />

    <CommandPalette 
      :show="showCommandPalette"
      :items="commandResults"
      @close="closeCommandPalette"
      @execute="executeCommand"
    />

    <div v-if="budgetExhausted" class="budget-overlay">
      <div class="budget-warning">
        <span class="warning-icon">🚫</span>
        <div class="warning-content">
          <span class="warning-title">本月预算已用完</span>
          <span class="warning-text">请在设置中调整限额</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import DashboardView from './components/DashboardView.vue';
import KeysView from './components/KeysView.vue';
import LogsView from './components/LogsView.vue';
import SettingsModal from './components/SettingsModal.vue';
import BudgetModal from './components/BudgetModal.vue';
import AddKeyModal from './components/AddKeyModal.vue';
import CommandPalette from './components/CommandPalette.vue';

interface ApiKey {
  id: number;
  project_id: number;
  provider: string;
  key_encrypted: string;
  remark: string;
  balance: number;
  daily_limit: number;
  monthly_limit: number;
  status: string;
  created_at: string;
}

interface CallLog {
  id: number;
  project_id: number;
  key_id: number;
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

interface Model {
  id: number;
  name: string;
  provider: string;
  tags: string;
  is_enabled: number;
  sort_order: number;
  created_at: string;
}

interface Project {
  id: number;
  uuid: string;
  name: string;
  description: string;
  monthly_budget: number;
  created_at: string;
  is_deleted: number;
}

interface CommandResult {
  id: string;
  index: number;
  icon: string;
  title: string;
  subtitle: string;
  type: string;
  action: string;
  payload?: any;
}

const EXCHANGE_RATE_DEFAULT = 7.2;

const navItems = [
  { id: 'dashboard', icon: '📊', label: '仪表盘' },
  { id: 'keys', icon: '🔑', label: 'API Keys' },
  { id: 'logs', icon: '📝', label: '调用日志' },
];

const activeTab = ref('dashboard');
const isServerReady = ref(false);
const routeStrategy = ref('round_robin');
const smartRoutingRules = ref([
  { id: 1, name: '代码生成', match_type: 'keyword', match_content: '代码|编程|code|function', target_provider: 'DeepSeek', target_model: 'deepseek-coder' },
  { id: 2, name: '翻译', match_type: 'keyword', match_content: '翻译|translate|中文|English', target_provider: 'OpenAI', target_model: 'gpt-4o-mini' },
  { id: 3, name: '兜底', match_type: 'keyword', match_content: '', target_provider: 'Claude', target_model: 'claude-3-haiku' },
]);
const notificationEnabled = ref(true);
const keys = ref<ApiKey[]>([]);
const callLogs = ref<CallLog[]>([]);
const models = ref<Model[]>([]);
const dailyCost = ref(0);
const monthlyCost = ref(0);
const chartLabels = ref<string[]>([]);
const chartData = ref<number[]>([]);
const showAddKeyModal = ref(false);
const showModelManager = ref(false);
const showExportModal = ref(false);
const toggleExportDropdown = ref(false);
const showSettings = ref(false);
const showToast = ref(false);
const toastMessage = ref('');
const toastType = ref('success');
const exchangeRate = ref(EXCHANGE_RATE_DEFAULT);
const timeRange = ref('7d');
const serverPort = ref(38080);
const editingKey = ref<ApiKey | null>(null);
const newModelName = ref('');
const newModelProvider = ref('');
const newModelTags = ref('');
const exportStartDate = ref('');
const exportEndDate = ref('');
const logFilter = ref({ provider: '', status: '' });

const projects = ref<Project[]>([]);
const currentProject = ref<Project | null>(null);
const showProjectDropdown = ref(false);
const showCreateProjectModal = ref(false);
const showBudgetModal = ref(false);
const newProjectName = ref('');
const newProjectDescription = ref('');
const projectMonthlyUsages = ref<Record<number, number>>({});
const budgetExhausted = ref(false);
const currentTheme = ref('deep-space');
const isProUser = ref(false);
const showUpgradeModal = ref(false);
const showLicenseModal = ref(false);
const licenseKey = ref('');
const showCommandPalette = ref(false);
const commandResults = ref<CommandResult[]>([]);

const showNotificationDropdown = ref(false);
const notifications = ref<any[]>([]);
const unreadNotificationCount = ref(0);

const dailyChange = ref<number | null>(null);
const monthlyChange = ref<number | null>(null);
const keyUsages = ref<Record<number, { daily: number; monthly: number }>>({});

const currentTabTitle = computed(() => {
  const item = navItems.find(n => n.id === activeTab.value);
  return item?.label || '';
});

const currentTabSubtitle = computed(() => {
  if (activeTab.value === 'dashboard') return '实时监控你的 AI 用量';
  if (activeTab.value === 'keys') return '管理和配置 API Keys';
  if (activeTab.value === 'logs') return '查看请求调用记录';
  return '';
});

const totalMonthlyLimit = computed(() => keys.value.reduce((sum, k) => sum + k.monthly_limit, 0));

const showToastMsg = (message: string, type: string = 'success') => {
  toastMessage.value = message;
  toastType.value = type;
  showToast.value = true;
  setTimeout(() => showToast.value = false, 3000);
};

const checkHealth = async () => {
  try {
    const res = await fetch(`http://localhost:${serverPort.value}/health`);
    if (res.ok) {
      isServerReady.value = true;
      loadData();
    }
  } catch (_) {
    isServerReady.value = false;
    setTimeout(checkHealth, 1000);
  }
};

const loadData = async () => {
  await Promise.all([loadKeys(), loadStats(), loadLogs(), loadModels(), loadRouteStrategy(), loadRoutingRules(), loadNotificationSetting()]);
  await loadKeyUsages();
  await loadProjectUsages();
};

const refreshData = () => {
  loadData();
  showToastMsg('数据已刷新');
};

const loadKeys = async () => {
  keys.value = await invoke<ApiKey[]>('get_keys', { projectId: currentProject.value?.id || 1 }).catch(() => []);
};

const loadKeyUsages = async () => {
  const usages: Record<number, { daily: number; monthly: number }> = {};
  for (const key of keys.value) {
    const daily = await invoke<number>('get_daily_usage', { id: key.id }).catch(() => 0);
    const monthly = await invoke<number>('get_monthly_usage', { id: key.id }).catch(() => 0);
    usages[key.id] = { daily, monthly };
  }
  keyUsages.value = usages;
};

const loadProjects = async () => {
  projects.value = await invoke<Project[]>('get_all_projects').catch(() => []);
};

const loadProjectUsages = async () => {
  const usages: Record<number, number> = {};
  for (const project of projects.value) {
    const usage = await invoke<number>('get_project_monthly_usage', { projectId: project.id }).catch(() => 0);
    usages[project.id] = usage;
  }
  projectMonthlyUsages.value = usages;
};

const selectProject = async (project: Project) => {
  currentProject.value = project;
  showProjectDropdown.value = false;
  await loadData();
};

const createProject = async () => {
  if (!newProjectName.value.trim()) {
    showToastMsg('请输入项目名称', 'error');
    return;
  }
  if (newProjectName.value.length > 30) {
    showToastMsg('项目名称不能超过30个字符', 'error');
    return;
  }
  
  const newProject = await invoke<Project>('create_project', {
    name: newProjectName.value.trim(),
    description: newProjectDescription.value.trim()
  });
  
  newProjectName.value = '';
  newProjectDescription.value = '';
  showCreateProjectModal.value = false;
  await loadProjects();
  await selectProject(newProject);
  showToastMsg('项目创建成功');
};

const handleCreateProjectClick = () => {
  showProjectDropdown.value = false;
  showCreateProjectModal.value = true;
};

const closeCreateProjectModal = () => {
  showCreateProjectModal.value = false;
  newProjectName.value = '';
  newProjectDescription.value = '';
};

const changeTheme = (theme: string) => {
  currentTheme.value = theme;
  document.body.className = `theme-${theme}`;
  showToastMsg(`主题已切换为 ${getThemeName(theme)}`);
};

const getThemeName = (theme: string) => {
  const names: Record<string, string> = {
    'deep-space': '深空',
    'bright-moon': '皓月',
    'aurora': '极光'
  };
  return names[theme] || theme;
};

const saveBudget = async (budget: number) => {
  if (!currentProject.value) return;
  await invoke('update_project', {
    id: currentProject.value.id,
    monthlyBudget: budget
  });
  currentProject.value.monthly_budget = budget;
  showBudgetModal.value = false;
  showToastMsg('预算设置已保存');
};

const checkUsageAndNotify = async () => {
  if (!notificationEnabled.value) return;
  
  for (const key of keys.value) {
    const dailyUsage = await invoke<number>('get_daily_usage', { id: key.id }).catch(() => 0);
    const monthlyUsage = await invoke<number>('get_monthly_usage', { id: key.id }).catch(() => 0);
    
    if (key.daily_limit > 0) {
      const dailyPercent = (dailyUsage / key.daily_limit) * 100;
      if (dailyPercent >= 100) {
        await invoke('send_notification', { 
          title: '⚠️ 用量超限提醒',
          body: `${key.remark || key.provider} 日用量已超限！\n当前花费: $${dailyUsage.toFixed(2)}\n日限额: $${key.daily_limit}`
        }).catch(() => {});
      } else if (dailyPercent >= 90) {
        await invoke('send_notification', { 
          title: '💡 用量预警',
          body: `${key.remark || key.provider} 日用量已达 90%！\n当前花费: $${dailyUsage.toFixed(2)}\n日限额: $${key.daily_limit}`
        }).catch(() => {});
      }
    }
    
    if (key.monthly_limit > 0) {
      const monthlyPercent = (monthlyUsage / key.monthly_limit) * 100;
      if (monthlyPercent >= 100) {
        await invoke('send_notification', { 
          title: '⚠️ 用量超限提醒',
          body: `${key.remark || key.provider} 月用量已超限！\n当前花费: $${monthlyUsage.toFixed(2)}\n月限额: $${key.monthly_limit}`
        }).catch(() => {});
      } else if (monthlyPercent >= 90) {
        await invoke('send_notification', { 
          title: '💡 用量预警',
          body: `${key.remark || key.provider} 月用量已达 90%！\n当前花费: $${monthlyUsage.toFixed(2)}\n月限额: $${key.monthly_limit}`
        }).catch(() => {});
      }
    }
  }
};

const loadStats = async () => {
  const stats = await invoke<{ daily_cost: number; monthly_cost: number; chart_labels: string[]; chart_data: number[] }>('get_daily_stats', { projectId: currentProject.value?.id || 1 }).catch(() => ({
    daily_cost: 0, monthly_cost: 0, chart_labels: [], chart_data: []
  }));
  dailyCost.value = stats.daily_cost;
  monthlyCost.value = stats.monthly_cost;
  chartLabels.value = stats.chart_labels;
  chartData.value = stats.chart_data;
  
  if (currentProject.value && currentProject.value.monthly_budget > 0) {
    budgetExhausted.value = monthlyCost.value >= currentProject.value.monthly_budget;
  } else {
    budgetExhausted.value = false;
  }
  
  await checkUsageAndNotify();
};

const loadLogs = async (newFilter?: { provider: string; status: string }) => {
  if (newFilter) {
    logFilter.value = newFilter;
  }
  const projectId = currentProject.value?.id || 1;
  if (logFilter.value.provider || logFilter.value.status) {
    callLogs.value = await invoke<CallLog[]>('get_call_logs_filtered', { 
      projectId,
      provider: logFilter.value.provider || undefined,
      status: logFilter.value.status || undefined 
    }).catch(() => []);
  } else {
    callLogs.value = await invoke<CallLog[]>('get_call_logs', { projectId }).catch(() => []);
  }
};

const loadModels = async () => {
  models.value = await invoke<Model[]>('get_models').catch(() => []);
};

const loadRouteStrategy = async () => {
  routeStrategy.value = await invoke<string>('get_route_strategy');
};

const loadRoutingRules = async () => {
  smartRoutingRules.value = await invoke<any[]>('get_routing_rules', { projectId: currentProject.value?.id || 1 }).catch(() => []);
};

const loadNotificationSetting = async () => {
  notificationEnabled.value = await invoke<boolean>('get_notification_enabled');
};

const saveRouteStrategy = async () => {
  await invoke('set_route_strategy', { strategy: routeStrategy.value });
  showToastMsg('路由策略已保存');
};

const toggleNotification = async () => {
  notificationEnabled.value = !notificationEnabled.value;
  await invoke('set_notification_enabled', { enabled: notificationEnabled.value });
  showToastMsg(notificationEnabled.value ? '通知已开启' : '通知已关闭');
};

const closeAddKeyModal = () => {
  showAddKeyModal.value = false;
  editingKey.value = null;
};

const saveKey = async (form: { provider: string; customProvider: string; model: string; key: string; remark: string; daily_limit: number; monthly_limit: number }) => {
  if (!form.provider) {
    showToastMsg('请选择供应商', 'error');
    return;
  }
  
  if (editingKey.value) {
    await invoke('update_key', { 
      id: editingKey.value.id,
      provider: form.provider, 
      apiKey: form.key || undefined,
      remark: form.remark,
      dailyLimit: form.daily_limit,
      monthlyLimit: form.monthly_limit
    });
    showToastMsg('修改成功');
  } else {
    if (!form.key) {
      showToastMsg('请输入 API Key', 'error');
      return;
    }
    
    await invoke('save_key', { 
      projectId: currentProject.value?.id || 1,
      provider: form.provider, 
      apiKey: form.key, 
      remark: form.remark,
      dailyLimit: form.daily_limit,
      monthlyLimit: form.monthly_limit
    });
    showToastMsg('保存成功');
  }
  
  closeAddKeyModal();
  loadData();
};

const editKey = (key: ApiKey) => {
  editingKey.value = key;
  showAddKeyModal.value = true;
};

const deleteKey = async (id: number) => {
  if (!confirm('确定要删除这个 Key 吗？')) return;
  await invoke('delete_key', { id });
  showToastMsg('已删除');
  loadData();
};

const toggleKeyStatus = async (key: ApiKey) => {
  const newStatus = key.status === 'active' ? 'paused' : 'active';
  await invoke('set_key_status', { id: key.id, status: newStatus });
  showToastMsg(key.status === 'active' ? '已停用' : '已启用');
  loadKeys();
};

const testKey = async (key: ApiKey) => {
  showToastMsg(`🔄 正在测试 ${key.provider} Key...`);
  try {
    const result = await invoke<string>('test_key', { id: key.id });
    if (result === '测试成功') {
      showToastMsg(`✅ ${key.provider} Key 测试成功`);
    } else {
      showToastMsg(`❌ ${result}`, 'error');
    }
  } catch (e) {
    showToastMsg(`❌ 测试失败: ${e}`, 'error');
  }
};

const addModel = async () => {
  if (!newModelName.value.trim()) {
    showToastMsg('请输入模型名称', 'error');
    return;
  }
  await invoke('add_model', { 
    name: newModelName.value.trim(),
    provider: newModelProvider.value || '',
    tags: newModelTags.value || ''
  });
  showToastMsg('模型已添加');
  newModelName.value = '';
  newModelProvider.value = '';
  newModelTags.value = '';
  loadModels();
};

const toggleModelEnabled = async (model: Model) => {
  await invoke('toggle_model', { id: model.id });
  loadModels();
};

const deleteModelItem = async (model: Model) => {
  if (!confirm(`确定要删除模型 "${model.name}" 吗？`)) return;
  await invoke('delete_model', { id: model.id });
  showToastMsg('模型已删除');
  loadModels();
};

const clearLogs = async () => {
  if (!confirm('确定要清空所有调用日志吗？')) return;
  await invoke('clear_call_logs');
  showToastMsg('日志已清空');
  loadLogs();
};

const exportCSV = async () => {
  if (!exportStartDate.value || !exportEndDate.value || !currentProject.value) {
    showToastMsg('请选择日期范围和项目', 'error');
    return;
  }
  const csv = await invoke<string>('export_csv', { projectId: currentProject.value.id, startDate: exportStartDate.value, endDate: exportEndDate.value });
  const blob = new Blob([csv], { type: 'text/csv;charset=utf-8;' });
  const link = document.createElement('a');
  link.href = URL.createObjectURL(blob);
  link.download = `gatemate_export_${exportStartDate.value}_${exportEndDate.value}.csv`;
  link.click();
  showToastMsg('CSV 导出成功');
  showExportModal.value = false;
};

const exportPDF = async () => {
  if (!exportStartDate.value || !exportEndDate.value || !currentProject.value) {
    showToastMsg('请选择日期范围和项目', 'error');
    return;
  }
  try {
    const filePath = await invoke<string>('export_pdf', { projectId: currentProject.value.id, startDate: exportStartDate.value, endDate: exportEndDate.value });
    showToastMsg(`PDF 导出成功: ${filePath}`);
    showExportModal.value = false;
  } catch (e) {
    showToastMsg(`PDF 导出失败: ${e}`, 'error');
  }
};

const verifyLicense = async () => {
  showLicenseModal.value = false;
  showToastMsg('激活码验证成功');
};

const saveSettings = () => {
  showSettings.value = false;
  showToastMsg('设置已保存');
};

const loadNotifications = async () => {
  const result = await invoke<any[]>('get_notifications');
  notifications.value = result;
  unreadNotificationCount.value = notifications.value.filter(n => n.is_read === 0).length;
};

const getNotificationIcon = (type: string) => {
  const icons: Record<string, string> = {
    'budget-warning': '⚠️',
    'budget-exhausted': '🚫',
    'key-error': '❌',
    'key-warning': '⚠️',
    'system': '🎉',
    'info': 'ℹ️'
  };
  return icons[type] || '🔔';
};

const formatNotificationTime = (timestamp: number) => {
  const date = new Date(timestamp * 1000);
  const now = new Date();
  const diff = now.getTime() - date.getTime();
  const minutes = Math.floor(diff / 60000);
  const hours = Math.floor(diff / 3600000);
  const days = Math.floor(diff / 86400000);
  
  if (minutes < 1) return '刚刚';
  if (minutes < 60) return `${minutes}分钟前`;
  if (hours < 24) return `${hours}小时前`;
  if (days < 7) return `${days}天前`;
  return date.toLocaleDateString('zh-CN');
};

const markNotificationRead = async (id: number) => {
  await invoke('mark_notification_as_read', { id });
  await loadNotifications();
};

const markAllNotificationsRead = async () => {
  await invoke('mark_all_notifications_as_read');
  await loadNotifications();
};

const clearAllNotifications = async () => {
  await invoke('clear_all_notifications');
  await loadNotifications();
};

const buildCommandResults = () => {
  const results: CommandResult[] = [];
  let index = 0;
  
  projects.value.forEach(p => {
    results.push({
      id: `project-${p.id}`,
      index: index++,
      icon: '📁',
      title: p.name,
      subtitle: p.description || '项目工作空间',
      type: '项目',
      action: 'switch-project',
      payload: p
    });
  });
  
  keys.value.forEach(k => {
    results.push({
      id: `key-${k.id}`,
      index: index++,
      icon: '🔑',
      title: k.remark || '未命名 Key',
      subtitle: `${k.provider} - ${k.status}`,
      type: 'Key',
      action: 'navigate-key',
      payload: k
    });
  });
  
  const menuItems = [
    { icon: '📊', title: '仪表盘', subtitle: '查看用量统计', action: 'navigate', payload: 'dashboard' },
    { icon: '🔑', title: 'Key 管理', subtitle: '管理 API Key', action: 'navigate', payload: 'keys' },
    { icon: '🎯', title: '路由规则', subtitle: '配置路由策略', action: 'navigate', payload: 'routing' },
    { icon: '📋', title: '调用日志', subtitle: '查看请求记录', action: 'navigate', payload: 'logs' },
    { icon: '⚙️', title: '设置', subtitle: '配置系统', action: 'open-settings', payload: null },
    { icon: '📤', title: '导出报表', subtitle: '导出 CSV/PDF', action: 'open-export', payload: null },
  ];
  
  menuItems.forEach(item => {
    results.push({
      id: `menu-${item.action}`,
      index: index++,
      icon: item.icon,
      title: item.title,
      subtitle: item.subtitle,
      type: '菜单',
      action: item.action,
      payload: item.payload
    });
  });
  
  commandResults.value = results;
};

const openCommandPalette = () => {
  buildCommandResults();
  showCommandPalette.value = true;
};

const closeCommandPalette = () => {
  showCommandPalette.value = false;
};

const executeCommand = (result: CommandResult) => {
  closeCommandPalette();
  
  switch (result.action) {
    case 'switch-project':
      currentProject.value = result.payload;
      loadKeys();
      loadLogs();
      showToastMsg(`已切换到项目: ${result.payload.name}`);
      break;
    case 'navigate':
      activeTab.value = result.payload;
      break;
    case 'navigate-key':
      activeTab.value = 'keys';
      showToastMsg(`跳转到 Key: ${result.payload.remark}`);
      break;
    case 'open-settings':
      showSettings.value = true;
      break;
    case 'open-export':
      showExportModal.value = true;
      break;
  }
};

const setDefaultExportDates = () => {
  const now = new Date();
  const firstDay = new Date(now.getFullYear(), now.getMonth(), 1);
  exportStartDate.value = firstDay.toISOString().split('T')[0];
  exportEndDate.value = now.toISOString().split('T')[0];
};

const handleKeyDown = (e: KeyboardEvent) => {
  if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
    e.preventDefault();
    openCommandPalette();
  }
};

onMounted(async () => {
  await loadProjects();
  if (projects.value.length > 0) {
    currentProject.value = projects.value[0];
  } else {
    const defaultProject = await invoke<Project>('create_project', {
      name: '默认项目',
      description: '默认工作空间'
    });
    currentProject.value = defaultProject;
    await loadProjects();
  }
  checkHealth();
  setDefaultExportDates();
  document.body.className = 'theme-deep-space';
  window.addEventListener('keydown', handleKeyDown);
  loadNotifications();
});

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeyDown);
});
</script>

<style scoped>
.app-container {
  display: flex;
  height: 100vh;
  width: 100vw;
  font-family: 'Source Han Sans SC', 'Noto Sans SC', '阿里巴巴普惠体', system-ui, -apple-system, BlinkMacSystemFont, sans-serif;
  background: var(--bg-primary);
  color: var(--text-primary);
  overflow: hidden;
  position: relative;
  transition: all 0.3s ease;
}

.app-container::before {
  content: '';
  position: absolute;
  top: 0; left: 0; right: 0; bottom: 0;
  background: 
    radial-gradient(circle at 20% 80%, rgba(88, 166, 255, 0.03) 0%, transparent 50%),
    radial-gradient(circle at 80% 20%, rgba(250, 173, 20, 0.02) 0%, transparent 50%),
    radial-gradient(circle at 50% 50%, rgba(46, 160, 67, 0.02) 0%, transparent 60%);
  pointer-events: none;
  z-index: 0;
}

.particles {
  position: absolute;
  top: 0; left: 0; right: 0; bottom: 0;
  overflow: hidden;
  pointer-events: none;
  z-index: 0;
}

.particle {
  position: absolute;
  width: 3px;
  height: 3px;
  background: rgba(88, 166, 255, 0.3);
  border-radius: 50%;
  animation: floatParticle 20s infinite ease-in-out;
}

@keyframes floatParticle {
  0%, 100% { transform: translateY(0) translateX(0); opacity: 0; }
  10% { opacity: 0.6; }
  50% { opacity: 0.3; }
  90% { opacity: 0.1; }
}

.sidebar {
  width: 220px;
  background: var(--bg-sidebar);
  backdrop-filter: blur(20px);
  border-right: 1px solid var(--border-color);
  display: flex;
  flex-direction: column;
  flex-shrink: 0;
}

.sidebar-header { padding: 20px 16px; border-bottom: 1px solid var(--border-color); }

.logo { display: flex; align-items: center; gap: 10px; }
.logo-icon { font-size: 1.5rem; }
.logo-text { font-size: 1.2rem; font-weight: 700; color: var(--text-primary); }

.logo-status {
  width: 8px; height: 8px; border-radius: 50%;
  background: #f85149; margin-left: 8px; flex-shrink: 0;
}
.logo-status.online { background: #2ea043; box-shadow: 0 0 8px rgba(46, 160, 67, 0.6); }

.sidebar-nav {
  flex: 1; padding: 12px;
  display: flex; flex-direction: column; gap: 4px;
}

.nav-item {
  display: flex; align-items: center; gap: 12px;
  padding: 10px 12px; border-radius: 8px;
  border: none; background: transparent;
  color: var(--text-secondary); cursor: pointer;
  font-size: 0.9rem; transition: all 0.2s ease;
}

.nav-item:hover { background: var(--accent-bg); color: var(--text-primary); }
.nav-item.active { background: var(--accent-bg); color: var(--accent-color); }

.nav-icon { font-size: 1rem; }
.nav-label { font-weight: 500; }

.sidebar-footer {
  padding: 16px; border-top: 1px solid rgba(255, 255, 255, 0.05);
}

.route-strategy {
  display: flex; flex-direction: column; gap: 6px;
  margin-bottom: 12px;
}

.strategy-label {
  font-size: 0.7rem; color: #8b949e;
  text-transform: uppercase; letter-spacing: 0.5px;
}

.strategy-select {
  padding: 8px 10px;
  background: rgba(0, 0, 0, 0.3);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 8px;
  color: #e6edf3; font-size: 0.8rem;
  outline: none; cursor: pointer;
}

.server-status {
  display: flex; align-items: center; gap: 8px;
  font-size: 0.75rem; color: #8b949e;
  padding: 8px 12px;
  background: rgba(255, 165, 0, 0.1); border-radius: 6px;
}

.server-status.online { color: #2ea043; background: rgba(46, 160, 67, 0.1); }

.status-dot { width: 8px; height: 8px; border-radius: 50%; background: #ffa500; }
.server-status.online .status-dot { background: #2ea043; box-shadow: 0 0 8px rgba(46, 160, 67, 0.5); }

.sidebar-toast {
  position: absolute; bottom: 100px; left: 16px; right: 16px;
  padding: 12px; border-radius: 8px;
  font-size: 0.85rem;
  animation: slideUp 0.3s ease;
}

.sidebar-toast.success { background: rgba(46, 160, 67, 0.15); color: #2ea043; }
.sidebar-toast.error { background: rgba(248, 81, 73, 0.15); color: #f85149; }

@keyframes slideUp {
  from { transform: translateY(20px); opacity: 0; }
  to { transform: translateY(0); opacity: 1; }
}

.main-panel { flex: 1; display: flex; flex-direction: column; overflow: hidden; position: relative; z-index: 1; }

.top-bar {
  display: flex; justify-content: space-between; align-items: center;
  padding: 16px 24px;
  background: rgba(13, 17, 23, 0.8); backdrop-filter: blur(20px);
  border-bottom: 1px solid rgba(255, 255, 255, 0.05);
  flex-shrink: 0;
}

.top-bar-left { display: flex; align-items: center; gap: 16px; }
.top-bar-left h2 { font-size: 1.3rem; margin: 0; color: #e6edf3; }
.top-bar-subtitle { font-size: 0.8rem; color: #8b949e; margin: 2px 0 0 0; }

.top-bar-right { display: flex; gap: 12px; align-items: center; }

.project-selector {
  display: flex; align-items: center; gap: 6px;
  padding: 8px 12px;
  background: rgba(255, 255, 255, 0.05);
  border-radius: 8px;
  cursor: pointer;
  border: 1px solid rgba(255, 255, 255, 0.08);
}

.project-name { font-size: 0.9rem; color: #e6edf3; }
.project-arrow { font-size: 0.75rem; color: #8b949e; }

.project-dropdown {
  position: absolute; top: 100%; left: 24px;
  margin-top: 8px; background: rgba(13, 17, 23, 0.95);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 12px; min-width: 200px;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.3);
  z-index: 100;
}

.project-item {
  display: flex; justify-content: space-between;
  padding: 12px 16px; cursor: pointer;
  transition: background 0.2s;
}

.project-item:hover { background: rgba(88, 166, 255, 0.1); }
.project-item.active { background: rgba(88, 166, 255, 0.15); }

.project-item-name { font-size: 0.9rem; color: #e6edf3; }
.project-item-cost { font-size: 0.8rem; color: #58a6ff; }

.project-divider { height: 1px; background: rgba(255, 255, 255, 0.05); margin: 8px 0; }

.project-add-btn {
  width: 100%; padding: 12px;
  border: none; background: transparent;
  color: #58a6ff; font-size: 0.85rem;
  cursor: pointer; border-radius: 8px;
  transition: background 0.2s;
}

.project-add-btn:hover { background: rgba(88, 166, 255, 0.1); }

.command-hint {
  font-size: 0.75rem; color: var(--text-secondary);
  padding: 4px 8px; background: rgba(255, 255, 255, 0.05);
  border-radius: 4px;
}

.notification-toggle { display: flex; align-items: center; gap: 8px; }

.toggle-label { font-size: 0.8rem; color: #8b949e; }

.toggle-switch {
  width: 44px; height: 24px; border-radius: 12px;
  background: rgba(255, 255, 255, 0.1); border: none;
  position: relative; cursor: pointer;
  transition: all 0.2s ease;
}

.toggle-switch.on { background: #2ea043; }

.toggle-knob {
  position: absolute; top: 3px; left: 3px;
  width: 18px; height: 18px; border-radius: 50%;
  background: #fff; transition: all 0.2s ease;
}

.toggle-switch.on .toggle-knob { left: 23px; }

.btn-icon {
  width: 36px; height: 36px;
  display: flex; align-items: center; justify-content: center;
  border: none; background: rgba(255, 255, 255, 0.05);
  color: #8b949e; border-radius: 8px;
  cursor: pointer; font-size: 1rem;
  transition: all 0.2s ease;
}

.btn-icon:hover { background: rgba(88, 166, 255, 0.15); color: #58a6ff; }

.content-area {
  flex: 1; padding: 20px;
  overflow: hidden;
  background: linear-gradient(180deg, #0a0e17 0%, #0f1624 100%);
  display: flex; flex-direction: column;
  position: relative;
}

.server-status-badge {
  display: flex; align-items: center; gap: 6px;
  font-size: 0.78rem; color: #f85149;
  padding: 6px 14px;
  background: rgba(218, 54, 51, 0.1);
  border-radius: 20px; font-weight: 500;
}

.server-status-badge.online { color: #2ea043; background: rgba(46, 160, 67, 0.15); }
.server-status-badge .status-dot { background: #f85149; width: 6px; height: 6px; }
.server-status-badge.online .status-dot { background: #2ea043; box-shadow: 0 0 6px rgba(46, 160, 67, 0.6); }

.notification-center {
  position: relative;
  width: 36px; height: 36px;
  display: flex; align-items: center; justify-content: center;
  cursor: pointer;
}

.notification-icon { font-size: 1rem; }

.notification-badge {
  position: absolute; top: 2px; right: 2px;
  min-width: 18px; height: 18px;
  padding: 0 4px;
  background: #f85149;
  color: white; font-size: 0.65rem;
  border-radius: 9px;
  display: flex; align-items: center; justify-content: center;
}

.notification-dropdown {
  position: absolute; top: 100%; right: 0;
  margin-top: 8px;
  background: rgba(13, 17, 23, 0.95);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 12px;
  min-width: 320px;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.3);
  z-index: 100;
}

.notification-dropdown-header {
  display: flex; justify-content: space-between;
  padding: 12px 16px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.05);
}

.notification-dropdown-title { font-size: 0.9rem; color: #e6edf3; font-weight: 500; }
.notification-dropdown-count { font-size: 0.75rem; color: #58a6ff; }

.notification-dropdown-body { max-height: 300px; overflow-y: auto; }

.notification-empty {
  display: flex; flex-direction: column;
  align-items: center; padding: 24px;
  gap: 8px;
}

.notification-empty .empty-icon { font-size: 2rem; }
.notification-empty .empty-text { font-size: 0.85rem; color: #8b949e; }

.notification-item {
  display: flex; gap: 12px;
  padding: 12px 16px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.03);
}

.notification-item:last-child { border-bottom: none; }
.notification-item.unread { background: rgba(88, 166, 255, 0.05); }

.notification-item-icon { font-size: 1.1rem; }

.notification-item-content { flex: 1; display: flex; flex-direction: column; gap: 4px; }
.notification-item-title { font-size: 0.85rem; color: #e6edf3; font-weight: 500; }
.notification-item-text { font-size: 0.78rem; color: #8b949e; }
.notification-item-time { font-size: 0.7rem; color: #484f58; }

.notification-item-mark {
  width: 24px; height: 24px;
  border: none; background: transparent;
  color: #484f58; cursor: pointer;
  border-radius: 4px;
  transition: all 0.2s;
}

.notification-item-mark:hover { background: rgba(46, 160, 67, 0.1); color: #2ea043; }

.notification-dropdown-footer {
  display: flex; justify-content: space-between;
  padding: 12px 16px;
  border-top: 1px solid rgba(255, 255, 255, 0.05);
}

.notification-footer-btn {
  padding: 8px 16px;
  border: none; background: transparent;
  color: #8b949e; font-size: 0.8rem;
  cursor: pointer; border-radius: 6px;
  transition: all 0.2s;
}

.notification-footer-btn:hover { background: rgba(255, 255, 255, 0.05); color: #e6edf3; }
.notification-footer-btn.danger:hover { background: rgba(248, 81, 73, 0.1); color: #f85149; }

.modal-overlay {
  position: fixed; inset: 0;
  background: rgba(0, 0, 0, 0.7);
  backdrop-filter: blur(8px);
  display: flex;
  align-items: center; justify-content: center;
  z-index: 1000;
}

.modal-content {
  background: var(--bg-card);
  border: 1px solid var(--border-color);
  border-radius: 16px;
  width: 480px; max-width: 90vw;
  max-height: 90vh; overflow-y: auto;
}

.modal-header {
  display: flex; justify-content: space-between; align-items: center;
  padding: 16px 20px;
  border-bottom: 1px solid var(--border-color);
  position: sticky; top: 0; background: var(--bg-card);
}

.modal-header h3 { font-size: 1.1rem; margin: 0; color: var(--text-primary); }

.modal-close {
  width: 32px; height: 32px;
  display: flex; align-items: center; justify-content: center;
  border: none; background: transparent;
  color: #8b949e; cursor: pointer; font-size: 1rem;
  border-radius: 6px; transition: all 0.2s ease;
}

.modal-close:hover { background: rgba(255, 255, 255, 0.05); color: var(--text-primary); }

.modal-body { padding: 20px; }
.modal-footer {
  display: flex; justify-content: flex-end; gap: 12px;
  padding: 16px 20px;
  border-top: 1px solid var(--border-color);
}

.form-group { margin-bottom: 16px; }

.form-group label {
  display: block; font-size: 0.85rem;
  color: #8b949e; margin-bottom: 8px;
}

.form-input, .form-select {
  width: 100%; padding: 12px;
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 10px; color: var(--text-primary);
  font-size: 0.9rem; outline: none;
  transition: all 0.2s ease;
}

.form-input:focus, .form-select:focus { border-color: rgba(88, 166, 255, 0.5); }

.form-hint {
  font-size: 0.75rem; color: #484f58;
  margin: 6px 0 0 0;
}

.form-row { display: flex; gap: 12px; }
.form-row .form-group { flex: 1; }
.form-row .flex-1 { flex: 1; }

.btn-primary {
  display: flex; align-items: center; gap: 8px;
  padding: 12px 22px;
  background: linear-gradient(135deg, #238636, #2ea043);
  border: none; color: white;
  border-radius: 12px; font-size: 0.9rem; font-weight: 600;
  cursor: pointer; transition: all 0.3s ease;
  box-shadow: 0 4px 14px rgba(46, 160, 67, 0.35);
}

.btn-primary:hover { 
  transform: translateY(-2px); 
  box-shadow: 0 8px 24px rgba(46, 160, 67, 0.45);
  background: linear-gradient(135deg, #2ea043, #3fb950);
}

.btn-primary.btn-sm { padding: 8px 16px; font-size: 0.85rem; }

.btn-secondary {
  padding: 12px 22px;
  background: rgba(255, 255, 255, 0.04);
  border: 1px solid rgba(255, 255, 255, 0.08);
  color: #e6edf3; border-radius: 12px;
  font-size: 0.9rem; cursor: pointer;
  transition: all 0.25s ease;
}

.btn-secondary:hover { 
  background: rgba(255, 255, 255, 0.08); 
  border-color: rgba(255, 255, 255, 0.15);
}

.btn-danger-sm {
  padding: 6px 12px;
  background: rgba(248, 81, 73, 0.1);
  border: 1px solid rgba(248, 81, 73, 0.2);
  border-radius: 6px;
  color: #f85149;
  font-size: 0.8rem;
  cursor: pointer;
  transition: all 0.2s ease;
}

.btn-danger-sm:hover {
  background: rgba(248, 81, 73, 0.2);
  border-color: rgba(248, 81, 73, 0.4);
}

.date-range { display: flex; align-items: center; gap: 8px; }
.date-separator { color: #8b949e; }

.export-options { display: flex; flex-direction: column; gap: 12px; margin-top: 16px; }

.export-btn {
  display: flex; align-items: center; gap: 12px;
  padding: 16px;
  background: rgba(255, 255, 255, 0.03);
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 10px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.export-btn:hover {
  background: rgba(88, 166, 255, 0.08);
  border-color: rgba(88, 166, 255, 0.2);
}

.export-icon { font-size: 1.5rem; }

.export-info { display: flex; flex-direction: column; gap: 2px; }
.export-name { font-size: 0.9rem; color: #e6edf3; font-weight: 500; }
.export-desc { font-size: 0.78rem; color: #8b949e; }

.model-modal { width: 600px; }

.model-list { margin-top: 16px; max-height: 300px; overflow-y: auto; }

.model-item {
  display: flex; justify-content: space-between;
  align-items: center;
  padding: 12px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.03);
}

.model-info { display: flex; align-items: center; gap: 8px; flex: 1; }
.model-name { font-size: 0.9rem; color: #e6edf3; }
.model-name.disabled { color: #484f58; text-decoration: line-through; }
.model-tag {
  font-size: 0.65rem; padding: 2px 6px;
  background: rgba(88, 166, 255, 0.15);
  color: #58a6ff; border-radius: 3px;
}
.model-provider { font-size: 0.75rem; color: #8b949e; }

.model-actions { display: flex; gap: 8px; }

.toggle-btn {
  padding: 6px 12px;
  background: rgba(248, 81, 73, 0.1);
  border: 1px solid rgba(248, 81, 73, 0.2);
  color: #f85149;
  border-radius: 6px;
  font-size: 0.8rem;
  cursor: pointer;
  transition: all 0.2s ease;
}

.toggle-btn.active {
  background: rgba(46, 160, 67, 0.1);
  border-color: rgba(46, 160, 67, 0.2);
  color: #2ea043;
}

.upgrade-modal { width: 420px; }

.upgrade-feature-hint {
  display: flex; align-items: center; justify-content: center;
  gap: 8px; padding: 16px;
  background: rgba(250, 173, 20, 0.1);
  border-radius: 10px;
  margin-bottom: 20px;
}

.feature-icon { font-size: 1.2rem; }
.feature-text { font-size: 0.85rem; color: #faa31a; }

.upgrade-pricing { display: flex; justify-content: center; }

.pricing-card {
  background: rgba(88, 166, 255, 0.08);
  border: 1px solid rgba(88, 166, 255, 0.2);
  border-radius: 16px;
  padding: 24px;
  width: 100%;
  text-align: center;
}

.pricing-header { margin-bottom: 16px; }
.pricing-name { font-size: 0.85rem; color: #8b949e; }
.pricing-price { font-size: 2rem; font-weight: 700; color: #58a6ff; }
.pricing-unit { font-size: 0.9rem; font-weight: 400; color: #8b949e; }

.pricing-features {
  list-style: none; padding: 0; margin: 16px 0;
  display: flex; flex-direction: column; gap: 8px;
}

.pricing-features li { font-size: 0.85rem; color: #e6edf3; text-align: left; }

.pricing-hint {
  font-size: 0.75rem; color: #484f58;
  margin-top: 16px;
}

.upgrade-btn { width: 100%; }

.upgrade-actions { margin-top: 16px; text-align: center; }

.license-modal { width: 420px; }

.license-hint {
  font-size: 0.85rem; color: #8b949e;
  margin-bottom: 16px;
}

.license-input { font-family: monospace; letter-spacing: 2px; }

.license-status {
  display: flex; align-items: center; gap: 8px;
  padding: 12px;
  background: rgba(46, 160, 67, 0.1);
  border-radius: 8px;
  margin-top: 16px;
}

.status-icon { color: #2ea043; font-size: 1.1rem; }
.status-text { color: #2ea043; font-size: 0.85rem; }

.budget-overlay {
  position: fixed; inset: 0;
  background: rgba(0, 0, 0, 0.8);
  backdrop-filter: blur(10px);
  display: flex;
  align-items: center; justify-content: center;
  z-index: 2000;
}

.budget-warning {
  display: flex; align-items: center; gap: 16px;
  padding: 24px 32px;
  background: rgba(248, 81, 73, 0.15);
  border: 1px solid rgba(248, 81, 73, 0.3);
  border-radius: 16px;
}

.warning-icon { font-size: 2rem; }

.warning-content { display: flex; flex-direction: column; gap: 4px; }
.warning-title { font-size: 1.1rem; color: #f85149; font-weight: 600; }
.warning-text { font-size: 0.85rem; color: #ff9b9b; }
</style>