mod mcp;

use crate::mcp::prompts::prompts_get;
use crate::mcp::prompts::prompts_list;
use crate::mcp::resources::resource_read;
use crate::mcp::resources::resources_list;
use crate::mcp::resources::{allowed_directories};
use crate::mcp::tools::register_tools;
use crate::mcp::types::CancelledNotification;
use crate::mcp::types::JsonRpcError;
use crate::mcp::types::JsonRpcResponse;
use crate::mcp::types::ToolCallRequestParams;
use crate::mcp::utilities::*;
use clap::Parser;
use dirs::data_local_dir;
use dirs::home_dir;
use dirs::state_dir;
use rpc_router::Error;
use rpc_router::Handler;
use rpc_router::Request;
use rpc_router::Router;
use rpc_router::RouterBuilder;
use serde_json::json;
use serde_json::Value;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use tokio::io::AsyncBufReadExt;
use tokio::signal;

fn build_rpc_router() -> Router {
    let builder = RouterBuilder::default()
        // append resources here
        .append_dyn("initialize", initialize.into_dyn())
        .append_dyn("ping", ping.into_dyn())
        .append_dyn("logging/setLevel", logging_set_level.into_dyn())
        .append_dyn("roots/list", roots_list.into_dyn())
        .append_dyn("prompts/list", prompts_list.into_dyn())
        .append_dyn("prompts/get", prompts_get.into_dyn())
        .append_dyn("resources/list", resources_list.into_dyn())
        .append_dyn("resources/read", resource_read.into_dyn())
        .append_dyn("resources/allowed_directories", allowed_directories.into_dyn());
    let builder = register_tools(builder);
    builder.build()
}

fn get_log_directory() -> PathBuf {
    if cfg!(target_os = "macos") {
        // macOS: ~/Library/Logs/Claude
        home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("Library/Logs/Claude")
    } else if cfg!(target_os = "windows") {
        // Windows: %LOCALAPPDATA%\Claude\logs
        data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("Claude")
            .join("logs")
    } else {
        // Linux: ~/.local/state/claude/logs
        state_dir()
            .unwrap_or_else(|| {
                home_dir()
                    .unwrap_or_else(|| PathBuf::from("."))
                    .join(".local/state")
            })
            .join("claude")
            .join("logs")
    }
}

#[tokio::main]
async fn main() {
    // Parse command-line arguments
    let args = Args::parse();
    if !args.mcp {
        display_info(&args);
        return;
    }

    // Clone necessary variables for the shutdown task
    let shutdown_handle = tokio::spawn(async {
        // Create a shutdown signal future
        #[cfg(unix)]
        let shutdown = async {
            // Listen for SIGINT and SIGTERM on Unix
            let mut sigint = signal::unix::signal(signal::unix::SignalKind::interrupt())
                .expect("Failed to set up SIGINT handler");
            let mut sigterm = signal::unix::signal(signal::unix::SignalKind::terminate())
                .expect("Failed to set up SIGTERM handler");

            tokio::select! {
                _ = sigint.recv() => {},
                _ = sigterm.recv() => {},
            }
        };

        #[cfg(windows)]
        let shutdown = async {
            // Listen for Ctrl+C on Windows
            signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
        };

        shutdown.await;
        graceful_shutdown();
        std::process::exit(0);
    });

    // Process JSON-RPC from MCP client
    let router = build_rpc_router();
    let log_path = env::var("MCP_LOG_FILE_PATH").map(PathBuf::from).unwrap_or_else(|_| {
        get_log_directory().join("rs_filesystem.logs.jsonl")
    });

    let mut logging_file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(&log_path)
        .unwrap();

    // Spawn a task to read lines from stdin
    let rpc_handle = tokio::spawn(async move {
        let mut reader = tokio::io::BufReader::new(tokio::io::stdin()).lines();

        while let Ok(Some(line)) = reader.next_line().await {
            writeln!(logging_file, "{}", line).unwrap();
            if !line.is_empty() {
                if let Ok(json_value) = serde_json::from_str::<Value>(&line) {
                    // Notifications, no response required
                    if json_value.is_object() && json_value.get("id").is_none() {
                        if let Some(method) = json_value.get("method") {
                            if method == "notifications/initialized" {
                                notifications_initialized();
                            } else if method == "notifications/cancelled" {
                                let params_value = json_value.get("params").unwrap();
                                let cancel_params: CancelledNotification =
                                    serde_json::from_value(params_value.clone()).unwrap();
                                notifications_cancelled(cancel_params);
                            }
                        }
                    } else if let Ok(mut rpc_request) = Request::from_value(json_value) {
                        // Normal JSON-RPC message, and response expected
                        let id = rpc_request.id.clone();
                        if rpc_request.method == "tools/call" {
                            let params = serde_json::from_value::<ToolCallRequestParams>(
                                rpc_request.params.unwrap(),
                            )
                            .unwrap();
                            rpc_request = Request {
                                id: id.clone(),
                                method: params.name,
                                params: params.arguments,
                            }
                        }
                        match router.call(rpc_request).await {
                            Ok(call_response) => {
                                if !call_response.value.is_null() {
                                    let response =
                                        JsonRpcResponse::new(id, call_response.value.clone());
                                    let response_json = serde_json::to_string(&response).unwrap();
                                    writeln!(logging_file, "{}\n", response_json).unwrap();
                                    println!("{}", response_json);
                                }
                            }
                            Err(error) => match &error.error {
                                // Error from JSON-RPC call
                                Error::Handler(handler) => {
                                    if let Some(error_value) = handler.get::<Value>() {
                                        let json_error = json!({
                                            "jsonrpc": "2.0",
                                            "error": error_value,
                                            "id": id
                                        });
                                        let response = serde_json::to_string(&json_error).unwrap();
                                        writeln!(logging_file, "{}\n", response).unwrap();
                                        println!("{}", response);
                                    }
                                }
                                _ => {
                                    let json_error = JsonRpcError::new(
                                        id,
                                        -1,
                                        format!(
                                            "Invalid json-rpc call, error: {}",
                                            error.error.to_string()
                                        )
                                        .as_str(),
                                    );
                                    let response = serde_json::to_string(&json_error).unwrap();
                                    writeln!(logging_file, "{}\n", response).unwrap();
                                    println!("{}", response);
                                }
                            },
                        }
                    }
                }
            }
        }
    });

    // Wait for either the RPC handling or shutdown to complete
    tokio::select! {
        _ = rpc_handle => {},
        _ = shutdown_handle => {},
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// List resources
    #[arg(long, default_value = "false")]
    resources: bool,
    /// List prompts
    #[arg(long, default_value = "false")]
    prompts: bool,
    /// List tools
    #[arg(long, default_value = "false")]
    tools: bool,
    /// Start MCP server
    #[arg(long, default_value = "false")]
    mcp: bool,
}

impl Args {
    fn is_args_available(&self) -> bool {
        self.prompts || self.resources || self.tools
    }
}

fn display_info(args: &Args) {
    if !args.is_args_available() {
        println!("Please use --help to see available options");
        return;
    }
    if args.prompts {
        println!(
            r#"prompts:
    - current_time: get current time in city
    "#
        );
    }
    if args.resources {
        println!(
            r#"resources:
    - sqlite: file:///path/to/sqlite.db
    "#
        );
    }
    if args.tools {
        println!(
            r#"tools:
    - get_current_time_in_city: get current time in city
    "#
        );
    }
}
