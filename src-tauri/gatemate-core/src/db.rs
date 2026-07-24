use rusqlite::{Connection, Result, params};
use chrono::{Local, Datelike};
use serde::{Serialize, Deserialize};

mod migrations;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiKey {
    pub id: i64,
    pub project_id: i64,
    pub provider: String,
    #[serde(skip_serializing)]
    pub encrypted_key: String,
    pub remark: String,
    pub status: String,
    pub daily_limit: f64,
    pub monthly_limit: f64,
    pub daily_usage: f64,
    pub monthly_usage: f64,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UsageLog {
    pub id: i64,
    pub key_id: i64,
    pub project_id: i64,
    pub provider: String,
    pub cost: f64,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CallLog {
    pub id: i64,
    pub project_id: i64,
    pub key_id: i64,
    pub provider: String,
    pub remark: String,
    pub model: String,
    pub prompt_tokens: i64,
    pub completion_tokens: i64,
    pub cost: f64,
    pub status: String,
    pub error_message: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Model {
    pub id: i64,
    pub name: String,
    pub provider: String,
    pub tags: String,
    #[serde(rename = "is_enabled")]
    pub enabled: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub monthly_budget: f64,
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Notification {
    pub id: i64,
    pub project_id: i64,
    pub r#type: String,
    pub title: String,
    pub content: String,
    pub is_read: i32,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RoutingRule {
    pub id: i64,
    pub project_id: i64,
    pub rule_name: String,
    pub match_type: String,
    pub match_content: String,
    pub target_provider: String,
    pub target_model: String,
    pub priority: i32,
    pub is_enabled: i32,
}

#[derive(Debug, Clone)]
pub struct RoutingRuleUpdate {
    pub rule_name: String,
    pub match_type: String,
    pub match_content: String,
    pub target_provider: String,
    pub target_model: String,
    pub priority: i32,
    pub is_enabled: i32,
}

fn get_current_month_range() -> (String, String) {
    let now = Local::now();
    let month = now.format("%Y-%m").to_string();
    let date = now.date_naive();
    let year = date.year();
    let month_num = date.month();
    let (next_year, next_month_num) = if month_num == 12 {
        (year + 1, 1)
    } else {
        (year, month_num + 1)
    };
    let next_month = format!("{}-{:02}-01", next_year, next_month_num);
    (month, next_month)
}

pub fn init_db(db_path: &str) -> Result<Connection> {
    let conn = Connection::open(db_path)?;
    
    conn.execute_batch(
        "PRAGMA journal_mode=WAL;
         PRAGMA synchronous=NORMAL;"
    )?;
    
    migrations::migrate(&conn)?;
    
    Ok(conn)
}

pub fn insert_key(conn: &Connection, project_id: i64, provider: &str, encrypted_key: &str, remark: &str, daily_limit: f64, monthly_limit: f64) -> Result<i64> {
    conn.execute(
        "INSERT INTO keys (project_id, provider, encrypted_key, remark, daily_limit, monthly_limit, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)",
        params![project_id, provider, encrypted_key, remark, daily_limit, monthly_limit, Local::now().format("%Y-%m-%d %H:%M:%S").to_string()],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn update_key(conn: &Connection, id: i64, provider: &str, encrypted_key: Option<&str>, remark: &str, daily_limit: f64, monthly_limit: f64) -> Result<()> {
    if let Some(encrypted_key) = encrypted_key {
        conn.execute(
            "UPDATE keys SET provider = ?, encrypted_key = ?, remark = ?, daily_limit = ?, monthly_limit = ? WHERE id = ?",
            params![provider, encrypted_key, remark, daily_limit, monthly_limit, id],
        )?;
    } else {
        conn.execute(
            "UPDATE keys SET provider = ?, remark = ?, daily_limit = ?, monthly_limit = ? WHERE id = ?",
            params![provider, remark, daily_limit, monthly_limit, id],
        )?;
    }
    Ok(())
}

pub fn get_all_keys(conn: &Connection, project_id: i64) -> Result<Vec<ApiKey>> {
    let mut stmt = conn.prepare("SELECT id, project_id, provider, encrypted_key, remark, status, daily_limit, monthly_limit, daily_usage, monthly_usage, created_at FROM keys WHERE project_id = ? ORDER BY created_at DESC")?;
    let keys = stmt.query_map(params![project_id], |row| {
        Ok(ApiKey {
            id: row.get(0)?,
            project_id: row.get(1)?,
            provider: row.get(2)?,
            encrypted_key: row.get(3)?,
            remark: row.get(4)?,
            status: row.get(5)?,
            daily_limit: row.get(6)?,
            monthly_limit: row.get(7)?,
            daily_usage: row.get(8)?,
            monthly_usage: row.get(9)?,
            created_at: row.get(10)?,
        })
    })?;
    
    let mut result = Vec::new();
    for key in keys {
        result.push(key?);
    }
    Ok(result)
}

pub fn get_key_by_id(conn: &Connection, id: i64) -> Result<ApiKey> {
    conn.query_row("SELECT id, project_id, provider, encrypted_key, remark, status, daily_limit, monthly_limit, daily_usage, monthly_usage, created_at FROM keys WHERE id = ?", params![id], |row| {
        Ok(ApiKey {
            id: row.get(0)?,
            project_id: row.get(1)?,
            provider: row.get(2)?,
            encrypted_key: row.get(3)?,
            remark: row.get(4)?,
            status: row.get(5)?,
            daily_limit: row.get(6)?,
            monthly_limit: row.get(7)?,
            daily_usage: row.get(8)?,
            monthly_usage: row.get(9)?,
            created_at: row.get(10)?,
        })
    })
}

pub fn get_keys_by_project_and_provider(conn: &Connection, project_id: i64, provider: &str) -> Result<Vec<ApiKey>> {
    let mut stmt = conn.prepare("SELECT id, project_id, provider, encrypted_key, remark, status, daily_limit, monthly_limit, daily_usage, monthly_usage, created_at FROM keys WHERE project_id = ? AND provider = ?")?;
    let keys = stmt.query_map(params![project_id, provider], |row| {
        Ok(ApiKey {
            id: row.get(0)?,
            project_id: row.get(1)?,
            provider: row.get(2)?,
            encrypted_key: row.get(3)?,
            remark: row.get(4)?,
            status: row.get(5)?,
            daily_limit: row.get(6)?,
            monthly_limit: row.get(7)?,
            daily_usage: row.get(8)?,
            monthly_usage: row.get(9)?,
            created_at: row.get(10)?,
        })
    })?;
    
    let mut result = Vec::new();
    for key in keys {
        result.push(key?);
    }
    Ok(result)
}

pub fn delete_key(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("DELETE FROM keys WHERE id = ?", params![id])?;
    Ok(())
}

pub fn set_key_status(conn: &Connection, id: i64, status: &str) -> Result<()> {
    conn.execute("UPDATE keys SET status = ? WHERE id = ?", params![status, id])?;
    Ok(())
}

pub fn get_daily_usage(conn: &Connection, key_id: i64) -> Result<f64> {
    let today = Local::now().format("%Y-%m-%d").to_string();
    conn.query_row(
        "SELECT COALESCE(SUM(cost), 0.0) FROM usage_logs WHERE key_id = ? AND created_at >= ? AND created_at < ?",
        params![key_id, format!("{} 00:00:00", today), format!("{} 23:59:59", today)],
        |row| row.get(0),
    )
}

pub fn get_monthly_usage(conn: &Connection, key_id: i64) -> Result<f64> {
    let (month, next_month) = get_current_month_range();
    conn.query_row(
        "SELECT COALESCE(SUM(cost), 0.0) FROM usage_logs WHERE key_id = ? AND created_at >= ? AND created_at < ?",
        params![key_id, format!("{} 00:00:00", month), format!("{} 00:00:00", next_month)],
        |row| row.get(0),
    )
}

pub fn add_key_daily_usage(conn: &Connection, key_id: i64, cost: f64) -> Result<()> {
    conn.execute(
        "INSERT INTO usage_logs (key_id, project_id, provider, cost, created_at) 
         SELECT ?, project_id, provider, ?, ? FROM keys WHERE id = ?",
        params![key_id, cost, Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), key_id],
    )?;
    
    let today = Local::now().format("%Y-%m-%d").to_string();
    conn.execute(
        "UPDATE keys SET daily_usage = (SELECT COALESCE(SUM(cost), 0.0) FROM usage_logs WHERE key_id = ? AND created_at >= ? AND created_at < ?) WHERE id = ?",
        params![key_id, format!("{} 00:00:00", today), format!("{} 23:59:59", today), key_id],
    )?;
    
    let (month, next_month) = get_current_month_range();
    conn.execute(
        "UPDATE keys SET monthly_usage = (SELECT COALESCE(SUM(cost), 0.0) FROM usage_logs WHERE key_id = ? AND created_at >= ? AND created_at < ?) WHERE id = ?",
        params![key_id, format!("{} 00:00:00", month), format!("{} 00:00:00", next_month), key_id],
    )?;
    
    Ok(())
}

pub fn get_project_daily_usage(conn: &Connection, project_id: i64) -> Result<f64> {
    let today = Local::now().format("%Y-%m-%d").to_string();
    conn.query_row(
        "SELECT COALESCE(SUM(cost), 0.0) FROM usage_logs WHERE project_id = ? AND created_at >= ? AND created_at < ?",
        params![project_id, format!("{} 00:00:00", today), format!("{} 23:59:59", today)],
        |row| row.get(0),
    )
}

pub fn get_project_monthly_usage(conn: &Connection, project_id: i64) -> Result<f64> {
    let (month, next_month) = get_current_month_range();
    conn.query_row(
        "SELECT COALESCE(SUM(cost), 0.0) FROM usage_logs WHERE project_id = ? AND created_at >= ? AND created_at < ?",
        params![project_id, format!("{} 00:00:00", month), format!("{} 00:00:00", next_month)],
        |row| row.get(0),
    )
}

pub fn reset_all_keys_daily_usage(conn: &Connection) -> Result<()> {
    conn.execute("UPDATE keys SET daily_usage = 0.0", [])?;
    Ok(())
}

pub fn reset_all_keys_monthly_usage(conn: &Connection) -> Result<()> {
    conn.execute("UPDATE keys SET monthly_usage = 0.0", [])?;
    Ok(())
}

pub fn get_monthly_usage_by_day(conn: &Connection) -> Result<Vec<(String, f64)>> {
    let mut stmt = conn.prepare("SELECT SUBSTR(created_at, 1, 10) as day, COALESCE(SUM(cost), 0.0) as total FROM usage_logs GROUP BY day ORDER BY day DESC LIMIT 30")?;
    let rows = stmt.query_map([], |row| {
        Ok((row.get(0)?, row.get(1)?))
    })?;
    
    let mut result = Vec::new();
    for row in rows {
        result.push(row?);
    }
    Ok(result)
}

pub fn get_monthly_usage_by_day_for_project(conn: &Connection, project_id: i64) -> Result<Vec<(String, f64)>> {
    let mut stmt = conn.prepare("SELECT SUBSTR(created_at, 1, 10) as day, COALESCE(SUM(cost), 0.0) as total FROM usage_logs WHERE project_id = ? GROUP BY day ORDER BY day DESC LIMIT 30")?;
    let rows = stmt.query_map([project_id], |row| {
        Ok((row.get(0)?, row.get(1)?))
    })?;
    
    let mut result = Vec::new();
    for row in rows {
        result.push(row?);
    }
    Ok(result)
}

pub fn get_all_usage_logs(conn: &Connection) -> Result<Vec<UsageLog>> {
    let mut stmt = conn.prepare("SELECT id, key_id, project_id, provider, cost, created_at FROM usage_logs ORDER BY created_at DESC LIMIT 100")?;
    let logs = stmt.query_map([], |row| {
        Ok(UsageLog {
            id: row.get(0)?,
            key_id: row.get(1)?,
            project_id: row.get(2)?,
            provider: row.get(3)?,
            cost: row.get(4)?,
            created_at: row.get(5)?,
        })
    })?;
    
    let mut result = Vec::new();
    for log in logs {
        result.push(log?);
    }
    Ok(result)
}

pub fn get_setting(conn: &Connection, key: &str) -> Result<Option<String>> {
    conn.query_row("SELECT value FROM settings WHERE key = ?", params![key], |row| {
        row.get(0)
    }).map(Some).or_else(|e| {
        if e == rusqlite::Error::QueryReturnedNoRows {
            Ok(None)
        } else {
            Err(e)
        }
    })
}

pub fn set_setting(conn: &Connection, key: &str, value: &str) -> Result<()> {
    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value) VALUES (?, ?)",
        params![key, value],
    )?;
    Ok(())
}

pub fn get_all_models(conn: &Connection) -> Result<Vec<Model>> {
    let mut stmt = conn.prepare("SELECT id, name, provider, tags, enabled FROM models ORDER BY name")?;
    let models = stmt.query_map([], |row| {
        Ok(Model {
            id: row.get(0)?,
            name: row.get(1)?,
            provider: row.get(2)?,
            tags: row.get(3)?,
            enabled: row.get(4)?,
            sort_order: None,
        })
    })?;
    
    let mut result = Vec::new();
    for model in models {
        result.push(model?);
    }
    Ok(result)
}

pub fn get_models_by_provider(conn: &Connection, provider: &str) -> Result<Vec<Model>> {
    let mut stmt = conn.prepare("SELECT id, name, provider, tags, enabled FROM models WHERE provider = ? AND enabled = 1 ORDER BY name")?;
    let models = stmt.query_map(params![provider], |row| {
        Ok(Model {
            id: row.get(0)?,
            name: row.get(1)?,
            provider: row.get(2)?,
            tags: row.get(3)?,
            enabled: row.get(4)?,
            sort_order: None,
        })
    })?;
    
    let mut result = Vec::new();
    for model in models {
        result.push(model?);
    }
    Ok(result)
}

pub fn insert_model(conn: &Connection, name: &str, provider: &str, tags: &str) -> Result<i64> {
    conn.execute(
        "INSERT INTO models (name, provider, tags) VALUES (?, ?, ?)",
        params![name, provider, tags],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn delete_model(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("DELETE FROM models WHERE id = ?", params![id])?;
    Ok(())
}

pub fn toggle_model_enabled(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("UPDATE models SET enabled = 1 - enabled WHERE id = ?", params![id])?;
    Ok(())
}

pub fn insert_call_log(conn: &Connection, project_id: i64, key_id: i64, provider: &str, remark: &str, model: &str) -> Result<i64> {
    conn.execute(
        "INSERT INTO call_logs (project_id, key_id, provider, remark, model, created_at) VALUES (?, ?, ?, ?, ?, ?)",
        params![project_id, key_id, provider, remark, model, Local::now().format("%Y-%m-%d %H:%M:%S").to_string()],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn update_call_log(conn: &Connection, id: i64, prompt_tokens: i64, completion_tokens: i64, cost: f64, status: &str, error_message: Option<String>) -> Result<()> {
    conn.execute(
        "UPDATE call_logs SET prompt_tokens = ?, completion_tokens = ?, cost = ?, status = ?, error_message = ? WHERE id = ?",
        params![prompt_tokens, completion_tokens, cost, status, error_message, id],
    )?;
    Ok(())
}

pub fn get_all_call_logs(conn: &Connection, project_id: i64) -> Result<Vec<CallLog>> {
    let mut stmt = conn.prepare("SELECT id, project_id, key_id, provider, remark, model, prompt_tokens, completion_tokens, cost, status, error_message, created_at FROM call_logs WHERE project_id = ? ORDER BY created_at DESC LIMIT 100")?;
    let logs = stmt.query_map(params![project_id], |row| {
        Ok(CallLog {
            id: row.get(0)?,
            project_id: row.get(1)?,
            key_id: row.get(2)?,
            provider: row.get(3)?,
            remark: row.get(4)?,
            model: row.get(5)?,
            prompt_tokens: row.get(6)?,
            completion_tokens: row.get(7)?,
            cost: row.get(8)?,
            status: row.get(9)?,
            error_message: row.get(10)?,
            created_at: row.get(11)?,
        })
    })?;
    
    let mut result = Vec::new();
    for log in logs {
        result.push(log?);
    }
    Ok(result)
}

pub fn get_call_logs_by_provider(conn: &Connection, project_id: i64, provider: &str) -> Result<Vec<CallLog>> {
    let mut stmt = conn.prepare("SELECT id, project_id, key_id, provider, remark, model, prompt_tokens, completion_tokens, cost, status, error_message, created_at FROM call_logs WHERE project_id = ? AND provider = ? ORDER BY created_at DESC LIMIT 100")?;
    let logs = stmt.query_map(params![project_id, provider], |row| {
        Ok(CallLog {
            id: row.get(0)?,
            project_id: row.get(1)?,
            key_id: row.get(2)?,
            provider: row.get(3)?,
            remark: row.get(4)?,
            model: row.get(5)?,
            prompt_tokens: row.get(6)?,
            completion_tokens: row.get(7)?,
            cost: row.get(8)?,
            status: row.get(9)?,
            error_message: row.get(10)?,
            created_at: row.get(11)?,
        })
    })?;
    
    let mut result = Vec::new();
    for log in logs {
        result.push(log?);
    }
    Ok(result)
}

pub fn get_call_logs_by_status(conn: &Connection, project_id: i64, status: &str) -> Result<Vec<CallLog>> {
    let mut stmt = conn.prepare("SELECT id, project_id, key_id, provider, remark, model, prompt_tokens, completion_tokens, cost, status, error_message, created_at FROM call_logs WHERE project_id = ? AND status = ? ORDER BY created_at DESC LIMIT 100")?;
    let logs = stmt.query_map(params![project_id, status], |row| {
        Ok(CallLog {
            id: row.get(0)?,
            project_id: row.get(1)?,
            key_id: row.get(2)?,
            provider: row.get(3)?,
            remark: row.get(4)?,
            model: row.get(5)?,
            prompt_tokens: row.get(6)?,
            completion_tokens: row.get(7)?,
            cost: row.get(8)?,
            status: row.get(9)?,
            error_message: row.get(10)?,
            created_at: row.get(11)?,
        })
    })?;
    
    let mut result = Vec::new();
    for log in logs {
        result.push(log?);
    }
    Ok(result)
}

pub fn get_call_logs_by_provider_and_status(conn: &Connection, project_id: i64, provider: &str, status: &str) -> Result<Vec<CallLog>> {
    let mut stmt = conn.prepare("SELECT id, project_id, key_id, provider, remark, model, prompt_tokens, completion_tokens, cost, status, error_message, created_at FROM call_logs WHERE project_id = ? AND provider = ? AND status = ? ORDER BY created_at DESC LIMIT 100")?;
    let logs = stmt.query_map(params![project_id, provider, status], |row| {
        Ok(CallLog {
            id: row.get(0)?,
            project_id: row.get(1)?,
            key_id: row.get(2)?,
            provider: row.get(3)?,
            remark: row.get(4)?,
            model: row.get(5)?,
            prompt_tokens: row.get(6)?,
            completion_tokens: row.get(7)?,
            cost: row.get(8)?,
            status: row.get(9)?,
            error_message: row.get(10)?,
            created_at: row.get(11)?,
        })
    })?;
    
    let mut result = Vec::new();
    for log in logs {
        result.push(log?);
    }
    Ok(result)
}

pub fn get_call_logs_for_export(conn: &Connection, project_id: i64, start_date: &str, end_date: &str) -> Result<Vec<CallLog>> {
    let mut stmt = conn.prepare("SELECT id, project_id, key_id, provider, remark, model, prompt_tokens, completion_tokens, cost, status, error_message, created_at FROM call_logs WHERE project_id = ? AND created_at >= ? AND created_at <= ? ORDER BY created_at DESC")?;
    let logs = stmt.query_map(params![project_id, format!("{} 00:00:00", start_date), format!("{} 23:59:59", end_date)], |row| {
        Ok(CallLog {
            id: row.get(0)?,
            project_id: row.get(1)?,
            key_id: row.get(2)?,
            provider: row.get(3)?,
            remark: row.get(4)?,
            model: row.get(5)?,
            prompt_tokens: row.get(6)?,
            completion_tokens: row.get(7)?,
            cost: row.get(8)?,
            status: row.get(9)?,
            error_message: row.get(10)?,
            created_at: row.get(11)?,
        })
    })?;
    
    let mut result = Vec::new();
    for log in logs {
        result.push(log?);
    }
    Ok(result)
}

pub fn clear_call_logs(conn: &Connection, project_id: i64) -> Result<()> {
    conn.execute("DELETE FROM call_logs WHERE project_id = ?", params![project_id])?;
    Ok(())
}

pub fn clear_call_logs_all(conn: &Connection) -> Result<()> {
    conn.execute("DELETE FROM call_logs", [])?;
    Ok(())
}

pub fn get_all_projects(conn: &Connection) -> Result<Vec<Project>> {
    let mut stmt = conn.prepare("SELECT id, name, description, monthly_budget, created_at FROM projects ORDER BY created_at DESC")?;
    let projects = stmt.query_map([], |row| {
        Ok(Project {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            monthly_budget: row.get(3)?,
            created_at: row.get(4)?,
            uuid: None,
            is_deleted: None,
        })
    })?;
    
    let mut result = Vec::new();
    for project in projects {
        result.push(project?);
    }
    Ok(result)
}

pub fn get_project(conn: &Connection, project_id: i64) -> Result<Option<Project>> {
    conn.query_row("SELECT id, name, description, monthly_budget, created_at FROM projects WHERE id = ?", params![project_id], |row| {
        Ok(Project {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            monthly_budget: row.get(3)?,
            created_at: row.get(4)?,
            uuid: None,
            is_deleted: None,
        })
    }).map(Some).or_else(|e| {
        if e == rusqlite::Error::QueryReturnedNoRows {
            Ok(None)
        } else {
            Err(e)
        }
    })
}

pub fn create_project(conn: &Connection, name: &str, description: &str) -> Result<i64> {
    conn.execute(
        "INSERT INTO projects (name, description, monthly_budget, created_at) VALUES (?, ?, ?, ?)",
        params![name, description, 0.0, Local::now().format("%Y-%m-%d %H:%M:%S").to_string()],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn update_project(conn: &Connection, project_id: i64, name: &str, description: &str, monthly_budget: f64) -> Result<()> {
    conn.execute(
        "UPDATE projects SET name = ?, description = ?, monthly_budget = ? WHERE id = ?",
        params![name, description, monthly_budget, project_id],
    )?;
    Ok(())
}

pub fn delete_project(conn: &Connection, project_id: i64) -> Result<()> {
    conn.execute("DELETE FROM keys WHERE project_id = ?", params![project_id])?;
    conn.execute("DELETE FROM usage_logs WHERE project_id = ?", params![project_id])?;
    conn.execute("DELETE FROM call_logs WHERE project_id = ?", params![project_id])?;
    conn.execute("DELETE FROM notifications WHERE project_id = ?", params![project_id])?;
    conn.execute("DELETE FROM routing_rules WHERE project_id = ?", params![project_id])?;
    conn.execute("DELETE FROM projects WHERE id = ?", params![project_id])?;
    Ok(())
}

pub fn get_all_notifications(conn: &Connection) -> Result<Vec<Notification>> {
    let mut stmt = conn.prepare("SELECT id, project_id, type, title, content, is_read, created_at FROM notifications ORDER BY created_at DESC")?;
    let notifications = stmt.query_map([], |row| {
        Ok(Notification {
            id: row.get(0)?,
            project_id: row.get(1)?,
            r#type: row.get(2)?,
            title: row.get(3)?,
            content: row.get(4)?,
            is_read: row.get(5)?,
            created_at: row.get(6)?,
        })
    })?;
    
    let mut result = Vec::new();
    for notification in notifications {
        result.push(notification?);
    }
    Ok(result)
}

pub fn get_unread_notifications_count(conn: &Connection) -> Result<i64> {
    conn.query_row("SELECT COUNT(*) FROM notifications WHERE is_read = 0", [], |row| row.get(0))
}

pub fn mark_notification_as_read(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("UPDATE notifications SET is_read = 1 WHERE id = ?", params![id])?;
    Ok(())
}

pub fn mark_all_notifications_as_read(conn: &Connection) -> Result<()> {
    conn.execute("UPDATE notifications SET is_read = 1 WHERE is_read = 0", [])?;
    Ok(())
}

pub fn clear_all_notifications(conn: &Connection) -> Result<()> {
    conn.execute("DELETE FROM notifications", [])?;
    Ok(())
}

pub fn insert_notification(conn: &Connection, project_id: i64, r#type: &str, title: &str, content: &str) -> Result<i64> {
    conn.execute(
        "INSERT INTO notifications (project_id, type, title, content, created_at) VALUES (?, ?, ?, ?, ?)",
        params![project_id, r#type, title, content, Local::now().format("%Y-%m-%d %H:%M:%S").to_string()],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn get_routing_rules(conn: &Connection, project_id: i64) -> Result<Vec<RoutingRule>> {
    let mut stmt = conn.prepare("SELECT id, project_id, rule_name, match_type, match_content, target_provider, target_model, priority, is_enabled FROM routing_rules WHERE project_id = ? ORDER BY priority DESC")?;
    let rules = stmt.query_map(params![project_id], |row| {
        Ok(RoutingRule {
            id: row.get(0)?,
            project_id: row.get(1)?,
            rule_name: row.get(2)?,
            match_type: row.get(3)?,
            match_content: row.get(4)?,
            target_provider: row.get(5)?,
            target_model: row.get(6)?,
            priority: row.get(7)?,
            is_enabled: row.get(8)?,
        })
    })?;
    
    let mut result = Vec::new();
    for rule in rules {
        result.push(rule?);
    }
    Ok(result)
}

pub fn insert_routing_rule(conn: &Connection, project_id: i64, rule_name: &str, match_type: &str, match_content: &str, target_provider: &str, target_model: &str) -> Result<i64> {
    conn.execute(
        "INSERT INTO routing_rules (project_id, rule_name, match_type, match_content, target_provider, target_model) VALUES (?, ?, ?, ?, ?, ?)",
        params![project_id, rule_name, match_type, match_content, target_provider, target_model],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn update_routing_rule(conn: &Connection, id: i64, update: &RoutingRuleUpdate) -> Result<()> {
    conn.execute(
        "UPDATE routing_rules SET rule_name = ?, match_type = ?, match_content = ?, target_provider = ?, target_model = ?, priority = ?, is_enabled = ? WHERE id = ?",
        params![update.rule_name, update.match_type, update.match_content, update.target_provider, update.target_model, update.priority, update.is_enabled, id],
    )?;
    Ok(())
}

pub fn delete_routing_rule(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("DELETE FROM routing_rules WHERE id = ?", params![id])?;
    Ok(())
}
