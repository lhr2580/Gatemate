pub mod crypto;
pub mod db;
pub mod providers;
mod server;
pub mod plugin_loader;

use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tauri::{Emitter, Manager, State};

#[tauri::command]
fn save_key(project_id: i64, provider: String, api_key: String, remark: String, daily_limit: f64, monthly_limit: f64, conn: State<'_, Arc<Mutex<Connection>>>) -> String {
    let conn = conn.lock().unwrap();
    let master = crypto::get_master_key();
    let encrypted = match crypto::encrypt(&api_key, &master) {
        Ok(e) => e,
        Err(e) => return format!("加密失败: {}", e),
    };
    match db::insert_key(&conn, project_id, &provider, &encrypted, &remark, daily_limit, monthly_limit) {
        Ok(_) => "success".to_string(),
        Err(e) => format!("失败: {}", e),
    }
}

#[tauri::command]
fn get_keys(project_id: i64, conn: State<'_, Arc<Mutex<Connection>>>) -> Vec<db::ApiKey> {
    let conn = conn.lock().unwrap();
    match db::get_all_keys(&conn, project_id) {
        Ok(keys) => keys,
        Err(e) => {
            eprintln!("⚠️ 读取 Key 失败: {}", e);
            Vec::new()
        }
    }
}

#[tauri::command]
fn delete_key(id: i64, conn: State<'_, Arc<Mutex<Connection>>>) -> String {
    let conn = conn.lock().unwrap();
    match db::delete_key(&conn, id) {
        Ok(_) => "success".to_string(),
        Err(e) => format!("失败: {}", e),
    }
}

#[tauri::command]
fn get_daily_stats(project_id: i64, conn: State<'_, Arc<Mutex<Connection>>>) -> Result<serde_json::Value, String> {
    let conn = conn.lock().unwrap();
    let daily = db::get_project_daily_usage(&conn, project_id).map_err(|e| format!("{}", e))?;
    let monthly = db::get_project_monthly_usage(&conn, project_id).map_err(|e| format!("{}", e))?;
    let daily_logs = db::get_monthly_usage_by_day_for_project(&conn, project_id).map_err(|e| format!("{}", e))?;
    
    let labels: Vec<String> = daily_logs.iter().map(|(d, _)| d.clone()).collect();
    let data: Vec<f64> = daily_logs.iter().map(|(_, c)| *c).collect();
    
    Ok(serde_json::json!({
        "daily_cost": daily,
        "monthly_cost": monthly,
        "chart_labels": labels,
        "chart_data": data
    }))
}

#[tauri::command]
fn get_daily_usage(id: i64, conn: State<'_, Arc<Mutex<Connection>>>) -> f64 {
    let conn = conn.lock().unwrap();
    match db::get_daily_usage(&conn, id) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("⚠️ 读取日用量失败: {}", e);
            0.0
        }
    }
}

#[tauri::command]
fn get_monthly_usage(id: i64, conn: State<'_, Arc<Mutex<Connection>>>) -> f64 {
    let conn = conn.lock().unwrap();
    match db::get_monthly_usage(&conn, id) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("⚠️ 读取月用量失败: {}", e);
            0.0
        }
    }
}

#[tauri::command]
fn set_key_status(id: i64, status: String, conn: State<'_, Arc<Mutex<Connection>>>) -> String {
    let conn = conn.lock().unwrap();
    match db::set_key_status(&conn, id, &status) {
        Ok(_) => "success".to_string(),
        Err(e) => format!("失败: {}", e),
    }
}

