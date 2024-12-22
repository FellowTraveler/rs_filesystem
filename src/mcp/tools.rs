use crate::mcp::types::*;
use maplit::hashmap;
use rpc_router::Handler;
use rpc_router::HandlerResult;
use rpc_router::RouterBuilder;
use rpc_router::RpcParams;
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::path::Path;
use git2::{Repository, Signature};

/// register all tools to the router
pub fn register_tools(router_builder: RouterBuilder) -> RouterBuilder {
    router_builder
        .append_dyn("tools/list", tools_list.into_dyn())
        .append_dyn("get_current_time_in_city", current_time.into_dyn())
        .append_dyn("get_local_time", get_local_time.into_dyn())
        .append_dyn("file_edit", file_edit.into_dyn())
        .append_dyn("read_file", read_file.into_dyn())
}

pub async fn tools_list(_request: Option<ListToolsRequest>) -> HandlerResult<ListToolsResult> {
    //let tools: Vec<Tool> = serde_json::from_str(include_str!("./templates/tools.json")).unwrap();
    let response = ListToolsResult {
        tools: vec![
            Tool {
                name: "get_current_time_in_city".to_string(),
                description: Some("Get the current time in the city".to_string()),
                input_schema: ToolInputSchema {
                    type_name: "object".to_string(),
                    properties: hashmap! {
                        "city".to_string() => ToolInputSchemaProperty {
                            type_name: Some("string".to_owned()),
                            description: Some("city name".to_owned()),
                            enum_values: None,
                        }
                    },
                    required: vec!["city".to_string()],
                },
            },
            Tool {
                name: "get_local_time".to_string(),
                description: Some("Get the current local time".to_string()),
                input_schema: ToolInputSchema {
                    type_name: "object".to_string(),
                    properties: hashmap!{},
                    required: vec![],
                },
            },
            Tool {
                name: "file_edit".to_string(),
                description: Some("Make a targeted edit to a file with optional git commit".to_string()),
                input_schema: ToolInputSchema {
                    type_name: "object".to_string(),
                    properties: hashmap! {
                        "file_path".to_string() => ToolInputSchemaProperty {
                            type_name: Some("string".to_owned()),
                            description: Some("Path to the file to edit".to_owned()),
                            enum_values: None,
                        },
                        "start_line".to_string() => ToolInputSchemaProperty {
                            type_name: Some("integer".to_owned()),
                            description: Some("Starting line number for the edit (0-based)".to_owned()),
                            enum_values: None,
                        },
                        "end_line".to_string() => ToolInputSchemaProperty {
                            type_name: Some("integer".to_owned()),
                            description: Some("Ending line number for the edit (0-based)".to_owned()),
                            enum_values: None,
                        },
                        "new_content".to_string() => ToolInputSchemaProperty {
                            type_name: Some("string".to_owned()),
                            description: Some("New content to insert".to_owned()),
                            enum_values: None,
                        },
                        "commit_message".to_string() => ToolInputSchemaProperty {
                            type_name: Some("string".to_owned()),
                            description: Some("Optional git commit message".to_owned()),
                            enum_values: None,
                        }
                    },
                    required: vec![
                        "file_path".to_string(),
                        "start_line".to_string(),
                        "end_line".to_string(),
                        "new_content".to_string()
                    ],
                },
            },
            Tool {
                name: "read_file".to_string(),
                description: Some("Read the contents of a file".to_string()),
                input_schema: ToolInputSchema {
                    type_name: "object".to_string(),
                    properties: hashmap! {
                        "file_path".to_string() => ToolInputSchemaProperty {
                            type_name: Some("string".to_owned()),
                            description: Some("Path to the file to read".to_owned()),
                            enum_values: None,
                        }
                    },
                    required: vec!["file_path".to_string()],
                },
            }
        ],
        next_cursor: None,
    };
    Ok(response)
}

#[derive(Deserialize, Serialize, RpcParams)]
pub struct CurrentTimeRequest {
    pub city: Option<String>,
}

pub async fn current_time(_request: CurrentTimeRequest) -> HandlerResult<CallToolResult> {
    let result = format!("Now: {}!", chrono::Local::now().to_rfc2822());
    Ok(CallToolResult {
        content: vec![CallToolResultContent::Text { text: result }],
        is_error: false,
    })
}

