use crate::mcp::types::*;
use crate::mcp::PROTOCOL_VERSION;
use crate::mcp::SERVER_NAME;
use crate::mcp::SERVER_VERSION;
use rpc_router::HandlerResult;
use serde_json::json;
use serde_json::Value;

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
