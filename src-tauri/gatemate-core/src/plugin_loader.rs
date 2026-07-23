use std::os::raw::c_char;
use std::sync::Mutex;
use std::ffi::{CStr, CString};
use libloading::{Library, Symbol};
use serde_json::json;

lazy_static::lazy_static! {
    static ref PLUGIN_LIBRARY: Mutex<Option<Library>> = Mutex::new(None);
    static ref IS_LOADED: Mutex<bool> = Mutex::new(false);
}

type GetProStatusFn = unsafe extern "C" fn() -> *const c_char;
type VerifyLicenseFn = unsafe extern "C" fn(*const c_char) -> *const c_char;
type FreeStringFn = unsafe extern "C" fn(*const c_char);
type SmartRouteFn = unsafe extern "C" fn(*const c_char) -> *const c_char;
type ExportPdfFn = unsafe extern "C" fn(*const c_char) -> *const c_char;

pub fn load_plugin() -> bool {
    let plugin_path = "./gatemate_plugin.dll";
    
    if !std::path::Path::new(plugin_path).exists() {
        return false;
    }
    
    match unsafe { Library::new(plugin_path) } {
        Ok(library) => {
            *PLUGIN_LIBRARY.lock().unwrap() = Some(library);
            *IS_LOADED.lock().unwrap() = true;
            true
        }
        Err(_) => {
            *IS_LOADED.lock().unwrap() = false;
            false
        }
    }
}

pub fn is_loaded() -> bool {
    *IS_LOADED.lock().unwrap()
}

fn cstr_to_string(ptr: *const c_char) -> String {
    if ptr.is_null() {
        return String::new();
    }
    unsafe {
        let c_str = CStr::from_ptr(ptr);
        c_str.to_string_lossy().into_owned()
    }
}

pub fn get_pro_status() -> serde_json::Value {
    if !is_loaded() {
        return json!({ "is_pro": false, "expires_at": null });
    }
    
    let library = PLUGIN_LIBRARY.lock().unwrap();
    if let Some(ref lib) = *library {
        unsafe {
            let func: Symbol<GetProStatusFn> = match lib.get(b"gatemate_plugin_get_pro_status") {
                Ok(f) => f,
                Err(_) => return json!({ "is_pro": false, "expires_at": null }),
            };
            
            let result_ptr = func();
            let result = cstr_to_string(result_ptr);
            
            let free_func: Symbol<FreeStringFn> = match lib.get(b"gatemate_plugin_free_string") {
                Ok(f) => f,
                Err(_) => return json!({ "is_pro": false, "expires_at": null }),
            };
            free_func(result_ptr);
            
            match serde_json::from_str(&result) {
                Ok(v) => v,
                Err(_) => json!({ "is_pro": false, "expires_at": null }),
            }
        }
    } else {
        json!({ "is_pro": false, "expires_at": null })
    }
}

pub fn smart_route(request: &serde_json::Value) -> serde_json::Value {
    if !is_loaded() {
        return json!({
            "provider": request["provider"].as_str().unwrap_or(""),
            "model": request["model"].as_str().unwrap_or(""),
            "api_key": ""
        });
    }
    
    let library = PLUGIN_LIBRARY.lock().unwrap();
    if let Some(ref lib) = *library {
        unsafe {
            let request_json = serde_json::to_string(request).unwrap();
            let request_cstr = CString::new(request_json).unwrap();
            
            let func: Symbol<SmartRouteFn> = match lib.get(b"gatemate_plugin_smart_route") {
                Ok(f) => f,
                Err(_) => {
                    return json!({
                        "provider": request["provider"].as_str().unwrap_or(""),
                        "model": request["model"].as_str().unwrap_or(""),
                        "api_key": ""
                    });
                }
            };
            
            let result_ptr = func(request_cstr.as_ptr());
            let result = cstr_to_string(result_ptr);
            
            let free_func: Symbol<FreeStringFn> = lib.get(b"gatemate_plugin_free_string").unwrap();
            free_func(result_ptr);
            
            match serde_json::from_str(&result) {
                Ok(v) => v,
                Err(_) => json!({
                    "provider": request["provider"].as_str().unwrap_or(""),
                    "model": request["model"].as_str().unwrap_or(""),
                    "api_key": ""
                })
            }
        }
    } else {
        json!({
            "provider": request["provider"].as_str().unwrap_or(""),
            "model": request["model"].as_str().unwrap_or(""),
            "api_key": ""
        })
    }
}

pub fn export_pdf(request: &serde_json::Value) -> serde_json::Value {
    if !is_loaded() {
        return json!({ "success": false, "message": "Plugin not loaded" });
    }
    
    let library = PLUGIN_LIBRARY.lock().unwrap();
    if let Some(ref lib) = *library {
        unsafe {
            let request_json = serde_json::to_string(request).unwrap();
            let request_cstr = CString::new(request_json).unwrap();
            
            let func: Symbol<ExportPdfFn> = match lib.get(b"gatemate_plugin_export_pdf") {
                Ok(f) => f,
                Err(_) => {
                    return json!({ "success": false, "message": "Function not found" });
                }
            };
            
            let result_ptr = func(request_cstr.as_ptr());
            let result = cstr_to_string(result_ptr);
            
            let free_func: Symbol<FreeStringFn> = lib.get(b"gatemate_plugin_free_string").unwrap();
            free_func(result_ptr);
            
            match serde_json::from_str(&result) {
                Ok(v) => v,
                Err(_) => json!({ "success": false, "message": "Invalid response" })
            }
        }
    } else {
        json!({ "success": false, "message": "Plugin not loaded" })
    }
}

pub fn verify_license(license_key: &str) -> serde_json::Value {
    if !is_loaded() {
        return json!({ "is_pro": false, "expires_at": null });
    }
    
    let library = PLUGIN_LIBRARY.lock().unwrap();
    if let Some(ref lib) = *library {
        unsafe {
            let key_cstr = CString::new(license_key).unwrap();
            
            let func: Symbol<VerifyLicenseFn> = match lib.get(b"gatemate_plugin_verify_license") {
                Ok(f) => f,
                Err(_) => {
                    return json!({ "is_pro": false, "expires_at": null });
                }
            };
            
            let result_ptr = func(key_cstr.as_ptr());
            let result = cstr_to_string(result_ptr);
            
            let free_func: Symbol<FreeStringFn> = lib.get(b"gatemate_plugin_free_string").unwrap();
            free_func(result_ptr);
            
            match serde_json::from_str(&result) {
                Ok(v) => v,
                Err(_) => json!({ "is_pro": false, "expires_at": null })
            }
        }
    } else {
        json!({ "is_pro": false, "expires_at": null })
    }
}