#[tauri::command]
fn test_key(id: i64, conn: State<'_, Arc<Mutex<Connection>>>) -> String {
    let (provider, encrypted_key) = {
        let conn = conn.lock().unwrap();
        let key = match db::get_key_by_id(&conn, id) {
            Ok(k) => k,
            Err(e) => return format!("获取密钥失败: {}", e),
        };
        (key.provider, key.encrypted_key)
    };
    
    let master = crypto::get_master_key();
    let decrypted_key = match crypto::decrypt(&encrypted_key, &master) {
        Ok(k) => k,
        Err(e) => return format!("解密密钥失败: {}", e),
    };
    
    let url = match provider.as_str() {
        "openai" => "https://api.openai.com/v1/models",
        "deepseek" => "https://api.deepseek.com/v1/models",
        "qwen" => "https://dashscope.aliyuncs.com/api/v1/services/aigc/text-generation/generation",
        "anthropic" => "https://api.anthropic.com/v1/models",
        "gemini" => "https://generativelanguage.googleapis.com/v1beta/models",
        "doubao" => "https://api.doubao.com/v1/models",
        "yiyan" => "https://aip.baidubce.com/rpc/2.0/ai_custom/v1/wenxinworkshop/chat/completions_pro",
        _ => return "不支持的供应商".to_string(),
    };
    
    let response = std::thread::spawn(move || {
        let client = reqwest::blocking::Client::new();
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        
        match provider.as_str() {
            "openai" | "deepseek" | "qwen" | "doubao" | "yiyan" => {
                headers.insert("Authorization", format!("Bearer {}", decrypted_key).parse().unwrap());
            }
            "anthropic" => {
                headers.insert("x-api-key", decrypted_key.parse().unwrap());
            }
            "gemini" => {
                headers.insert("x-goog-api-key", decrypted_key.parse().unwrap());
            }
            _ => {}
        }
        
        client.get(url).headers(headers).send()
    }).join().unwrap();
    
    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                "测试成功".to_string()
            } else {
                let error_text = resp.text().unwrap_or("未知错误".to_string());
                format!("测试失败: {}", error_text)
            }
        }
        Err(e) => format!("网络请求失败: {}", e),
    }
}

#[tauri::command]
fn update_key(id: i64, provider: String, api_key: Option<String>, remark: String, daily_limit: f64, monthly_limit: f64, conn: State<'_, Arc<Mutex<Connection>>>) -> String {
    let conn = conn.lock().unwrap();
    
    let encrypted_key: Option<String> = if let Some(key) = api_key {
        let master = crypto::get_master_key();
        match crypto::encrypt(&key, &master) {
            Ok(e) => Some(e),
            Err(e) => return format!("加密失败: {}", e),
        }
    } else {
        None
    };
    
    let result = if let Some(ref e_key) = encrypted_key {
        db::update_key(&conn, id, &provider, Some(e_key), &remark, daily_limit, monthly_limit)
    } else {
        db::update_key(&conn, id, &provider, None, &remark, daily_limit, monthly_limit)
    };
    
    match result {
        Ok(_) => "success".to_string(),
        Err(e) => format!("失败: {}", e),
    }
}

#[tauri::command]
fn get_usage_logs(conn: State<'_, Arc<Mutex<Connection>>>) -> Vec<db::UsageLog> {
    let conn = conn.lock().unwrap();
    match db::get_all_usage_logs(&conn) {
        Ok(logs) => logs,
        Err(e) => {
            eprintln!("⚠️ 读取日志失败: {}", e);
            Vec::new()
        }
    }
}

#[tauri::command]
fn get_route_strategy(conn: State<'_, Arc<Mutex<Connection>>>) -> String {
    let conn = conn.lock().unwrap();
    match db::get_setting(&conn, "route_strategy") {
        Ok(Some(s)) => s,
        Ok(None) => "round_robin".to_string(),
        Err(e) => {
            eprintln!("⚠️ 读取路由策略失败: {}", e);
            "round_robin".to_string()
        }
    }
}

#[tauri::command]
fn set_route_strategy(strategy: String, conn: State<'_, Arc<Mutex<Connection>>>) -> String {
    let conn = conn.lock().unwrap();
    match db::set_setting(&conn, "route_strategy", &strategy) {
        Ok(_) => "success".to_string(),
        Err(e) => format!("失败: {}", e),
    }
}

#[tauri::command]
fn get_notification_enabled(conn: State<'_, Arc<Mutex<Connection>>>) -> bool {
    let conn = conn.lock().unwrap();
    match db::get_setting(&conn, "notification_enabled") {
        Ok(Some(s)) => s == "true",
        Ok(None) => true,
        Err(e) => {
            eprintln!("⚠️ 读取通知设置失败: {}", e);
            true
        }
    }
}

