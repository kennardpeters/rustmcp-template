use anyhow::{Result, Context};
use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::{Parameters}}, 
    model::*, schemars, service::{NotificationContext, QuitReason, RequestContext}, tool, tool_handler, tool_router, ErrorData, RoleServer, ServerHandler
};
use rmcp::{ServiceExt};

use serde::{Deserialize, Serialize};
use serde_json::{json};
use tokio::io::{stdin, stdout};

// Define the arguments struct
#[derive(Deserialize, Serialize, schemars::JsonSchema)]
struct InputArgs {
    #[schemars(description = "Description for parameter 1")]
    pub param_1: String,
    #[schemars(description = "Description for parameter 2")]
    pub param_2: Option<String>,
}

#[derive(Deserialize, Serialize, schemars::JsonSchema)]
struct OutputResults {
    pub field_1: String,
    pub field_2: String,
}

#[derive(Debug, Clone)]
pub struct ExampleHandler {
    tool_router: ToolRouter<Self>,
}

#[tool_router(router = tool_router)]
impl ExampleHandler {

    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Get additional Context")]
    async fn get_example_context(&self, Parameters(req): Parameters<InputArgs>) -> Result<CallToolResult, ErrorData> {

        // *** PUT YOUR TOOL LOGIC HERE *****
        //
        // Fake logic (simulating DB)
        let stats = json!({
            "field_1": req.param_1,
            "field_2": "result".to_string(),
        });

        Ok(CallToolResult::success(vec![Content::json(stats).unwrap()]))

    }
}

// Implement ServerHandler trait
#[tool_handler(router = self.tool_router)]
impl ServerHandler for ExampleHandler {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities {
                tools: Some(ToolsCapability {
                    list_changed: Some(false),
                }),
                ..Default::default()
            },
            instructions: Some("Example MCP Server providing additonal context through Model Context Protocol".into()),
            server_info: Implementation {
                name: "example-mcp-server".into(),
                version: env!("CARGO_PKG_VERSION").into(),
                ..Default::default()
            },
        }
    }

    async fn ping(&self, _ctx: RequestContext<RoleServer>) -> Result<(), ErrorData> {
        eprintln!("Received ping request");
        Ok(())
    }

    async fn on_initialized(&self, _ctx: NotificationContext<RoleServer>) {
        eprintln!("Client initialized successfully");
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the tracing subscriber with file and stdout logging
    // tracing_subscriber::fmt()
    //     .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
    //     .with_writer(std::io::stderr)
    //     .with_ansi(false)
    //     .init();
    eprintln!("MCP Server initialized");

    // tracing::info!("Starting MCP server");

    // Create an instance of our counter router
    let server = ExampleHandler::new();

    match server.serve((stdin(), stdout())).await
        .context("Failed to serve MCP server")
        .inspect_err(|e| {
        eprintln!("serving error: {:?}", e);
        // tracing::error!("serving error: {:?}", e);
    })?.waiting().await {
        Ok(..) => QuitReason::Closed,
        Err(e) => {
            eprintln!("AN error occurred while waiting: {:?}", e);
            QuitReason::Cancelled
        },
    };

    Ok(())
}