#[derive(Deserialize, Serialize, RpcParams)]
pub struct GetLocalTimeRequest {}

pub async fn get_local_time(_request: GetLocalTimeRequest) -> HandlerResult<CallToolResult> {
    let result = format!("Local time: {}", chrono::Local::now().to_rfc2822());
    Ok(CallToolResult {
        content: vec![CallToolResultContent::Text { text: result }],
        is_error: false,
    })
}

#[derive(Deserialize, Serialize, RpcParams)]
pub struct FileEditRequest {
    pub file_path: String,
    pub start_line: usize,
    pub end_line: usize,
    pub new_content: String,
    pub commit_message: Option<String>,
}

pub async fn file_edit(request: FileEditRequest) -> HandlerResult<CallToolResult> {
    // Read the file
    let path = Path::new(&request.file_path);
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => return Ok(CallToolResult {
            content: vec![CallToolResultContent::Text { 
                text: format!("Error reading file: {}", e) 
            }],
            is_error: true,
        }),
    };

    // Split into lines
    let mut lines: Vec<String> = content.lines().map(String::from).collect();

    // Validate line range
    if request.start_line > request.end_line || request.end_line >= lines.len() {
        return Ok(CallToolResult {
            content: vec![CallToolResultContent::Text { 
                text: format!("Invalid line range: {} to {}", request.start_line, request.end_line) 
            }],
            is_error: true,
        });
    }

    // Replace lines with new content
    let new_lines: Vec<String> = request.new_content.lines().map(String::from).collect();
    lines.splice(request.start_line..=request.end_line, new_lines);

    // Write back to file
    if let Err(e) = fs::write(path, lines.join("\n")) {
        return Ok(CallToolResult {
            content: vec![CallToolResultContent::Text { 
                text: format!("Error writing file: {}", e) 
            }],
            is_error: true,
        });
    }

    // Handle git commit if requested
    let mut message = String::from("File edited successfully");
    if let Some(commit_msg) = request.commit_message {
        if let Some(repo_path) = find_git_repo(path) {
            match commit_to_git(&repo_path, path, &commit_msg) {
                Ok(_) => message.push_str(". Changes committed to git"),
                Err(e) => message.push_str(&format!(". Git commit failed: {}", e)),
            }
        }
    }

    Ok(CallToolResult {
        content: vec![CallToolResultContent::Text { text: message }],
        is_error: false,
    })
}

#[derive(Deserialize, Serialize, RpcParams)]
pub struct ReadFileRequest {
    pub file_path: String,
}

pub async fn read_file(request: ReadFileRequest) -> HandlerResult<CallToolResult> {
    let path = Path::new(&request.file_path);
    match fs::read_to_string(path) {
        Ok(content) => Ok(CallToolResult {
            content: vec![CallToolResultContent::Text { text: content }],
            is_error: false,
        }),
        Err(e) => Ok(CallToolResult {
            content: vec![CallToolResultContent::Text { 
                text: format!("Error reading file: {}", e) 
            }],
            is_error: true,
        }),
    }
}

fn find_git_repo(path: &Path) -> Option<String> {
    let mut current = path.to_path_buf();
    while let Some(parent) = current.parent() {
        if parent.join(".git").exists() {
            return Some(parent.to_string_lossy().into_owned());
        }
        current = parent.to_path_buf();
    }
    None
}

fn commit_to_git(repo_path: &str, file_path: &Path, message: &str) -> Result<(), git2::Error> {
    let repo = Repository::open(repo_path)?;
    let mut index = repo.index()?;
    
    let relative_path = file_path.strip_prefix(repo_path)
        .unwrap_or(file_path)
        .to_string_lossy()
        .into_owned();
    
    index.add_path(Path::new(&relative_path))?;
    index.write()?;

    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    
    let signature = Signature::now("MCP Server", "mcp@example.com")?;
    let parent = repo.head()?.peel_to_commit()?;
    
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        message,
        &tree,
        &[&parent]
    )?;

    Ok(())
}
