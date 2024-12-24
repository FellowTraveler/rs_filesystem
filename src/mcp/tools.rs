use crate::mcp::types::*;
use maplit::hashmap;
use rpc_router::RouterBuilder;
use rpc_router::HandlerResult;
use rpc_router::Handler;
use rpc_router::RpcParams;
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::path::Path;
use git2::{Repository, Signature};
use crate::mcp::utilities::{validate_path_or_error, validate_paths_or_error, is_path_allowed};

/// register all tools to the router
pub fn register_tools(router_builder: RouterBuilder) -> RouterBuilder {
    router_builder
        .append_dyn("tools/list", tools_list.into_dyn())
        .append_dyn("get_current_time_in_city", current_time.into_dyn())
        .append_dyn("get_local_time", get_local_time.into_dyn())
        .append_dyn("file_edit", file_edit.into_dyn())
        .append_dyn("read_file", read_file.into_dyn())
        .append_dyn("list_directory", list_directory.into_dyn())
        .append_dyn("move_or_rename", move_or_rename.into_dyn())
        .append_dyn("get_file_info", get_file_info.into_dyn())
        .append_dyn("create_directory", create_directory.into_dyn())
        .append_dyn("overwrite_file", overwrite_file.into_dyn())
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
                            description: Some("Message describing the purpose of this edit".to_owned()),
                            enum_values: None,
                        }
                    },
                    required: vec![
                        "file_path".to_string(),
                        "start_line".to_string(),
                        "end_line".to_string(),
                        "new_content".to_string(),
                        "commit_message".to_string()
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
            },
            Tool {
                name: "list_directory".to_string(),
                description: Some("List contents of a directory".to_string()),
                input_schema: ToolInputSchema {
                    type_name: "object".to_string(),
                    properties: hashmap! {
                        "path".to_string() => ToolInputSchemaProperty {
                            type_name: Some("string".to_owned()),
                            description: Some("Path to directory to list".to_owned()),
                            enum_values: None,
                        }
                    },
                    required: vec!["path".to_string()],
                },
            },
            Tool {
                name: "move_or_rename".to_string(),
                description: Some("Move or rename a file or directory".to_string()),
                input_schema: ToolInputSchema {
                    type_name: "object".to_string(),
                    properties: hashmap! {
                        "source_path".to_string() => ToolInputSchemaProperty {
                            type_name: Some("string".to_owned()),
                            description: Some("Source path to move/rename from".to_owned()),
                            enum_values: None,
                        },
                        "target_path".to_string() => ToolInputSchemaProperty {
                            type_name: Some("string".to_owned()),
                            description: Some("Target path to move/rename to".to_owned()),
                            enum_values: None,
                        }
                    },
                    required: vec!["source_path".to_string(), "target_path".to_string()],
                },
            },
            Tool {
                name: "get_file_info".to_string(),
                description: Some("Get metadata about a file".to_string()),
                input_schema: ToolInputSchema {
                    type_name: "object".to_string(),
                    properties: hashmap! {
                        "path".to_string() => ToolInputSchemaProperty {
                            type_name: Some("string".to_owned()),
                            description: Some("Path to file to get info about".to_owned()),
                            enum_values: None,
                        }
                    },
                    required: vec!["path".to_string()],
                },
            },
            Tool {
                name: "create_directory".to_string(),
                description: Some("Create a new directory".to_string()),
                input_schema: ToolInputSchema {
                    type_name: "object".to_string(),
                    properties: hashmap! {
                        "path".to_string() => ToolInputSchemaProperty {
                            type_name: Some("string".to_owned()),
                            description: Some("Path to the new directory".to_owned()),
                            enum_values: None,
                        }
                    },
                    required: vec!["path".to_string()],
                },
            },
            Tool {
                name: "overwrite_file".to_string(),
                description: Some("Overwrite the contents of a file".to_string()),
                input_schema: ToolInputSchema {
                    type_name: "object".to_string(),
                    properties: hashmap! {
                        "path".to_string() => ToolInputSchemaProperty {
                            type_name: Some("string".to_owned()),
                            description: Some("Path to the file to overwrite".to_owned()),
                            enum_values: None,
                        },
                        "content".to_string() => ToolInputSchemaProperty {
                            type_name: Some("string".to_owned()),
                            description: Some("New content to write".to_owned()),
                            enum_values: None,
                        }
                    },
                    required: vec!["path".to_string(), "content".to_string()],
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
    pub commit_message: String,
}


pub async fn file_edit(request: FileEditRequest) -> HandlerResult<CallToolResult> {
    // Validate path is within allowed directories
    let path = Path::new(&request.file_path);
    if let Err(msg) = validate_path_or_error(path) {
        return Ok(CallToolResult {
            content: vec![CallToolResultContent::Text { text: msg }],
            is_error: true,
        });
    }

    // Read the file
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
    if let Some(repo_path) = find_git_repo(path) {
        match commit_to_git(&repo_path, path, &request.commit_message) {
            Ok(_) => message.push_str(". Changes committed to git"),
            Err(e) => message.push_str(&format!(". Git commit failed: {}", e)),
        }
    }

    Ok(CallToolResult {
        content: vec![CallToolResultContent::Text { text: message }],
        is_error: false,
    })
}

#[derive(Deserialize, Serialize, RpcParams)]
pub struct CreateDirectoryRequest {
    pub path: String,
}

pub async fn create_directory(request: CreateDirectoryRequest) -> HandlerResult<CallToolResult> {
    let path = Path::new(&request.path);
    if let Err(msg) = validate_path_or_error(path) {
        return Ok(CallToolResult {
            content: vec![CallToolResultContent::Text { text: msg }],
            is_error: true,
        });
    }

    match std::fs::create_dir_all(&path) {
        Ok(_) => Ok(CallToolResult {
            content: vec![CallToolResultContent::Text { 
                text: format!("Directory created successfully: {}", path.display()) 
            }],
            is_error: false,
        }),
        Err(e) => Ok(CallToolResult {
            content: vec![CallToolResultContent::Text { 
                text: format!("Failed to create directory: {}", e) 
            }],
            is_error: true,
        }),
    }
}

#[derive(Deserialize, Serialize, RpcParams)]
pub struct OverwriteFileRequest {
    pub path: String,
    pub content: String,
}

pub async fn overwrite_file(request: OverwriteFileRequest) -> HandlerResult<CallToolResult> {
    let path = Path::new(&request.path);
    if let Err(msg) = validate_path_or_error(path) {
        return Ok(CallToolResult {
            content: vec![CallToolResultContent::Text { text: msg }],
            is_error: true,
        });
    }

    match std::fs::write(path, &request.content) {
        Ok(_) => Ok(CallToolResult {
            content: vec![CallToolResultContent::Text { 
                text: format!("File written successfully: {}", path.display()) 
            }],
            is_error: false,
        }),
        Err(e) => Ok(CallToolResult {
            content: vec![CallToolResultContent::Text { 
                text: format!("Failed to write file: {}", e) 
            }],
            is_error: true,
        }),
    }
}

#[derive(Deserialize, Serialize, RpcParams)]
pub struct ReadFileRequest {
    pub file_path: String,
}

pub async fn read_file(request: ReadFileRequest) -> HandlerResult<CallToolResult> {
    let path = Path::new(&request.file_path);
    if let Err(msg) = validate_path_or_error(path) {
        return Ok(CallToolResult {
            content: vec![CallToolResultContent::Text { text: msg }],
            is_error: true,
        });
    }

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

#[derive(Deserialize, Serialize, RpcParams)]
pub struct ListDirectoryRequest {
    pub path: String,
}

pub async fn list_directory(request: ListDirectoryRequest) -> HandlerResult<CallToolResult> {
    let path = Path::new(&request.path);
    if let Err(msg) = validate_path_or_error(path) {
        return Ok(CallToolResult {
            content: vec![CallToolResultContent::Text { text: msg }],
            is_error: true,
        });
    }

    match fs::read_dir(path) {
        Ok(dir) => {
            let mut content = String::new();
            for entry in dir {
                if let Ok(entry) = entry {
                    // Also validate each entry is within allowed directories
                    if is_path_allowed(&entry.path()) {
                        content.push_str(&format!("{}\n", entry.file_name().to_string_lossy()));
                    }
                }
            }
            Ok(CallToolResult {
                content: vec![CallToolResultContent::Text { text: content }],
                is_error: false,
            })
        },
        Err(e) => Ok(CallToolResult {
            content: vec![CallToolResultContent::Text { 
                text: format!("Error listing directory: {}", e) 
            }],
            is_error: true,
        }),
    }
}

#[derive(Deserialize, Serialize, RpcParams)]
pub struct MoveOrRenameRequest {
    pub source_path: String,
    pub target_path: String,
}

pub async fn move_or_rename(request: MoveOrRenameRequest) -> HandlerResult<CallToolResult> {
    let source_path = Path::new(&request.source_path);
    let target_path = Path::new(&request.target_path);
    
    if let Err(msg) = validate_paths_or_error(source_path, target_path) {
        return Ok(CallToolResult {
            content: vec![CallToolResultContent::Text { text: msg }],
            is_error: true,
        });
    }

    match fs::rename(source_path, target_path) {
        Ok(_) => Ok(CallToolResult {
            content: vec![CallToolResultContent::Text { 
                text: format!("Moved or renamed successfully: {} to {}", source_path.display(), target_path.display()) 
            }],
            is_error: false,
        }),
        Err(e) => Ok(CallToolResult {
            content: vec![CallToolResultContent::Text { 
                text: format!("Failed to move or rename: {}", e) 
            }],
            is_error: true,
        }),
    }
}

#[derive(Deserialize, Serialize, RpcParams)]
pub struct GetFileInfoRequest {
    pub path: String,
}

pub async fn get_file_info(request: GetFileInfoRequest) -> HandlerResult<CallToolResult> {
    let path = Path::new(&request.path);
    if let Err(msg) = validate_path_or_error(path) {
        return Ok(CallToolResult {
            content: vec![CallToolResultContent::Text { text: msg }],
            is_error: true,
        });
    }

    match fs::metadata(path) {
        Ok(metadata) => {
            let mut content = String::new();
            content.push_str(&format!("File size: {}\n", metadata.len()));
            content.push_str(&format!("File type: {:?}\n", metadata.file_type()));
            if let Ok(modified) = metadata.modified() {
                content.push_str(&format!("Last modified: {:?}\n", modified));
            }
            Ok(CallToolResult {
                content: vec![CallToolResultContent::Text { text: content }],
                is_error: false,
            })
        },
        Err(e) => Ok(CallToolResult {
            content: vec![CallToolResultContent::Text { 
                text: format!("Error getting file info: {}", e) 
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    use tempfile::TempDir;
    use std::env;

    fn setup_test_env() -> (TempDir, String) {
        let _temp_dir = TempDir::new().unwrap();
        let canonical_path = _temp_dir.path().canonicalize().unwrap();
        let _temp_path = canonical_path.to_str().unwrap().to_string();
        
        // On macOS, we need to allow both the /var and /private/var paths
        #[cfg(target_os = "macos")]
        {
            // Always allow both /var and /private/var paths if applicable
            let (var_path, private_var_path) = if _temp_path.starts_with("/private/var") {
                (_temp_path.strip_prefix("/private").unwrap().to_string(), _temp_path.clone())
            } else if _temp_path.starts_with("/var") {
                (_temp_path.clone(), format!("/private{}", _temp_path))
            } else {
                (_temp_path.clone(), _temp_path.clone())
            };
            
            // Set both paths in the environment variable
            env::set_var(
                "MCP_RS_FILESYSTEM_ALLOWED_DIRECTORIES",
                if var_path != private_var_path {
                    format!("{}:{}", var_path, private_var_path)
                } else {
                    var_path
                }
            );
        }
    
        #[cfg(not(target_os = "macos"))]
        {
            env::set_var(
                "MCP_RS_FILESYSTEM_ALLOWED_DIRECTORIES",
                &_temp_path
            );
        }
        
        (_temp_dir, _temp_path)
    }
    
    fn setup_git_repo() -> (TempDir, String) {
        let (_temp_dir, _temp_path) = setup_test_env();
        
        // Initialize git repo using the temp directory path
        let repo = git2::Repository::init(_temp_dir.path()).unwrap();
        
        // Create test file in the temp directory
        let test_file_path = _temp_dir.path().join("test.txt");
        fs::write(&test_file_path, "initial content\n").unwrap();
        
        // Make initial commit
        let mut index = repo.index().unwrap();
        index.add_path(Path::new("test.txt")).unwrap();
        index.write().unwrap();
        let tree_id = index.write_tree().unwrap();
        let tree = repo.find_tree(tree_id).unwrap();
        let signature = git2::Signature::now("Test User", "test@example.com").unwrap();
        repo.commit(Some("HEAD"), &signature, &signature, "Initial commit", &tree, &[]).unwrap();
        
        // Canonicalize the file path after creating it
        let canonical_file_path = test_file_path.canonicalize().unwrap();
        (_temp_dir, canonical_file_path.to_str().unwrap().to_string())
    }

    #[tokio::test]
    async fn test_file_edit_with_git() {
        let (_temp_dir, file_path) = setup_git_repo();
        
        let request = FileEditRequest {
            file_path: file_path.clone(),
            start_line: 0,
            end_line: 0,
            new_content: "modified content".to_string(),
            commit_message: "test commit".to_string(),
        };

        let result = file_edit(request).await.unwrap();
        assert!(!result.is_error, "file_edit failed: {:?}", result.content);
        
        // Verify file was modified
        let content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "modified content");
        
        // Verify git commit happened
        let repo = git2::Repository::open(_temp_dir.path()).unwrap();
        let head_commit = repo.head().unwrap().peel_to_commit().unwrap();
        assert_eq!(head_commit.message().unwrap(), "test commit");
    }

    #[tokio::test]
    async fn test_file_edit_without_git() {
        let (_temp_dir, _temp_path) = setup_test_env();
        
        // Create test file in the temp directory
        let test_file_path = Path::new(&_temp_path).join("test.txt");
        fs::write(&test_file_path, "initial content\n").unwrap();
        
        // Canonicalize the file path after creating it
        let canonical_file_path = test_file_path.canonicalize().unwrap();
        
        // Edit the file
        let request = FileEditRequest {
            file_path: canonical_file_path.to_str().unwrap().to_string(),
            start_line: 0,
            end_line: 0,
            new_content: "modified content".to_string(),
            commit_message: "".to_string(),
        };
        
        let result = file_edit(request).await.unwrap();
        assert!(!result.is_error);
        
        // Verify file content
        let content = fs::read_to_string(&canonical_file_path).unwrap();
        assert_eq!(content, "modified content");
    }
}
