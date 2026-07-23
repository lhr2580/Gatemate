export const TOAST_DURATION_MS = 3000;
export const HEALTH_CHECK_INTERVAL_MS = 1000;
export const MS_PER_MINUTE = 60000;
export const MS_PER_HOUR = 3600000;
export const MS_PER_DAY = 86400000;

export const BUDGET_WARNING_THRESHOLD = 80;
export const USAGE_ALERT_THRESHOLD = 90;
export const USAGE_EXHAUSTED_THRESHOLD = 100;
export const KEY_UTILIZATION_HIGH = 70;
export const KEY_UTILIZATION_MEDIUM = 40;

export const PARTICLE_COUNT = 20;
export const CHART_DEFAULT_DAYS = 7;
export const DEFAULT_DAILY_LIMIT = 100;
export const DEFAULT_MONTHLY_LIMIT = 100;
export const MAX_PROJECT_NAME_LENGTH = 30;
export const MAX_COMMAND_RESULTS = 10;
export const KEY_MASK_THRESHOLD = 8;
export const MAX_LICENSE_KEY_LENGTH = 64;

export const DEFAULT_SERVER_PORT = 38080;

export enum ThemeEnum {
  DEEP_SPACE = 'deep-space',
  BRIGHT_MOON = 'bright-moon',
  AURORA = 'aurora',
}

export enum RouteStrategy {
  ROUND_ROBIN = 'round_robin',
  RANDOM = 'random',
  SMART = 'smart',
}

export const CHART_ANIMATION_DURATION_MS = 800;

export const SIDEBAR_WIDTH = 220;

export const DASHBOARD_TITLE = 'AI 调度中心';
export const DEFAULT_PROJECT_NAME = '默认项目';
export const DEFAULT_PROJECT_DESC = '默认项目描述';

export const ROUTING_RULE_MATCH_ANY = 'any';
export const ROUTING_RULE_MATCH_CONTAINS = 'contains';
export const ROUTING_RULE_MATCH_REGEX = 'regex';

export const LOG_LEVEL_INFO = 'info';
export const LOG_LEVEL_WARN = 'warn';
export const LOG_LEVEL_ERROR = 'error';

export const NOTIFICATION_TYPE_SUCCESS = 'success';
export const NOTIFICATION_TYPE_ERROR = 'error';
export const NOTIFICATION_TYPE_WARNING = 'warning';
export const NOTIFICATION_TYPE_INFO = 'info';

export const API_PATH_HEALTH = '/health';
export const API_PATH_V1 = '/api/v1';

export const CURRENCY_SYMBOL = '¥';
export const DECIMAL_PLACES = 4;

export const PAGE_SIZE_LOGS = 50;
export const PAGE_SIZE_KEYS = 20;

export const DATE_FORMAT_DISPLAY = 'YYYY-MM-DD HH:mm:ss';
export const DATE_FORMAT_SHORT = 'YYYY-MM-DD';

export const MAX_LOG_DISPLAY_LENGTH = 100;
export const MAX_KEY_DISPLAY_LENGTH = 10;

export const SORT_ORDER_ASC = 'asc';
export const SORT_ORDER_DESC = 'desc';

export const STATUS_ACTIVE = 'active';
export const STATUS_INACTIVE = 'inactive';
export const STATUS_SUCCESS = 'success';
export const STATUS_FAILED = 'failed';
export const STATUS_PENDING = 'pending';

export const PROVIDER_OPENAI = 'openai';
export const PROVIDER_DEEPSEEK = 'deepseek';
export const PROVIDER_QWEN = 'qwen';
export const PROVIDER_DOUBAO = 'doubao';
export const PROVIDER_YIYAN = 'yiyan';
export const PROVIDER_ANTHROPIC = 'anthropic';
export const PROVIDER_GEMINI = 'gemini';

export const PLUGIN_FILENAME = 'gatemate_plugin.dll';

export const DB_FILENAME = 'gatemate.db';

export const BACKUP_DIR_NAME = 'backups';
export const DATA_DIR_NAME = 'data';
export const LOGS_DIR_NAME = 'logs';
