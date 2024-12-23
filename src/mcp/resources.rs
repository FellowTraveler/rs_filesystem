use crate::mcp::types::*;
use rpc_router::HandlerResult;
use url::Url;
use std::path::PathBuf;
use std::fs;
use std::time::{SystemTime, Duration};
use serde_json::json;

pub async fn resources_list(
    _request: Option<ListResourcesRequest>,
) -> HandlerResult<ListResourcesResult> {
    let response = ListResourcesResult {
        resources: vec![
            Resource {
                uri: Url::parse("file:///logs/app.log").unwrap(),
                name: "Application Logs".to_string(),
                description: None,
                mime_type: Some("text/plain".to_string()),
            },
            Resource {
                uri: Url::parse("file:///api/list_directory").unwrap(),
                name: "List Directory".to_string(),
                description: Some("List contents of a directory".to_string()),
                mime_type: Some("application/json".to_string()),
            },
            Resource {
                uri: Url::parse("file:///api/get_file_info").unwrap(),
                name: "File Info".to_string(),
                description: Some("Get metadata about a file".to_string()),
                mime_type: Some("application/json".to_string()),
            }
        ],
        next_cursor: None,
    };
    Ok(response)
}

pub async fn resource_read(request: ReadResourceRequest) -> HandlerResult<ReadResourceResult> {
    let response = ReadResourceResult {
        content: ResourceContent {
            uri: request.uri.clone(),
            mime_type: Some("text/plain".to_string()),
            text: Some("2024-11-28T08:19:18.974368Z,INFO,main,this is message".to_string()),
            blob: None,
        },
    };
    Ok(response)
}

#[derive(Debug, Deserialize)]
pub struct ListDirectoryRequest {
    pub path: String,
}

pub async fn list_directory(request: ListDirectoryRequest) -> HandlerResult<ReadResourceResult> {
    let path = PathBuf::from(&request.path);
    
    if !path.exists() {
        return Err(JsonRpcError::invalid_params(format!(
            "Directory does not exist: {}", path.display()
        )));
    }
    
    if !path.is_dir() {
        return Err(JsonRpcError::invalid_params(format!(
            "Path is not a directory: {}", path.display()
        )));
    }

    let mut entries = Vec::new();
    match fs::read_dir(&path) {
        Ok(dir_entries) => {
            for entry in dir_entries {
                if let Ok(entry) = entry {
                    let metadata = entry.metadata().ok();
                    let file_type = metadata.as_ref().map(|m| m.file_type().is_file());
                    let size = metadata.as_ref().map(|m| m.len());
                    let modified = metadata.as_ref().and_then(|m| m.modified().ok());
                    
                    entries.push(json!({
                        "name": entry.file_name().to_string_lossy(),
                        "path": entry.path().to_string_lossy(),
                        "is_file": file_type.unwrap_or(false),
                        "size": size.unwrap_or(0),
                        "modified": modified.map(|t| t.duration_since(SystemTime::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs())
                    }));
                }
            }
        }
        Err(e) => return Err(JsonRpcError::internal_error(format!(
            "Failed to read directory: {}", e
        ))),
    }

    Ok(ReadResourceResult {
        content: ResourceContent {
            uri: Url::parse(&format!("file://{}", path.display())).unwrap(),
            mime_type: Some("application/json".to_string()),
            text: Some(serde_json::to_string_pretty(&entries).unwrap()),
            blob: None,
        },
    })
}

#[derive(Debug, Deserialize)]
pub struct GetFileInfoRequest {
    pub path: String,
}

pub async fn get_file_info(request: GetFileInfoRequest) -> HandlerResult<ReadResourceResult> {
    let path = PathBuf::from(&request.path);
    
    if !path.exists() {
        return Err(JsonRpcError::invalid_params(format!(
            "File does not exist: {}", path.display()
        )));
    }

    match path.metadata() {
        Ok(metadata) => {
            let info = json!({
                "path": path.to_string_lossy(),
                "is_file": metadata.is_file(),
                "is_dir": metadata.is_dir(),
                "size": metadata.len(),
                "modified": metadata.modified().ok().map(|t| 
                    t.duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs()),
                "created": metadata.created().ok().map(|t| 
                    t.duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs()),
                "readonly": metadata.permissions().readonly(),
            });

            Ok(ReadResourceResult {
                content: ResourceContent {
                    uri: Url::parse(&format!("file://{}", path.display())).unwrap(),
                    mime_type: Some("application/json".to_string()),
                    text: Some(serde_json::to_string_pretty(&info).unwrap()),
                    blob: None,
                },
            })
        }
        Err(e) => Err(JsonRpcError::internal_error(format!(
            "Failed to get file info: {}", e
        ))),
    }
}