#[tauri::command]
fn set_notification_enabled(enabled: bool, conn: State<'_, Arc<Mutex<Connection>>>) -> String {
    let conn = conn.lock().unwrap();
    match db::set_setting(&conn, "notification_enabled", &enabled.to_string()) {
        Ok(_) => "success".to_string(),
        Err(e) => format!("失败: {}", e),
    }
}

#[tauri::command]
fn get_notifications(conn: State<'_, Arc<Mutex<Connection>>>) -> Vec<db::Notification> {
    let conn = conn.lock().unwrap();
    match db::get_all_notifications(&conn) {
        Ok(notifications) => notifications,
        Err(e) => {
            eprintln!("⚠️ 读取通知列表失败: {}", e);
            Vec::new()
        }
    }
}

#[tauri::command]
fn get_unread_notifications_count(conn: State<'_, Arc<Mutex<Connection>>>) -> i64 {
    let conn = conn.lock().unwrap();
    match db::get_unread_notifications_count(&conn) {
        Ok(count) => count,
        Err(e) => {
            eprintln!("⚠️ 读取未读通知数量失败: {}", e);
            0
        }
    }
}

#[tauri::command]
fn mark_notification_as_read(id: i64, conn: State<'_, Arc<Mutex<Connection>>>) -> String {
    let conn = conn.lock().unwrap();
    match db::mark_notification_as_read(&conn, id) {
        Ok(_) => "success".to_string(),
        Err(e) => format!("失败: {}", e),
    }
}

#[tauri::command]
fn mark_all_notifications_as_read(conn: State<'_, Arc<Mutex<Connection>>>) -> String {
    let conn = conn.lock().unwrap();
    match db::mark_all_notifications_as_read(&conn) {
        Ok(_) => "success".to_string(),
        Err(e) => format!("失败: {}", e),
    }
}

#[tauri::command]
fn clear_all_notifications(conn: State<'_, Arc<Mutex<Connection>>>) -> String {
    let conn = conn.lock().unwrap();
    match db::clear_all_notifications(&conn) {
        Ok(_) => "success".to_string(),
        Err(e) => format!("失败: {}", e),
    }
}

#[tauri::command]
fn insert_notification(project_id: i64, r#type: String, title: String, content: String, conn: State<'_, Arc<Mutex<Connection>>>) -> String {
    let conn = conn.lock().unwrap();
    match db::insert_notification(&conn, project_id, &r#type, &title, &content) {
        Ok(_) => "success".to_string(),
        Err(e) => format!("失败: {}", e),
    }
}

#[tauri::command]
fn get_models(conn: State<'_, Arc<Mutex<Connection>>>) -> Vec<db::Model> {
    let conn = conn.lock().unwrap();
    match db::get_all_models(&conn) {
        Ok(models) => models,
        Err(e) => {
            eprintln!("⚠️ 读取模型列表失败: {}", e);
            Vec::new()
        }
    }
}

#[tauri::command]
fn add_model(name: String, provider: String, tags: String, conn: State<'_, Arc<Mutex<Connection>>>) -> String {
    let conn = conn.lock().unwrap();
    match db::insert_model(&conn, &name, &provider, &tags) {
        Ok(_) => "success".to_string(),
        Err(e) => format!("失败: {}", e),
    }
}

#[tauri::command]
fn get_models_by_provider(provider: String, conn: State<'_, Arc<Mutex<Connection>>>) -> Vec<db::Model> {
    let conn = conn.lock().unwrap();
    match db::get_models_by_provider(&conn, &provider) {
        Ok(models) => models,
        Err(e) => {
            eprintln!("⚠️ 读取模型列表失败: {}", e);
            Vec::new()
        }
    }
}

#[tauri::command]
fn delete_model(id: i64, conn: State<'_, Arc<Mutex<Connection>>>) -> String {
    let conn = conn.lock().unwrap();
    match db::delete_model(&conn, id) {
        Ok(_) => "success".to_string(),
        Err(e) => format!("失败: {}", e),
    }
}

#[tauri::command]
fn toggle_model(id: i64, conn: State<'_, Arc<Mutex<Connection>>>) -> String {
    let conn = conn.lock().unwrap();
    match db::toggle_model_enabled(&conn, id) {
        Ok(_) => "success".to_string(),
        Err(e) => format!("失败: {}", e),
    }
}

