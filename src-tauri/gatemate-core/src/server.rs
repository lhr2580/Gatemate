use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use reqwest::Client;
use rusqlite::Connection;
use futures_util::{StreamExt, SinkExt};
use serde_json::{json, Value, Map};
use tokio::io::AsyncWriteExt;

use crate::{db, crypto, providers};

struct RouteStrategy {
    strategy: String,
    round_robin_index: usize,
}

pub async fn start_http_server(db_path: String) -> Result<(), Box<dyn std::error::Error>> {
    let port = std::env::var("GATEMATE_SERVER_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(38080);
    
    let bind_addr = std::env::var("GATEMATE_BIND_ADDRESS")
        .ok()
        .unwrap_or_else(|| "127.0.0.1".to_string());
    
    let addr = format!("{}:{}", bind_addr, port);
    let listener = TcpListener::bind(&addr).await?;
    println!("🚀 GateMate HTTP Server 已启动: http://{}", addr);
    
    let db_arc = Arc::new(Mutex::new(Connection::open(&db_path)?));
    let route_strategy = Arc::new(Mutex::new(RouteStrategy {
        strategy: "round_robin".to_string(),
        round_robin_index: 0,
    }));
    
    let (_shutdown_tx, mut shutdown_rx) = mpsc::channel::<()>(1);
    
    tokio::spawn(async move {
        loop {
            tokio::select! {
                Ok((stream, addr)) = listener.accept() => {
                    let db_arc_clone = Arc::clone(&db_arc);
                    let route_strategy_clone = Arc::clone(&route_strategy);
                    let db_path_clone = db_path.clone();
                    
                    tokio::spawn(async move {
                        if let Err(e) = handle_connection(stream, addr, db_arc_clone, route_strategy_clone, db_path_clone).await {
                            eprintln!("⚠️ 连接处理失败: {}", e);
                        }
                    });
                }
                _ = shutdown_rx.recv() => {
                    println!("🛑 HTTP Server 正在关闭...");
                    break;
                }
            }
        }
    });
    
    Ok(())
}

async fn handle_connection(
    stream: TcpStream,
    _addr: std::net::SocketAddr,
    db_arc: Arc<Mutex<Connection>>,
    route_strategy: Arc<Mutex<RouteStrategy>>,
    _db_path: String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut buf = [0; 1024];
    let n = stream.peek(&mut buf).await?;
    
    if n == 0 {
        return Ok(());
    }
    
    let request_str = String::from_utf8_lossy(&buf[..n]);
    
    if request_str.contains("Upgrade: websocket") || request_str.contains("upgrade: websocket") {
        let ws_stream = accept_async(stream).await?;
        let (mut write, mut read) = ws_stream.split();
        
        loop {
            let msg = read.next().await;
            if msg.is_none() {
                break;
            }
            
            let msg = msg.unwrap()?;
            
            if msg.is_text() {
                let text = msg.to_text()?;
                let request: Value = serde_json::from_str(text)?;
                
                let project_id = request["project_id"].as_i64().unwrap_or(0);
                let provider = request["provider"].as_str().unwrap_or("");
                let model = request["model"].as_str().unwrap_or("");
                let _api_key = request["api_key"].as_str().unwrap_or("");
                let remark = request["remark"].as_str().unwrap_or("");
                let user_message = request["message"].as_str().unwrap_or("");
                let stream_mode = request["stream"].as_bool().unwrap_or(false);
                
                let selected_key = select_key(&db_arc, project_id, provider, &route_strategy, user_message).await;
                
                if selected_key.is_none() {
                    let response = json!({
                        "type": "error",
                        "message": "无法选择合适的 API Key"
                    });
                    write.send(Message::Text(response.to_string())).await?;
                    continue;
                }
                
                let selected_key = selected_key.unwrap();
                let master = crypto::get_master_key();
                let decrypted_key = match crypto::decrypt(&selected_key.encrypted_key, &master) {
                    Ok(k) => k,
                    Err(e) => {
                        eprintln!("❌ 解密密钥失败: {}", e);
                        let response = json!({
                            "type": "error",
                            "message": format!("密钥解密失败: {}", e)
                        });
                        write.send(Message::Text(response.to_string())).await?;
                        continue;
                    }
                };
                
                let budget_error = check_budget(&db_arc, project_id);
                if !budget_error.is_empty() {
                    let response = json!({
                        "type": "error",
                        "message": budget_error
                    });
                    write.send(Message::Text(response.to_string())).await?;
                    continue;
                }
                
                let client = Client::new();
                let url = get_provider_url(provider, model);
                
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert("Content-Type", "application/json".parse().unwrap());
                
                match provider {
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
                
                let request_body = json!({
                    "model": model,
                    "messages": [{"role": "user", "content": user_message}],
                    "stream": stream_mode
                });
                
                let call_log_id = insert_call_log(&db_arc, project_id, selected_key.id, provider, remark, model);
                
                if stream_mode {
                    let response = send_request_with_retry(&client, &url, &headers, &request_body, 3, 120).await;
                    
                    if let Some(mut response) = response {
                        if response.status().is_success() {
                            let mut buffer = String::new();
                            let mut prompt_tokens = 0;
                            let mut completion_tokens = 0;
                            
                            while let Some(chunk) = response.chunk().await? {
                                let chunk_str = String::from_utf8_lossy(&chunk);
                                buffer.push_str(&chunk_str);
                                
                                for line in chunk_str.lines() {
                                    if let Some(data) = line.strip_prefix("data: ") {
                                        if data == "[DONE]" {
                                            break;
                                        }
                                        if let Ok(parsed) = serde_json::from_str::<Value>(data) {
                                            if let Some(usage) = parsed["usage"].as_object() {
                                                prompt_tokens = usage["prompt_tokens"].as_i64().unwrap_or(0);
                                                completion_tokens = usage["completion_tokens"].as_i64().unwrap_or(0);
                                            }
                                            let delta = parsed["choices"][0]["delta"].clone();
                                            let content = delta["content"].as_str().unwrap_or("");
                                            if !content.is_empty() {
                                                write.send(Message::Text(json!({
                                                    "type": "stream",
                                                    "content": content,
                                                    "id": call_log_id,
                                                }).to_string())).await?;
                                            }
                                        }
                                    }
                                }
                            }
                            
                            let total_cost = providers::calculate_cost(provider, model, prompt_tokens, completion_tokens);
                            update_call_log(&db_arc, call_log_id, prompt_tokens, completion_tokens, total_cost, "success", None);
                            update_key_usage(&db_arc, selected_key.id, total_cost);
                            
                            write.send(Message::Text(json!({
                                "type": "end",
                                "id": call_log_id,
                                "usage": {
                                    "prompt_tokens": prompt_tokens,
                                    "completion_tokens": completion_tokens,
                                    "total_cost": total_cost
                                }
                            }).to_string())).await?;
                        } else {
                            let error_text = response.text().await?;
                            update_call_log(&db_arc, call_log_id, 0, 0, 0.0, "failed", Some(error_text.clone()));
                            write.send(Message::Text(json!({
                                "type": "error",
                                "message": error_text,
                                "id": call_log_id
                            }).to_string())).await?;
                        }
                    } else {
                        update_call_log(&db_arc, call_log_id, 0, 0, 0.0, "failed", Some("网络请求失败".to_string()));
                        write.send(Message::Text(json!({
                            "type": "error",
                            "message": "网络请求失败",
                            "id": call_log_id
                        }).to_string())).await?;
                    }
                } else {
                    let response = send_request_with_retry(&client, &url, &headers, &request_body, 3, 60).await;
                    
                    if let Some(response) = response {
                        if response.status().is_success() {
                            let response_json: Value = response.json().await?;
                            let empty_map = Map::new();
                            let usage = response_json["usage"].as_object().unwrap_or(&empty_map);
                            let prompt_tokens = usage["prompt_tokens"].as_i64().unwrap_or(0);
                            let completion_tokens = usage["completion_tokens"].as_i64().unwrap_or(0);
                            let total_cost = providers::calculate_cost(provider, model, prompt_tokens, completion_tokens);
                            
                            update_call_log(&db_arc, call_log_id, prompt_tokens, completion_tokens, total_cost, "success", None);
                            update_key_usage(&db_arc, selected_key.id, total_cost);
                            
                            write.send(Message::Text(json!({
                                "type": "response",
                                "id": call_log_id,
                                "content": response_json["choices"][0]["message"]["content"].as_str().unwrap_or(""),
                                "usage": {
                                    "prompt_tokens": prompt_tokens,
                                    "completion_tokens": completion_tokens,
                                    "total_cost": total_cost
                                }
                            }).to_string())).await?;
                        } else {
                            let error_text = response.text().await?;
                            update_call_log(&db_arc, call_log_id, 0, 0, 0.0, "failed", Some(error_text.clone()));
                            write.send(Message::Text(json!({
                                "type": "error",
                                "message": error_text,
                                "id": call_log_id
                            }).to_string())).await?;
                        }
                    } else {
                        update_call_log(&db_arc, call_log_id, 0, 0, 0.0, "failed", Some("网络请求失败".to_string()));
                        write.send(Message::Text(json!({
                            "type": "error",
                            "message": "网络请求失败",
                            "id": call_log_id
                        }).to_string())).await?;
                    }
                }
            }
        }
    } else {
        let mut stream = stream;
        let mut buf = [0; 1024];
        
        let n = match stream.peek(&mut buf).await {
            Ok(n) => n,
            Err(_) => return Ok(()),
        };
        
        if n == 0 {
            return Ok(());
        }
        
        let request_str = String::from_utf8_lossy(&buf[..n]);
        
        let (status_line, body) = if request_str.starts_with("GET /health") {
            let body = json!({
                "status": "ok",
                "service": "gatemate",
                "version": "2.0.0"
            }).to_string();
            ("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: ".to_string() + &body.len().to_string() + "\r\n\r\n", body)
        } else if request_str.starts_with("GET / ") || request_str.starts_with("GET /\r") {
            let body = "<html><body><h1>GateMate HTTP Server</h1><p>WebSocket endpoint available at /</p></body></html>".to_string();
            ("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: ".to_string() + &body.len().to_string() + "\r\n\r\n", body)
        } else {
            let body = json!({
                "error": "Not Found"
            }).to_string();
            ("HTTP/1.1 404 Not Found\r\nContent-Type: application/json\r\nContent-Length: ".to_string() + &body.len().to_string() + "\r\n\r\n", body)
        };
        
        let response = status_line + &body;
        let _ = stream.write_all(response.as_bytes()).await;
        
        return Ok(());
    }
    
    Ok(())
}

async fn select_key(
    db_arc: &Arc<Mutex<Connection>>,
    project_id: i64,
    provider: &str,
    route_strategy: &Arc<Mutex<RouteStrategy>>,
    user_message: &str,
) -> Option<db::ApiKey> {
    let conn = db_arc.lock().unwrap();
    let keys = match db::get_keys_by_project_and_provider(&conn, project_id, provider) {
        Ok(k) => k,
        Err(e) => {
            eprintln!("⚠️ 获取密钥失败: {}", e);
            return None;
        }
    };
    
    if keys.is_empty() {
        return None;
    }
    
    let active_keys: Vec<_> = keys.into_iter().filter(|k| k.status == "active").collect();
    if active_keys.is_empty() {
        return None;
    }
    
    let strategy = route_strategy.lock().unwrap().strategy.clone();
    
    if strategy == "smart" {
        let routing_rules = match db::get_routing_rules(&conn, project_id) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("⚠️ 获取路由规则失败: {}", e);
                return None;
            }
        };
        
        for rule in routing_rules {
            if rule.is_enabled != 0 {
                let content = rule.match_content.to_lowercase();
                let message = user_message.to_lowercase();
                
                if message.contains(&content) || content.is_empty() {
                    for key in &active_keys {
                        if key.provider == rule.target_provider {
                            return Some(key.clone());
                        }
                    }
                }
            }
        }
    }
    
    match strategy.as_str() {
        "random" => {
            use rand::Rng;
            let len = active_keys.len();
            if len == 0 {
                None
            } else {
                let mut rng = rand::thread_rng();
                active_keys.get(rng.gen_range(0..len)).cloned()
            }
        }
        _ => {
            let mut strategy = route_strategy.lock().unwrap();
            let idx = strategy.round_robin_index % active_keys.len();
            strategy.round_robin_index += 1;
            active_keys.get(idx).cloned()
        }
    }
}

