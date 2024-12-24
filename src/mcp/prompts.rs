use crate::mcp::types::*;
use rpc_router::HandlerResult;
use rpc_router::IntoHandlerError;
use serde_json::json;

pub async fn prompts_list(
    _request: Option<ListPromptsRequest>,
) -> HandlerResult<ListPromptsResult> {
    //let prompts: Vec<Prompt> = serde_json::from_str(include_str!("./templates/prompts.json")).unwrap();
    let response = ListPromptsResult {
        next_cursor: None,
        prompts: vec![
            Prompt {
                name: "current_time".to_string(),
                description: Some("Display current time in the city".to_string()),
                arguments: Some(vec![PromptArgument {
                    name: "city".to_string(),
                    description: Some("city name".to_string()),
                    required: Some(true),
                }]),
            },
            Prompt {
                name: "get_local_time".to_string(),
                description: Some("Get the current local time".to_string()),
                arguments: None,
            },
            Prompt {
                name: "file_edit".to_string(),
                description: Some("Make a targeted edit to a file".to_string()),
                arguments: Some(vec![
                    PromptArgument {
                        name: "file_path".to_string(),
                        description: Some("Path to file to edit".to_string()),
                        required: Some(true),
                    },
                    PromptArgument {
                        name: "commit_message".to_string(),
                        description: Some("Message describing the edit".to_string()),
                        required: Some(true),
                    },
                ]),
            },
            Prompt {
                name: "read_file".to_string(),
                description: Some("Read contents of a file".to_string()),
                arguments: Some(vec![PromptArgument {
                    name: "file_path".to_string(),
                    description: Some("Path to file to read".to_string()),
                    required: Some(true),
                }]),
            },
            Prompt {
                name: "list_directory".to_string(),
                description: Some("List contents of a directory".to_string()),
                arguments: Some(vec![PromptArgument {
                    name: "path".to_string(),
                    description: Some("Path to directory to list".to_string()),
                    required: Some(true),
                }]),
            },
            Prompt {
                name: "move_or_rename".to_string(),
                description: Some("Move or rename a file or directory".to_string()),
                arguments: Some(vec![
                    PromptArgument {
                        name: "source_path".to_string(),
                        description: Some("Source path to move/rename from".to_string()),
                        required: Some(true),
                    },
                    PromptArgument {
                        name: "target_path".to_string(),
                        description: Some("Target path to move/rename to".to_string()),
                        required: Some(true),
                    },
                ]),
            },
            Prompt {
                name: "get_file_info".to_string(),
                description: Some("Get metadata about a file".to_string()),
                arguments: Some(vec![PromptArgument {
                    name: "path".to_string(),
                    description: Some("Path to file to get info about".to_string()),
                    required: Some(true),
                }]),
            },
            Prompt {
                name: "create_directory".to_string(),
                description: Some("Create a new directory".to_string()),
                arguments: Some(vec![PromptArgument {
                    name: "path".to_string(),
                    description: Some("Path to directory to create".to_string()),
                    required: Some(true),
                }]),
            },
            Prompt {
                name: "overwrite_file".to_string(),
                description: Some("Overwrite contents of a file".to_string()),
                arguments: Some(vec![
                    PromptArgument {
                        name: "file_path".to_string(),
                        description: Some("Path to file to overwrite".to_string()),
                        required: Some(true),
                    },
                    PromptArgument {
                        name: "content".to_string(),
                        description: Some("New content for the file".to_string()),
                        required: Some(true),
                    },
                ]),
            },
        ],
    };
    Ok(response)
}

pub async fn prompts_get(request: GetPromptRequest) -> HandlerResult<PromptResult> {
    let response = match request.name.as_str() {
        "current_time" => PromptResult {
            description: "Get the current time in city".to_string(),
            messages: Some(vec![PromptMessage {
                role: "user".to_string(),
                content: PromptMessageContent {
                    type_name: "text".to_string(),
                    text: format!(
                        "What's the time of {}?",
                        request.arguments.unwrap()["city"].as_str().unwrap()
                    ),
                },
            }]),
        },
        "get_local_time" => PromptResult {
            description: "Get the current local time".to_string(),
            messages: Some(vec![PromptMessage {
                role: "user".to_string(),
                content: PromptMessageContent {
                    type_name: "text".to_string(),
                    text: "What's the current local time?".to_string(),
                },
            }]),
        },
        "file_edit" => PromptResult {
            description: "Edit a file".to_string(),
            messages: Some(vec![PromptMessage {
                role: "user".to_string(),
                content: PromptMessageContent {
                    type_name: "text".to_string(),
                    text: format!(
                        "Edit file {} with message: {}",
                        request.arguments.unwrap()["file_path"].as_str().unwrap(),
                        request.arguments.unwrap()["commit_message"].as_str().unwrap()
                    ),
                },
            }]),
        },
        "read_file" => PromptResult {
            description: "Read a file".to_string(),
            messages: Some(vec![PromptMessage {
                role: "user".to_string(),
                content: PromptMessageContent {
                    type_name: "text".to_string(),
                    text: format!(
                        "Read file {}",
                        request.arguments.unwrap()["file_path"].as_str().unwrap()
                    ),
                },
            }]),
        },
        "list_directory" => PromptResult {
            description: "List directory contents".to_string(),
            messages: Some(vec![PromptMessage {
                role: "user".to_string(),
                content: PromptMessageContent {
                    type_name: "text".to_string(),
                    text: format!(
                        "List contents of directory {}",
                        request.arguments.unwrap()["path"].as_str().unwrap()
                    ),
                },
            }]),
        },
        "move_or_rename" => PromptResult {
            description: "Move or rename file/directory".to_string(),
            messages: Some(vec![PromptMessage {
                role: "user".to_string(),
                content: PromptMessageContent {
                    type_name: "text".to_string(),
                    text: format!(
                        "Move/rename {} to {}",
                        request.arguments.unwrap()["source_path"].as_str().unwrap(),
                        request.arguments.unwrap()["target_path"].as_str().unwrap()
                    ),
                },
            }]),
        },
        "get_file_info" => PromptResult {
            description: "Get file metadata".to_string(),
            messages: Some(vec![PromptMessage {
                role: "user".to_string(),
                content: PromptMessageContent {
                    type_name: "text".to_string(),
                    text: format!(
                        "Get info about file {}",
                        request.arguments.unwrap()["path"].as_str().unwrap()
                    ),
                },
            }]),
        },
        "create_directory" => PromptResult {
            description: "Create directory".to_string(),
            messages: Some(vec![PromptMessage {
                role: "user".to_string(),
                content: PromptMessageContent {
                    type_name: "text".to_string(),
                    text: format!(
                        "Create directory {}",
                        request.arguments.unwrap()["path"].as_str().unwrap()
                    ),
                },
            }]),
        },
        "overwrite_file" => PromptResult {
            description: "Overwrite file".to_string(),
            messages: Some(vec![PromptMessage {
                role: "user".to_string(),
                content: PromptMessageContent {
                    type_name: "text".to_string(),
                    text: format!(
                        "Overwrite file {} with new content",
                        request.arguments.unwrap()["file_path"].as_str().unwrap()
                    ),
                },
            }]),
        },
        _ => return Err("Unknown prompt".into()),
    };
    Ok(response)
}