#[tauri::command]
fn get_call_logs(project_id: i64, conn: State<'_, Arc<Mutex<Connection>>>) -> Vec<db::CallLog> {
    let conn = conn.lock().unwrap();
    match db::get_all_call_logs(&conn, project_id) {
        Ok(logs) => logs,
        Err(e) => {
            eprintln!("⚠️ 读取调用日志失败: {}", e);
            Vec::new()
        }
    }
}

#[tauri::command]
fn get_call_logs_filtered(project_id: i64, provider: Option<String>, status: Option<String>, conn: State<'_, Arc<Mutex<Connection>>>) -> Vec<db::CallLog> {
    let conn = conn.lock().unwrap();
    match (provider, status) {
        (Some(p), Some(s)) => match db::get_call_logs_by_provider_and_status(&conn, project_id, &p, &s) {
            Ok(logs) => logs,
            Err(e) => {
                eprintln!("⚠️ 读取调用日志失败: {}", e);
                Vec::new()
            }
        },
        (Some(p), None) => match db::get_call_logs_by_provider(&conn, project_id, &p) {
            Ok(logs) => logs,
            Err(e) => {
                eprintln!("⚠️ 读取调用日志失败: {}", e);
                Vec::new()
            }
        },
        (None, Some(s)) => match db::get_call_logs_by_status(&conn, project_id, &s) {
            Ok(logs) => logs,
            Err(e) => {
                eprintln!("⚠️ 读取调用日志失败: {}", e);
                Vec::new()
            }
        },
        (None, None) => match db::get_all_call_logs(&conn, project_id) {
            Ok(logs) => logs,
            Err(e) => {
                eprintln!("⚠️ 读取调用日志失败: {}", e);
                Vec::new()
            }
        },
    }
}

#[tauri::command]
fn clear_call_logs(conn: State<'_, Arc<Mutex<Connection>>>) -> String {
    let conn = conn.lock().unwrap();
    match db::clear_call_logs_all(&conn) {
        Ok(_) => "success".to_string(),
        Err(e) => format!("失败: {}", e),
    }
}

#[tauri::command]
fn send_notification(title: String, body: String, app: tauri::AppHandle) -> String {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.emit("notification", serde_json::json!({
            "title": title,
            "body": body
        }));
    }
    "success".to_string()
}

#[tauri::command]
fn export_csv(project_id: i64, start_date: String, end_date: String, conn: State<'_, Arc<Mutex<Connection>>>) -> Result<String, String> {
    let conn = conn.lock().unwrap();
    let logs = db::get_call_logs_for_export(&conn, project_id, &start_date, &end_date).map_err(|e| format!("{}", e))?;
    
    let mut csv = "ID,时间,供应商,备注,模型,Prompt Token,Completion Token,花费(美元),状态,错误信息\n".to_string();
    for log in logs {
        let error = log.error_message.clone().unwrap_or_default().replace(",", "");
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{:.4},{},{}\n",
            log.id,
            log.created_at,
            log.provider,
            log.remark.replace(",", ""),
            log.model,
            log.prompt_tokens,
            log.completion_tokens,
            log.cost,
            log.status,
            error
        ));
    }
    
    Ok(csv)
}

