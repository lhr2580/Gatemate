export const EXCHANGE_RATE_DEFAULT = 7.2;

export const MAX_LICENSE_KEY_LENGTH = 64;

export const PROVIDERS = [
  { id: 'openai', name: 'OpenAI', label: 'OpenAI' },
  { id: 'deepseek', name: 'DeepSeek', label: 'DeepSeek' },
  { id: 'doubao', name: 'Doubao', label: '豆包' },
  { id: 'qwen', name: 'Qwen', label: '通义千问' },
  { id: 'anthropic', name: 'Claude', label: 'Claude' },
  { id: 'gemini', name: 'Gemini', label: 'Gemini' },
  { id: 'groq', name: 'Groq', label: 'Groq' },
  { id: 'ollama', name: 'Ollama', label: 'Ollama' },
];

export const PROVIDER_LABELS: Record<string, string> = {
  'openai': 'OpenAI',
  'deepseek': 'DeepSeek',
  'doubao': '豆包',
  'qwen': '通义千问',
  'anthropic': 'Claude',
  'gemini': 'Gemini',
  'groq': 'Groq',
  'ollama': 'Ollama',
};

export const PROVIDER_COLORS: Record<string, string> = {
  'openai': '#10a37f',
  'deepseek': '#7b61ff',
  'doubao': '#3b82f6',
  'qwen': '#e67e22',
  'anthropic': '#52525b',
  'gemini': '#4285f4',
  'groq': '#000000',
  'ollama': '#000000',
};

export const ROUTE_STRATEGIES = [
  { id: 'round_robin', label: '轮询' },
  { id: 'random', label: '随机' },
  { id: 'smart', label: '智能路由' },
];

export const MONTHLY_BUDGET_MAX = 10000;

export const STORAGE_KEYS = {
  THEME: 'gatemate_theme',
  EXCHANGE_RATE: 'gatemate_exchange_rate',
};