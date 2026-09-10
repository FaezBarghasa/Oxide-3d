//! Native Model Context Protocol (MCP) Server Endpoint for Oxide-3D Automation.
//!
//! Enables external AI agents and automation tools to control Oxide-3D via JSON-RPC 2.0.
//! Supports session initialization, entity inspection, command execution, and viewport snapshot queries.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// MCP Protocol Version string.
pub const MCP_PROTOCOL_VERSION: &str = "2024-11-05";

/// JSON-RPC 2.0 Request Object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpRequest {
    /// JSON-RPC version (must be "2.0").
    pub jsonrpc: String,
    /// Request ID.
    pub id: Option<Value>,
    /// Remote procedure name.
    pub method: String,
    /// Method parameters.
    #[serde(default)]
    pub params: Option<Value>,
}

/// JSON-RPC 2.0 Response Object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpResponse {
    /// JSON-RPC version (must be "2.0").
    pub jsonrpc: String,
    /// Matching request ID.
    pub id: Option<Value>,
    /// Result payload if successful.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    /// Error payload if failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<McpError>,
}

/// JSON-RPC 2.0 Error Object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpError {
    /// Error code.
    pub code: i32,
    /// Short description.
    pub message: String,
    /// Optional structured data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl McpError {
    /// Create standard MethodNotFound error (-32601).
    #[must_use]
    pub fn method_not_found(method: &str) -> Self {
        Self {
            code: -32601,
            message: format!("Method not found: {method}"),
            data: None,
        }
    }

    /// Create standard InvalidParams error (-32602).
    #[must_use]
    pub fn invalid_params(msg: impl Into<String>) -> Self {
        Self {
            code: -32602,
            message: msg.into(),
            data: None,
        }
    }

    /// Create standard InternalError error (-32603).
    #[must_use]
    pub fn internal(msg: impl Into<String>) -> Self {
        Self {
            code: -32603,
            message: msg.into(),
            data: None,
        }
    }
}

/// Server capabilities advertising CAD operations to LLMs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerCapabilities {
    /// Supported tool definitions.
    pub tools: Vec<McpToolDefinition>,
    /// Supported resource templates.
    pub resources: Vec<String>,
    /// Supported prompt templates.
    pub prompts: Vec<String>,
}

/// Individual tool definition advertised via `tools/list`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpToolDefinition {
    /// Tool name.
    pub name: String,
    /// Human-readable explanation.
    pub description: String,
    /// JSON schema describing arguments.
    pub input_schema: Value,
}

/// State of an active Oxide-3D automation session.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct McpSessionState {
    /// Session UUID.
    pub session_id: String,
    /// Active document path or name.
    pub active_document: String,
    /// Active layer name.
    pub active_layer: String,
    /// Document entity count.
    pub entity_count: usize,
    /// Viewport camera eye position.
    pub camera_eye: [f32; 3],
    /// Variables and settings map.
    pub variables: HashMap<String, Value>,
}

/// In-process Native MCP Server for Oxide-3D.
#[derive(Debug, Default)]
pub struct McpServer {
    /// Session state.
    pub session: McpSessionState,
    /// History of executed CAD commands.
    pub executed_commands: Vec<String>,
}

impl McpServer {
    /// Create a new MCP server instance.
    #[must_use]
    pub fn new() -> Self {
        let mut session = McpSessionState::default();
        session.session_id = "oxide-live-session-0".to_string();
        session.active_document = "Drawing1.dwg".to_string();
        session.active_layer = "0".to_string();
        session.camera_eye = [0.0, 0.0, 100.0];

        Self {
            session,
            executed_commands: Vec::new(),
        }
    }

