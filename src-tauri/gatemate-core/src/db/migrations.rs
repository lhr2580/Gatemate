use rusqlite::{Connection, Result};

pub struct Migration {
    pub version: i32,
    pub name: &'static str,
    pub sql: &'static str,
}

pub const MIGRATIONS: &[Migration] = &[
    Migration {
        version: 1,
        name: "initial_schema",
        sql: r#"
            CREATE TABLE IF NOT EXISTS migrations (
                version INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                applied_at TEXT NOT NULL
            );
            
            CREATE TABLE IF NOT EXISTS projects (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                description TEXT DEFAULT '',
                monthly_budget REAL DEFAULT 0.0,
                created_at TEXT NOT NULL
            );
            
            CREATE TABLE IF NOT EXISTS keys (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                project_id INTEGER NOT NULL,
                provider TEXT NOT NULL,
                encrypted_key TEXT NOT NULL,
                remark TEXT DEFAULT '',
                status TEXT DEFAULT 'active',
                daily_limit REAL DEFAULT 0.0,
                monthly_limit REAL DEFAULT 0.0,
                daily_usage REAL DEFAULT 0.0,
                monthly_usage REAL DEFAULT 0.0,
                created_at TEXT NOT NULL,
                FOREIGN KEY (project_id) REFERENCES projects(id)
            );
            
            CREATE TABLE IF NOT EXISTS usage_logs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                key_id INTEGER NOT NULL,
                project_id INTEGER NOT NULL,
                provider TEXT NOT NULL,
                cost REAL NOT NULL,
                created_at TEXT NOT NULL,
                FOREIGN KEY (key_id) REFERENCES keys(id)
            );
            
            CREATE TABLE IF NOT EXISTS call_logs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                project_id INTEGER NOT NULL,
                key_id INTEGER NOT NULL,
                provider TEXT NOT NULL,
                remark TEXT DEFAULT '',
                model TEXT DEFAULT '',
                prompt_tokens INTEGER DEFAULT 0,
                completion_tokens INTEGER DEFAULT 0,
                cost REAL DEFAULT 0.0,
                status TEXT DEFAULT 'pending',
                error_message TEXT,
                created_at TEXT NOT NULL,
                FOREIGN KEY (key_id) REFERENCES keys(id)
            );
            
            CREATE TABLE IF NOT EXISTS models (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                provider TEXT NOT NULL,
                tags TEXT DEFAULT '',
                enabled INTEGER DEFAULT 1
            );
            
            CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );
            
            CREATE TABLE IF NOT EXISTS notifications (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                project_id INTEGER NOT NULL,
                type TEXT NOT NULL,
                title TEXT NOT NULL,
                content TEXT NOT NULL,
                is_read INTEGER DEFAULT 0,
                created_at TEXT NOT NULL,
                FOREIGN KEY (project_id) REFERENCES projects(id)
            );
            
            CREATE TABLE IF NOT EXISTS routing_rules (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                project_id INTEGER NOT NULL,
                rule_name TEXT NOT NULL,
                match_type TEXT NOT NULL DEFAULT 'keyword',
                match_content TEXT NOT NULL DEFAULT '',
                target_provider TEXT NOT NULL,
                target_model TEXT NOT NULL,
                priority INTEGER DEFAULT 0,
                is_enabled INTEGER DEFAULT 1,
                FOREIGN KEY (project_id) REFERENCES projects(id)
            );
        "#,
    },
    Migration {
        version: 2,
        name: "add_default_project",
        sql: r#"
            INSERT OR IGNORE INTO projects (name, description, monthly_budget, created_at) 
            VALUES ('默认项目', '默认项目描述', 0.0, datetime('now'));
        "#,
    },
    Migration {
        version: 3,
        name: "add_indexes",
        sql: r#"
            CREATE INDEX IF NOT EXISTS idx_keys_provider ON keys(provider);
            CREATE INDEX IF NOT EXISTS idx_usage_logs_key_id ON usage_logs(key_id);
            CREATE INDEX IF NOT EXISTS idx_usage_logs_project_id ON usage_logs(project_id);
            CREATE INDEX IF NOT EXISTS idx_call_logs_project_id ON call_logs(project_id);
            CREATE INDEX IF NOT EXISTS idx_notifications_project_id ON notifications(project_id);
            CREATE INDEX IF NOT EXISTS idx_routing_rules_project_id ON routing_rules(project_id);
        "#,
    },
];

pub fn migrate(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS migrations (version INTEGER PRIMARY KEY, name TEXT NOT NULL, applied_at TEXT NOT NULL)",
        [],
    )?;

    let current_version: Option<i32> = conn.query_row(
        "SELECT MAX(version) FROM migrations",
        [],
        |row| row.get(0),
    )?;

    let current_version = current_version.unwrap_or(0);

    for migration in MIGRATIONS.iter().filter(|m| m.version > current_version) {
        conn.execute_batch(migration.sql)?;
        conn.execute(
            "INSERT INTO migrations (version, name, applied_at) VALUES (?, ?, datetime('now'))",
            (migration.version, migration.name),
        )?;
    }

    Ok(())
}
