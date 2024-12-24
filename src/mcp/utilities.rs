use crate::mcp::types::*;
use crate::mcp::PROTOCOL_VERSION;
use crate::mcp::SERVER_NAME;
use crate::mcp::SERVER_VERSION;
use rpc_router::HandlerResult;
use serde_json::json;
use serde_json::Value;
use std::path::Path;

pub fn get_allowed_directories() -> Vec<String> {
    std::env::var("MCP_RS_FILESYSTEM_ALLOWED_DIRECTORIES")
        .unwrap_or_default()
        .split(':')
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect()
}

/// handler for `initialize` request from client
pub async fn initialize(_request: InitializeRequest) -> HandlerResult<InitializeResult> {
    let result = InitializeResult {
        protocol_version: PROTOCOL_VERSION.to_string(),
        server_info: Implementation {
            name: SERVER_NAME.to_string(),
            version: SERVER_VERSION.to_string(),
        },
        capabilities: ServerCapabilities {
            experimental: None,
            prompts: Some(PromptCapabilities::default()),
            resources: None,
            tools: Some(json!({})),
            roots: None,
            sampling: None,
            logging: None,
        },
        instructions: None,
    };
    Ok(result)
}

/// handler for SIGINT by client
pub fn graceful_shutdown() {
    // shutdown server
}

/// handler for `notifications/initialized` from client
pub fn notifications_initialized() {}

/// handler for `notifications/cancelled` from client
pub fn notifications_cancelled(_params: CancelledNotification) {
    // cancel request
}

pub async fn ping() -> HandlerResult<EmptyResult> {
    Ok(EmptyResult {})
}

pub async fn logging_set_level(_request: SetLevelRequest) -> HandlerResult<LoggingResponse> {
    Ok(LoggingResponse {})
}

pub async fn roots_list(_request: Option<ListRootsRequest>) -> HandlerResult<ListRootsResult> {
    let response = ListRootsResult {
        roots: vec![Root {
            name: "my project".to_string(),
            url: "file:///home/user/projects/my-project".to_string(),
        }],
    };
    Ok(response)
}

/// send notification to client
#[allow(dead_code)]
pub fn notify(method: &str, params: Option<Value>) {
    let notification = json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": params,
    });
    println!("{}", serde_json::to_string(&notification).unwrap());
}

pub fn is_path_allowed(path: &Path) -> bool {
    let allowed_dirs = get_allowed_directories();
    if allowed_dirs.is_empty() {
        return false; // If no directories are explicitly allowed, deny all access
    }

    // Get all parent directories of the path, including itself
    let mut check_path = path.to_path_buf();
    loop {
        // Try to canonicalize the current path if it exists
        let canonical_check = if check_path.exists() {
            match check_path.canonicalize() {
                Ok(p) => p,
                Err(_) => check_path.clone(),
            }
        } else {
            check_path.clone()
        };

        // Check if this path or parent is allowed
        for allowed_dir in &allowed_dirs {
            let allowed_path = Path::new(allowed_dir);
            let canonical_allowed = if allowed_path.exists() {
                match allowed_path.canonicalize() {
                    Ok(p) => p,
                    Err(_) => continue,
                }
            } else {
                continue;
            };

            if canonical_check.starts_with(&canonical_allowed) {
                return true;
            }
        }

        // Move up to parent directory
        match check_path.parent() {
            Some(parent) => check_path = parent.to_path_buf(),
            None => break,
        }
    }

    false
}

pub fn validate_path_or_error(path: &Path) -> Result<(), String> {
    if !is_path_allowed(path) {
        Err(format!(
            "Access denied: {} is not within allowed directories. Use the allowed_directories resource to view permitted locations.",
            path.display()
        ))
    } else {
        Ok(())
    }
}

// For operations that involve two paths (like move/rename)
pub fn validate_paths_or_error(source: &Path, target: &Path) -> Result<(), String> {
    if !is_path_allowed(source) {
        Err(format!(
            "Access denied: source path {} is not within allowed directories",
            source.display()
        ))
    } else if !is_path_allowed(target) {
        Err(format!(
            "Access denied: target path {} is not within allowed directories",
            target.display()
        ))
    } else {
        Ok(())
    }
}