#[tauri::command]
fn export_pdf(project_id: i64, start_date: String, end_date: String, conn: State<'_, Arc<Mutex<Connection>>>) -> Result<String, String> {
    let conn = conn.lock().unwrap();
    let logs = db::get_call_logs_for_export(&conn, project_id, &start_date, &end_date).map_err(|e| format!("{}", e))?;
    
    let app_data_dir = get_app_data_dir();
    let pdf_path = app_data_dir.join("backups").join(format!("report_{}.pdf", chrono::Local::now().format("%Y%m%d_%H%M%S")));
    
    if let Err(e) = std::fs::create_dir_all(app_data_dir.join("backups")) {
        return Err(format!("创建备份目录失败: {}", e));
    }
    
    use printpdf::{PdfDocument, Mm, BuiltinFont};
    use std::io::BufWriter;
    
    let (doc, page1, layer1) = PdfDocument::new("GateMate Report", Mm(210.0), Mm(297.0), "Layer 1");
    let font = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();
    
    let current_layer = doc.get_page(page1).get_layer(layer1);
    let mut y_pos = Mm(270.0);
    
    current_layer.use_text("GateMate Report", 24.0, Mm(10.0), y_pos, &font);
    y_pos -= Mm(30.0);
    
    current_layer.use_text(format!("Project ID: {}", project_id), 12.0, Mm(10.0), y_pos, &font);
    y_pos -= Mm(15.0);
    
    current_layer.use_text(format!("Date Range: {} ~ {}", start_date, end_date), 12.0, Mm(10.0), y_pos, &font);
    y_pos -= Mm(15.0);
    
    current_layer.use_text(format!("Generated: {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S")), 12.0, Mm(10.0), y_pos, &font);
    y_pos -= Mm(30.0);
    
    let headers = ["Time", "Provider", "Model", "Prompt", "Completion", "Cost"];
    let col_widths = [Mm(40.0), Mm(50.0), Mm(50.0), Mm(30.0), Mm(30.0), Mm(30.0)];
    
    let mut x_pos = Mm(10.0);
    for (header, width) in headers.iter().zip(col_widths.iter()) {
        current_layer.use_text(*header, 10.0, x_pos, y_pos, &font);
        x_pos += *width;
    }
    y_pos -= Mm(15.0);
    
    let mut total_cost = 0.0;
    
    for log in &logs {
        if y_pos < Mm(30.0) {
            let (new_page, new_layer) = doc.add_page(Mm(210.0), Mm(297.0), "Layer 1");
            let current_layer = doc.get_page(new_page).get_layer(new_layer);
            
            y_pos = Mm(270.0);
            x_pos = Mm(10.0);
            for (header, width) in headers.iter().zip(col_widths.iter()) {
                current_layer.use_text(*header, 10.0, x_pos, y_pos, &font);
                x_pos += *width;
            }
            y_pos -= Mm(15.0);
        }
        
        total_cost += log.cost;
        x_pos = Mm(10.0);
        
        let time_str = log.created_at.split(' ').next_back().unwrap_or(&log.created_at);
        current_layer.use_text(time_str, 8.0, x_pos, y_pos, &font);
        x_pos += col_widths[0];
        
        current_layer.use_text(&log.provider, 8.0, x_pos, y_pos, &font);
        x_pos += col_widths[1];
        
        let model_display = if log.model.len() > 15 { &log.model[..15] } else { &log.model };
        current_layer.use_text(model_display, 8.0, x_pos, y_pos, &font);
        x_pos += col_widths[2];
        
        current_layer.use_text(log.prompt_tokens.to_string(), 8.0, x_pos, y_pos, &font);
        x_pos += col_widths[3];
        
        current_layer.use_text(log.completion_tokens.to_string(), 8.0, x_pos, y_pos, &font);
        x_pos += col_widths[4];
        
        current_layer.use_text(format!("${:.4}", log.cost), 8.0, x_pos, y_pos, &font);
        y_pos -= Mm(12.0);
    }
    
    y_pos -= Mm(20.0);
    current_layer.use_text(format!("Total: {} calls, Cost ${:.4}", logs.len(), total_cost), 14.0, Mm(10.0), y_pos, &font);
    
    let file = std::fs::File::create(&pdf_path).map_err(|e| format!("创建文件失败: {}", e))?;
    let mut writer = BufWriter::new(file);
    doc.save(&mut writer).map_err(|e| format!("保存 PDF 失败: {}", e))?;
    
    Ok(pdf_path.to_str().unwrap_or("report.pdf").to_string())
}

#[tauri::command]
fn is_pro_user() -> bool {
    plugin_loader::load_plugin();
    let status = plugin_loader::get_pro_status();
    status["is_pro"].as_bool().unwrap_or(false)
}

#[tauri::command]
fn reload_plugins() -> String {
    if plugin_loader::load_plugin() {
        "Plugin loaded successfully".to_string()
    } else {
        "Plugin not found".to_string()
    }
}

#[tauri::command]
fn verify_license(license_key: String) -> serde_json::Value {
    plugin_loader::load_plugin();
    plugin_loader::verify_license(&license_key)
}

