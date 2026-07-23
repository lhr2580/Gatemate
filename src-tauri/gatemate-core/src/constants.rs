pub const AES_KEY_SIZE: usize = 32;
pub const AES_NONCE_SIZE: usize = 12;

pub const DEFAULT_SERVER_PORT: u16 = 38080;
pub const BIND_ADDRESS: &str = "127.0.0.1";

pub const DB_FILENAME: &str = "gatemate.db";
pub const DB_DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";
pub const DB_DATE_FORMAT: &str = "%Y-%m-%d";

pub const MAX_LOG_RESULTS: i64 = 100;
pub const MAX_CHART_DAYS: i64 = 30;

pub const DEFAULT_ROUTE_STRATEGY: &str = "round_robin";
pub const DEFAULT_PROJECT_NAME: &str = "默认项目";
pub const DEFAULT_PROJECT_DESC: &str = "默认项目描述";

pub const LAYER_NAME: &str = "Layer 1";
pub const A4_WIDTH_MM: f64 = 210.0;
pub const A4_HEIGHT_MM: f64 = 297.0;

pub const PLUGIN_FILENAME: &str = "gatemate_plugin.dll";

pub const MIN_LICENSE_KEY_LENGTH: usize = 16;
pub const LICENSE_VALIDITY_DAYS: i64 = 30;

pub const MASTER_KEY_FILE: &str = "master_key.bin";

pub const DATA_DIR_NAME: &str = "data";
pub const BACKUPS_DIR_NAME: &str = "backups";
pub const LOGS_DIR_NAME: &str = "logs";

pub const SUCCESS_RESULT: &str = "success";

pub const TIMEOUT_SECONDS: u64 = 30;

pub const MAX_PROJECT_NAME_LENGTH: usize = 30;
pub const MAX_KEY_REMARK_LENGTH: usize = 100;

pub const DEFAULT_DAILY_LIMIT: f64 = 100.0;
pub const DEFAULT_MONTHLY_LIMIT: f64 = 100.0;

pub const BUDGET_WARNING_THRESHOLD: f64 = 0.8;
pub const BUDGET_EXHAUSTED_THRESHOLD: f64 = 1.0;

pub const PDF_TITLE: &str = "GateMate Report";

pub const SERVICE_NAME: &str = "gatemate";
pub const SERVICE_VERSION: &str = "2.0.0";