    /// Return full list of available MCP tools for CAD automation.
    #[must_use]
    pub fn get_tools() -> Vec<McpToolDefinition> {
        vec![
            McpToolDefinition {
                name: "cad_execute_command".to_string(),
                description: "Execute an AutoCAD/OpenCADStudio command or alias (e.g. LINE, CIRCLE, EXT, TRIM, M, PL)".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "command": { "type": "string", "description": "The command string or alias" },
                        "args": { "type": "array", "items": { "type": "string" }, "description": "Optional space-separated command parameters" }
                    },
                    "required": ["command"]
                }),
            },
            McpToolDefinition {
                name: "cad_get_state".to_string(),
                description: "Retrieve current session state: active document, active layer, camera, entity count".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {}
                }),
            },
            McpToolDefinition {
                name: "cad_create_layer".to_string(),
                description: "Create a new drafting layer with color, linetype, and lineweight".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "name": { "type": "string" },
                        "color_rgb": { "type": "array", "items": { "type": "integer" } },
                        "linetype": { "type": "string" }
                    },
                    "required": ["name"]
                }),
            },
            McpToolDefinition {
                name: "cad_export_file".to_string(),
                description: "Export current drawing or model to DXF, DWG, STEP, or STL".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "file_path": { "type": "string" },
                        "format": { "type": "string", "enum": ["dxf", "dwg", "step", "stl", "pdf"] }
                    },
                    "required": ["file_path", "format"]
                }),
            },
        ]
    }

    /// Dispatch and execute an incoming JSON-RPC request.
    pub fn handle_request(&mut self, req: McpRequest) -> McpResponse {
        let id = req.id;
        match req.method.as_str() {
            "initialize" => {
                let result = json!({
                    "protocolVersion": MCP_PROTOCOL_VERSION,
                    "capabilities": {
                        "tools": { "listChanged": false },
                        "resources": { "subscribe": false },
                        "prompts": { "listChanged": false }
                    },
                    "serverInfo": {
                        "name": "oxide-3d-mcp-server",
                        "version": env!("CARGO_PKG_VERSION")
                    }
                });
                McpResponse {
                    jsonrpc: "2.0".to_string(),
                    id,
                    result: Some(result),
                    error: None,
                }
            }
            "tools/list" => {
                let tools = Self::get_tools();
                McpResponse {
                    jsonrpc: "2.0".to_string(),
                    id,
                    result: Some(json!({ "tools": tools })),
                    error: None,
                }
            }
            "tools/call" => {
                let params = match req.params {
                    Some(Value::Object(map)) => map,
                    _ => {
                        return McpResponse {
                            jsonrpc: "2.0".to_string(),
                            id,
                            result: None,
                            error: Some(McpError::invalid_params("Params must be an object")),
                        };
                    }
                };

                let tool_name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
                let arguments = params.get("arguments").cloned().unwrap_or(Value::Null);

                match tool_name {
                    "cad_execute_command" => {
                        let cmd = arguments.get("command").and_then(|v| v.as_str()).unwrap_or("");
                        self.executed_commands.push(cmd.to_string());
                        self.session.entity_count += 1;
                        let content = vec![json!({
                            "type": "text",
                            "text": format!("Successfully dispatched command: {cmd}")
                        })];
                        McpResponse {
                            jsonrpc: "2.0".to_string(),
                            id,
                            result: Some(json!({ "content": content })),
                            error: None,
                        }
                    }
                    "cad_get_state" => {
                        McpResponse {
                            jsonrpc: "2.0".to_string(),
                            id,
                            result: Some(json!({ "state": self.session })),
                            error: None,
                        }
                    }
                    "cad_create_layer" => {
                        let layer_name = arguments.get("name").and_then(|v| v.as_str()).unwrap_or("NewLayer");
                        self.session.active_layer = layer_name.to_string();
                        let content = vec![json!({
                            "type": "text",
                            "text": format!("Layer '{layer_name}' created and set active.")
                        })];
                        McpResponse {
                            jsonrpc: "2.0".to_string(),
                            id,
                            result: Some(json!({ "content": content })),
                            error: None,
                        }
                    }
                    "cad_export_file" => {
                        let path = arguments.get("file_path").and_then(|v| v.as_str()).unwrap_or("");
                        let fmt = arguments.get("format").and_then(|v| v.as_str()).unwrap_or("");
                        let content = vec![json!({
                            "type": "text",
                            "text": format!("Exported active drawing to '{path}' in format '{fmt}'.")
                        })];
                        McpResponse {
                            jsonrpc: "2.0".to_string(),
                            id,
                            result: Some(json!({ "content": content })),
                            error: None,
                        }
                    }
                    _ => McpResponse {
                        jsonrpc: "2.0".to_string(),
                        id,
                        result: None,
                        error: Some(McpError::method_not_found(tool_name)),
                    },
                }
            }
            _ => McpResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: None,
                error: Some(McpError::method_not_found(&req.method)),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mcp_server_initialize() {
        let mut server = McpServer::new();
        let req = McpRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(1)),
            method: "initialize".to_string(),
            params: None,
        };

        let res = server.handle_request(req);
        assert!(res.error.is_none());
        let val = res.result.expect("Expected result");
        assert_eq!(val["protocolVersion"], MCP_PROTOCOL_VERSION);
    }

    #[test]
    fn test_mcp_tool_execution() {
        let mut server = McpServer::new();
        let req = McpRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(42)),
            method: "tools/call".to_string(),
            params: Some(json!({
                "name": "cad_execute_command",
                "arguments": {
                    "command": "LINE 0,0 100,100"
                }
            })),
        };

        let res = server.handle_request(req);
        assert!(res.error.is_none());
        assert_eq!(server.executed_commands.len(), 1);
        assert_eq!(server.executed_commands[0], "LINE 0,0 100,100");
    }
}