fn check_budget(db_arc: &Arc<Mutex<Connection>>, project_id: i64) -> String {
    let conn = match db_arc.lock() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("⚠️ 获取数据库连接失败: {}", e);
            return "".to_string();
        }
    };
    
    let project = match db::get_project(&conn, project_id) {
        Ok(p) => p,
        Err(_) => return "".to_string(),
    };
    
    if let Some(project) = project {
        if project.monthly_budget <= 0.0 {
            return "".to_string();
        }
        let monthly_usage = match db::get_project_monthly_usage(&conn, project_id) {
            Ok(u) => u,
            Err(_) => return "".to_string(),
        };
        if monthly_usage >= project.monthly_budget {
            "budget_exhausted".to_string()
        } else {
            "".to_string()
        }
    } else {
        "".to_string()
    }
}

fn get_provider_url(provider: &str, model: &str) -> String {
    providers::get_provider_url_with_model(provider, model)
}



fn insert_call_log(db_arc: &Arc<Mutex<Connection>>, project_id: i64, key_id: i64, provider: &str, remark: &str, model: &str) -> i64 {
    let conn = db_arc.lock().unwrap();
    match db::insert_call_log(&conn, project_id, key_id, provider, remark, model) {
        Ok(id) => id,
        Err(e) => {
            eprintln!("⚠️ 插入调用日志失败: {}", e);
            -1
        }
    }
}

