use rusqlite::Connection;
use gatemate_core::db;

fn create_test_connection() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute("PRAGMA journal_mode=WAL;", []).unwrap();
    conn.execute("PRAGMA synchronous=NORMAL;", []).unwrap();
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS projects (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT DEFAULT '',
            monthly_budget REAL DEFAULT 0.0,
            created_at TEXT NOT NULL
        )",
        [],
    ).unwrap();
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS keys (
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
        )",
        [],
    ).unwrap();
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS models (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            provider TEXT NOT NULL,
            tags TEXT DEFAULT '',
            enabled INTEGER DEFAULT 1
        )",
        [],
    ).unwrap();
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        )",
        [],
    ).unwrap();
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS routing_rules (
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
        )",
        [],
    ).unwrap();
    
    conn
}

#[test]
fn test_create_project() {
    let conn = create_test_connection();
    
    let id = db::create_project(&conn, "Test Project", "Test Description").unwrap();
    assert!(id > 0);
    
    let project = db::get_project(&conn, id).unwrap();
    assert!(project.is_some());
    assert_eq!(project.unwrap().name, "Test Project");
}

#[test]
fn test_get_project_not_found() {
    let conn = create_test_connection();
    
    let project = db::get_project(&conn, 999).unwrap();
    assert!(project.is_none());
}

#[test]
fn test_get_all_projects() {
    let conn = create_test_connection();
    
    db::create_project(&conn, "Project 1", "Desc 1").unwrap();
    db::create_project(&conn, "Project 2", "Desc 2").unwrap();
    
    let projects = db::get_all_projects(&conn).unwrap();
    assert!(projects.len() >= 2);
}

#[test]
fn test_insert_key() {
    let conn = create_test_connection();
    
    let project_id = db::create_project(&conn, "Test Project", "").unwrap();
    
    let key_id = db::insert_key(&conn, project_id, "openai", "encrypted_key", "Test Key", 10.0, 100.0).unwrap();
    assert!(key_id > 0);
    
    let key = db::get_key_by_id(&conn, key_id).unwrap();
    assert_eq!(key.provider, "openai");
    assert_eq!(key.remark, "Test Key");
}

#[test]
fn test_get_all_keys() {
    let conn = create_test_connection();
    
    let project_id = db::create_project(&conn, "Test Project", "").unwrap();
    
    db::insert_key(&conn, project_id, "openai", "key1", "Key 1", 10.0, 100.0).unwrap();
    db::insert_key(&conn, project_id, "deepseek", "key2", "Key 2", 5.0, 50.0).unwrap();
    
    let keys = db::get_all_keys(&conn, project_id).unwrap();
    assert_eq!(keys.len(), 2);
}

#[test]
fn test_delete_key() {
    let conn = create_test_connection();
    
    let project_id = db::create_project(&conn, "Test Project", "").unwrap();
    let key_id = db::insert_key(&conn, project_id, "openai", "key1", "Key 1", 10.0, 100.0).unwrap();
    
    db::delete_key(&conn, key_id).unwrap();
    
    let result = db::get_key_by_id(&conn, key_id);
    assert!(result.is_err());
}

#[test]
fn test_set_key_status() {
    let conn = create_test_connection();
    
    let project_id = db::create_project(&conn, "Test Project", "").unwrap();
    let key_id = db::insert_key(&conn, project_id, "openai", "key1", "Key 1", 10.0, 100.0).unwrap();
    
    db::set_key_status(&conn, key_id, "paused").unwrap();
    
    let key = db::get_key_by_id(&conn, key_id).unwrap();
    assert_eq!(key.status, "paused");
}

#[test]
fn test_insert_and_get_model() {
    let conn = create_test_connection();
    
    let id = db::insert_model(&conn, "gpt-4o", "OpenAI", "推荐").unwrap();
    assert!(id > 0);
    
    let models = db::get_all_models(&conn).unwrap();
    assert!(models.iter().any(|m| m.name == "gpt-4o"));
}

#[test]
fn test_toggle_model_enabled() {
    let conn = create_test_connection();
    
    let id = db::insert_model(&conn, "test-model", "OpenAI", "").unwrap();
    
    let model = db::get_all_models(&conn).unwrap().into_iter().find(|m| m.id == id).unwrap();
    assert_eq!(model.enabled, 1);
    
    db::toggle_model_enabled(&conn, id).unwrap();
    
    let model = db::get_all_models(&conn).unwrap().into_iter().find(|m| m.id == id).unwrap();
    assert_eq!(model.enabled, 0);
}

#[test]
fn test_insert_routing_rule() {
    let conn = create_test_connection();
    
    let project_id = db::create_project(&conn, "Test Project", "").unwrap();
    
    let id = db::insert_routing_rule(&conn, project_id, "Test Rule", "keyword", "test", "openai", "gpt-4").unwrap();
    assert!(id > 0);
    
    let rules = db::get_routing_rules(&conn, project_id).unwrap();
    assert_eq!(rules.len(), 1);
    assert_eq!(rules[0].rule_name, "Test Rule");
}

#[test]
fn test_update_routing_rule() {
    let conn = create_test_connection();
    
    let project_id = db::create_project(&conn, "Test Project", "").unwrap();
    let id = db::insert_routing_rule(&conn, project_id, "Test Rule", "keyword", "test", "openai", "gpt-4").unwrap();
    
    db::update_routing_rule(&conn, id, "Updated Rule", "regex", "new-pattern", "deepseek", "deepseek-chat", 1, 0).unwrap();
    
    let rules = db::get_routing_rules(&conn, project_id).unwrap();
    assert_eq!(rules[0].rule_name, "Updated Rule");
    assert_eq!(rules[0].is_enabled, 0);
}

#[test]
fn test_delete_routing_rule() {
    let conn = create_test_connection();
    
    let project_id = db::create_project(&conn, "Test Project", "").unwrap();
    let id = db::insert_routing_rule(&conn, project_id, "Test Rule", "keyword", "test", "openai", "gpt-4").unwrap();
    
    db::delete_routing_rule(&conn, id).unwrap();
    
    let rules = db::get_routing_rules(&conn, project_id).unwrap();
    assert_eq!(rules.len(), 0);
}

#[test]
fn test_settings() {
    let conn = create_test_connection();
    
    db::set_setting(&conn, "test_key", "test_value").unwrap();
    
    let value = db::get_setting(&conn, "test_key").unwrap();
    assert_eq!(value, Some("test_value".to_string()));
    
    let not_found = db::get_setting(&conn, "not_found").unwrap();
    assert_eq!(not_found, None);
}

#[test]
fn test_delete_project_cascading() {
    let conn = create_test_connection();
    
    let project_id = db::create_project(&conn, "Test Project", "").unwrap();
    db::insert_key(&conn, project_id, "openai", "key1", "Key 1", 10.0, 100.0).unwrap();
    db::insert_routing_rule(&conn, project_id, "Test Rule", "keyword", "test", "openai", "gpt-4").unwrap();
    
    db::delete_project(&conn, project_id).unwrap();
    
    let keys = db::get_all_keys(&conn, project_id).unwrap();
    assert_eq!(keys.len(), 0);
    
    let rules = db::get_routing_rules(&conn, project_id).unwrap();
    assert_eq!(rules.len(), 0);
    
    let project = db::get_project(&conn, project_id).unwrap();
    assert!(project.is_none());
}