use crate::mcp::types::*;
use rpc_router::HandlerResult;
use rpc_router::RpcParams;
use rpc_router::IntoHandlerError;
use url::Url;
use serde_json::json;
use serde::{Deserialize, Serialize};


pub async fn resources_list(
    _request: Option<ListResourcesRequest>,
) -> HandlerResult<ListResourcesResult> {
    let response = ListResourcesResult {
        resources: vec![
            Resource {
                uri: Url::parse("file:///api/allowed_directories").unwrap(),
                name: "Allowed Directories".to_string(),
                description: Some("List of directories that can be accessed".to_string()),
                mime_type: Some("application/json".to_string()),
            }
        ],
        next_cursor: None,
    };
    Ok(response)
}

pub async fn resource_read(request: ReadResourceRequest) -> HandlerResult<ReadResourceResult> {
    let response = match request.uri.path() {
        "/api/allowed_directories" => {
            let allowed_dirs = std::env::var("MCP_RS_FILESYSTEM_ALLOWED_DIRECTORIES")
                .unwrap_or_default()
                .split(':')
                .map(String::from)
                .collect::<Vec<_>>();

            ReadResourceResult {
                contents: vec![TextResourceContents {
                    uri: request.uri.clone(),
                    mime_type: Some("application/json".to_string()),
                    text: serde_json::to_string_pretty(&allowed_dirs).unwrap(),
                }],
            }
        },
        _ => return Err(json!({"code": -32602, "message": "Resource not found"}).into_handler_error()),
    };
    Ok(response)
}


#[derive(Debug, Deserialize, Serialize, RpcParams)]
pub struct GetAllowedDirectoriesRequest {
}

pub async fn allowed_directories(_request: GetAllowedDirectoriesRequest) -> HandlerResult<ReadResourceResult> {
    let allowed_dirs = std::env::var("MCP_RS_FILESYSTEM_ALLOWED_DIRECTORIES")
        .unwrap_or_default()
        .split(':')
        .map(String::from)
        .collect::<Vec<_>>();

    Ok(ReadResourceResult {
        contents: vec![TextResourceContents {
            uri: Url::parse("file:///api/allowed_directories").unwrap(),
            mime_type: Some("application/json".to_string()),
            text: serde_json::to_string_pretty(&allowed_dirs).unwrap(),
        }],
    })
}