#[tauri::command]
fn get_all_projects(conn: State<'_, Arc<Mutex<Connection>>>) -> Vec<db::Project> {
    let conn = conn.lock().unwrap();
    match db::get_all_projects(&conn) {
        Ok(projects) => projects,
        Err(e) => {
            eprintln!("⚠️ 读取项目列表失败: {}", e);
            Vec::new()
        }
    }
}

#[tauri::command]
fn get_project(project_id: i64, conn: State<'_, Arc<Mutex<Connection>>>) -> Option<db::Project> {
    let conn = conn.lock().unwrap();
    match db::get_project(&conn, project_id) {
        Ok(project) => project,
        Err(e) => {
            eprintln!("⚠️ 读取项目失败: {}", e);
            None
        }
    }
}

#[tauri::command]
fn create_project(name: String, description: String, conn: State<'_, Arc<Mutex<Connection>>>) -> Result<db::Project, String> {
    let conn = conn.lock().unwrap();
    match db::create_project(&conn, &name, &description) {
        Ok(id) => {
            match db::get_project(&conn, id) {
                Ok(Some(project)) => Ok(project),
                Ok(None) => Err("创建项目失败".to_string()),
                Err(e) => {
                    eprintln!("⚠️ 获取项目失败: {}", e);
                    Err("创建项目失败".to_string())
                }
            }
        }
        Err(e) => {
            eprintln!("⚠️ 创建项目失败: {}", e);
            Err(format!("创建项目失败: {}", e))
        }
    }
}

#[tauri::command]
fn update_project(id: i64, monthly_budget: f64, conn: State<'_, Arc<Mutex<Connection>>>) -> String {
    let conn = conn.lock().unwrap();
    match db::get_project(&conn, id) {
        Ok(Some(project)) => {
            match db::update_project(&conn, id, &project.name, &project.description, monthly_budget) {
                Ok(_) => "success".to_string(),
                Err(e) => format!("失败: {}", e),
            }
        }
        Ok(None) => "项目不存在".to_string(),
        Err(e) => format!("失败: {}", e),
    }
}

#[tauri::command]
fn delete_project(project_id: i64, conn: State<'_, Arc<Mutex<Connection>>>) -> String {
    let conn = conn.lock().unwrap();
    match db::delete_project(&conn, project_id) {
        Ok(_) => "success".to_string(),
        Err(e) => format!("失败: {}", e),
    }
}

#[tauri::command]
fn get_project_monthly_usage(project_id: i64, conn: State<'_, Arc<Mutex<Connection>>>) -> f64 {
    let conn = conn.lock().unwrap();
    match db::get_project_monthly_usage(&conn, project_id) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("⚠️ 读取项目月用量失败: {}", e);
            0.0
        }
    }
}

#[tauri::command]
fn get_routing_rules(project_id: i64, conn: State<'_, Arc<Mutex<Connection>>>) -> Vec<db::RoutingRule> {
    let conn = conn.lock().unwrap();
    match db::get_routing_rules(&conn, project_id) {
        Ok(rules) => rules,
        Err(e) => {
            eprintln!("⚠️ 读取路由规则失败: {}", e);
            Vec::new()
        }
    }
}

#[tauri::command]
fn insert_routing_rule(project_id: i64, rule_name: String, match_type: String, match_content: String, target_provider: String, target_model: String, conn: State<'_, Arc<Mutex<Connection>>>) -> i64 {
    let conn = conn.lock().unwrap();
    match db::insert_routing_rule(&conn, project_id, &rule_name, &match_type, &match_content, &target_provider, &target_model) {
        Ok(id) => id,
        Err(e) => {
            eprintln!("⚠️ 创建路由规则失败: {}", e);
            -1
        }
    }
}

#[tauri::command]
#[allow(clippy::too_many_arguments)]
fn update_routing_rule(id: i64, rule_name: String, match_type: String, match_content: String, target_provider: String, target_model: String, priority: i32, is_enabled: i32, conn: State<'_, Arc<Mutex<Connection>>>) -> String {
    let conn = conn.lock().unwrap();
    let update = db::RoutingRuleUpdate {
        rule_name,
        match_type,
        match_content,
        target_provider,
        target_model,
        priority,
        is_enabled,
    };
    match db::update_routing_rule(&conn, id, &update) {
        Ok(_) => "success".to_string(),
        Err(e) => format!("失败: {}", e),
    }
}