fn update_call_log(db_arc: &Arc<Mutex<Connection>>, id: i64, prompt_tokens: i64, completion_tokens: i64, cost: f64, status: &str, error_message: Option<String>) {
    let conn = db_arc.lock().unwrap();
    if let Err(e) = db::update_call_log(&conn, id, prompt_tokens, completion_tokens, cost, status, error_message) {
        eprintln!("⚠️ 更新调用日志失败: {}", e);
    }
}

fn update_key_usage(db_arc: &Arc<Mutex<Connection>>, key_id: i64, cost: f64) {
    let conn = db_arc.lock().unwrap();
    if let Err(e) = db::add_key_daily_usage(&conn, key_id, cost) {
        eprintln!("⚠️ 更新密钥用量失败: {}", e);
    }
}

async fn send_request_with_retry(
    client: &Client,
    url: &str,
    headers: &reqwest::header::HeaderMap,
    body: &serde_json::Value,
    max_retries: usize,
    timeout_secs: u64,
) -> Option<reqwest::Response> {
    for attempt in 1..=max_retries {
        match tokio::time::timeout(tokio::time::Duration::from_secs(timeout_secs), client.post(url).headers(headers.clone()).json(body).send()).await {
            Ok(Ok(response)) => {
                if response.status().is_server_error() && attempt < max_retries {
                    eprintln!("⚠️ 请求失败 (状态码: {}), 正在重试 {}/{}", response.status(), attempt, max_retries);
                    tokio::time::sleep(tokio::time::Duration::from_secs(2u64.pow(attempt as u32))).await;
                    continue;
                }
                return Some(response);
            }
            Ok(Err(e)) => {
                eprintln!("⚠️ 请求失败, 正在重试 {}/{}: {}", attempt, max_retries, e);
                if attempt < max_retries {
                    tokio::time::sleep(tokio::time::Duration::from_secs(2u64.pow(attempt as u32))).await;
                }
            }
            Err(_) => {
                eprintln!("⚠️ 请求超时, 正在重试 {}/{}", attempt, max_retries);
                if attempt < max_retries {
                    tokio::time::sleep(tokio::time::Duration::from_secs(2u64.pow(attempt as u32))).await;
                }
            }
        }
    }
    
    None
}

