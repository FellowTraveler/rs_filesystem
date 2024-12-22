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
                name: "analyze-code".to_string(),
                description: Some("Analyze code for potential improvements".to_string()),
                arguments: Some(vec![PromptArgument {
                    name: "language".to_string(),
                    description: Some("Programming language".to_string()),
                    required: Some(true),
                }]),
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
        _ => {
            return Err(json!({"code": -32602, "message": "Prompt not found"}).into_handler_error())
        }
    };
    Ok(response)
}