#[tauri::command]
fn delete_routing_rule(id: i64, conn: State<'_, Arc<Mutex<Connection>>>) -> String {
    let conn = conn.lock().unwrap();
    match db::delete_routing_rule(&conn, id) {
        Ok(_) => "success".to_string(),
        Err(e) => format!("失败: {}", e),
    }
}



fn get_app_data_dir() -> PathBuf {
    #[cfg(windows)]
    {
        if let Ok(appdata) = std::env::var("LOCALAPPDATA") {
            PathBuf::from(appdata).join("gatemate")
        } else if let Ok(appdata) = std::env::var("APPDATA") {
            PathBuf::from(appdata).join("gatemate")
        } else {
            PathBuf::from(".")
        }
    }
    #[cfg(target_os = "macos")]
    {
        PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".to_string())).join("Library").join("Application Support").join("gatemate")
    }
    #[cfg(target_os = "linux")]
    {
        PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".to_string())).join(".config").join("gatemate")
    }
    #[cfg(not(any(windows, target_os = "macos", target_os = "linux")))]
    {
        PathBuf::from(".")
    }
}

pub fn run() {
    let app_data_dir = get_app_data_dir();
    if let Err(e) = std::fs::create_dir_all(app_data_dir.join("data")) {
        eprintln!("⚠️ 创建数据目录失败: {}", e);
    }
    if let Err(e) = std::fs::create_dir_all(app_data_dir.join("backups")) {
        eprintln!("⚠️ 创建备份目录失败: {}", e);
    }
    if let Err(e) = std::fs::create_dir_all(app_data_dir.join("logs")) {
        eprintln!("⚠️ 创建日志目录失败: {}", e);
    }
    
    let db_path = app_data_dir.join("data").join("gatemate.db");
    let db_path_str = db_path.to_str().unwrap_or("gatemate.db").to_string();
    
    let db_conn = match db::init_db(&db_path_str) {
        Ok(conn) => {
            println!("✅ 数据库初始化成功: {}", db_path.display());
            conn
        }
        Err(e) => {
            eprintln!("❌ 数据库初始化失败: {}", e);
            eprintln!("⚠️ 尝试删除旧数据库并重新初始化...");
            let _ = std::fs::remove_file(&db_path);
            match db::init_db(&db_path_str) {
                Ok(conn) => {
                    println!("✅ 数据库重新初始化成功");
                    conn
                }
                Err(e2) => {
                    eprintln!("❌ 数据库重新初始化失败: {}", e2);
                    return;
                }
            }
        }
    };
    
    let db_path_clone = db_path_str.clone();
    
    tauri::async_runtime::spawn(async move {
        if let Err(e) = server::start_http_server(db_path_clone).await {
            eprintln!("❌ HTTP Server 启动失败: {}", e);
        }
    });

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            save_key, get_keys, delete_key, update_key, test_key, get_daily_stats, get_daily_usage, get_monthly_usage, 
            set_key_status, get_usage_logs, get_route_strategy, set_route_strategy, 
            get_notification_enabled, set_notification_enabled, get_models, add_model, 
            delete_model, toggle_model, get_models_by_provider, get_call_logs, 
            get_call_logs_filtered, clear_call_logs, export_csv, export_pdf, send_notification,
            get_all_projects, get_project, create_project, update_project, delete_project,
            get_project_monthly_usage, get_routing_rules, insert_routing_rule, 
            update_routing_rule, delete_routing_rule,
            get_notifications, get_unread_notifications_count, mark_notification_as_read,
            mark_all_notifications_as_read, clear_all_notifications, insert_notification,
            is_pro_user, reload_plugins, verify_license
        ])
        .manage(Arc::new(Mutex::new(db_conn)))
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                let _ = window.close();
            }
        })
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.eval(
                        "console.log('🚀 GateMate 本地服务已启动: http://localhost:38080')",
                    );
                }
            }
            #[allow(unused_variables)]
            let _ = app;
            Ok(())
        })
        .run(tauri::generate_context!("./tauri.conf.json"))
        .expect("error while running tauri application");
}
