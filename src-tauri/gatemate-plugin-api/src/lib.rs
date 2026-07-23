use serde::{Deserialize, Serialize};
use std::os::raw::c_char;
use std::ffi::{CStr, CString};

#[derive(Debug, Serialize, Deserialize)]
pub struct PluginInfo {
    pub name: String,
    pub version: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RouteRequest {
    pub provider: String,
    pub model: String,
    pub project_id: i64,
    pub prompt_tokens: i64,
    pub completion_tokens: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RouteResponse {
    pub provider: String,
    pub model: String,
    pub api_key: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PdfExportRequest {
    pub project_id: i64,
    pub start_date: String,
    pub end_date: String,
    pub logs_json: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProStatus {
    pub is_pro: bool,
    pub expires_at: Option<String>,
}

pub trait Plugin: Send + Sync {
    fn get_info(&self) -> PluginInfo;
    fn get_pro_status(&self) -> ProStatus;
    fn smart_route(&self, request: RouteRequest) -> RouteResponse;
    fn export_pdf(&self, request: PdfExportRequest) -> Result<String, String>;
    fn verify_license(&self, license_key: &str) -> Result<ProStatus, String>;
}

#[no_mangle]
pub extern "C" fn gatemate_plugin_get_info() -> *const c_char {
    let info = PluginInfo {
        name: "gatemate-pro-plugin".to_string(),
        version: "1.0.0".to_string(),
        description: "GateMate Pro Plugin".to_string(),
    };
    let json = serde_json::to_string(&info).unwrap();
    string_to_c_char(json)
}

#[no_mangle]
pub extern "C" fn gatemate_plugin_get_pro_status() -> *const c_char {
    let status = ProStatus {
        is_pro: false,
        expires_at: None,
    };
    let json = serde_json::to_string(&status).unwrap();
    string_to_c_char(json)
}

#[no_mangle]
pub extern "C" fn gatemate_plugin_smart_route(request_json: *const c_char) -> *const c_char {
    let request = c_char_to_string(request_json);
    let request: RouteRequest = serde_json::from_str(&request).unwrap_or_default();
    
    let response = RouteResponse {
        provider: request.provider,
        model: request.model,
        api_key: "".to_string(),
    };
    
    let json = serde_json::to_string(&response).unwrap();
    string_to_c_char(json)
}

#[no_mangle]
pub extern "C" fn gatemate_plugin_export_pdf(request_json: *const c_char) -> *const c_char {
    let request = c_char_to_string(request_json);
    let _request: PdfExportRequest = serde_json::from_str(&request).unwrap_or_default();
    
    let result = serde_json::to_string(&serde_json::json!({
        "success": false,
        "message": "Not implemented"
    })).unwrap();
    string_to_c_char(result)
}

#[no_mangle]
pub extern "C" fn gatemate_plugin_verify_license(license_key: *const c_char) -> *const c_char {
    let _key = c_char_to_string(license_key);
    
    let status = ProStatus {
        is_pro: false,
        expires_at: None,
    };
    let json = serde_json::to_string(&status).unwrap();
    string_to_c_char(json)
}

#[no_mangle]
pub extern "C" fn gatemate_plugin_free_string(ptr: *const c_char) {
    unsafe {
        if !ptr.is_null() {
            let _ = CString::from_raw(ptr as *mut c_char);
        }
    }
}

fn string_to_c_char(s: String) -> *const c_char {
    let c_string = CString::new(s).unwrap();
    CString::into_raw(c_string)
}

fn c_char_to_string(ptr: *const c_char) -> String {
    if ptr.is_null() {
        return String::new();
    }
    unsafe {
        let c_str = CStr::from_ptr(ptr);
        c_str.to_string_lossy().into_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_plugin_info() {
        let info = PluginInfo {
            name: "test".to_string(),
            version: "1.0.0".to_string(),
            description: "Test plugin".to_string(),
        };
        let json = serde_json::to_string(&info).unwrap();
        assert!(json.contains("test"));
    }
    
    #[test]
    fn test_string_conversion() {
        let original = "hello world".to_string();
        let ptr = string_to_c_char(original.clone());
        let converted = c_char_to_string(ptr);
        assert_eq!(converted, original);
        unsafe {
            let _ = CString::from_raw(ptr as *mut c_char);
        }
    }
}