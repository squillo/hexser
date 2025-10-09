//! JSON-RPC 2.0 protocol types for Model Context Protocol.
//!
//! Defines request, response, and error structures conforming to JSON-RPC 2.0.
//! MCP uses JSON-RPC 2.0 as transport layer for all client-server communication.
//! Supports both method calls with parameters and notification messages.
//!
//! Revision History
//! - 2025-10-08T23:35:00Z @AI: Initial JSON-RPC 2.0 protocol types.

/// JSON-RPC 2.0 request structure.
///
/// Represents a method call from client to server. Contains method name,
/// optional parameters (as JSON value), and request ID for correlation.
/// The jsonrpc field must always be "2.0" per specification.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct JsonRpcRequest {
    /// JSON-RPC protocol version, must be "2.0"
    pub jsonrpc: String,

    /// Request identifier for correlation with response
    pub id: serde_json::Value,

    /// Method name to invoke
    pub method: String,

    /// Optional parameters for the method
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
}

impl JsonRpcRequest {
    /// Creates a new JSON-RPC request with the given method and parameters.
    ///
    /// # Arguments
    ///
    /// * `id` - Request identifier for correlation
    /// * `method` - Method name to invoke
    /// * `params` - Optional parameters as JSON value
    ///
    /// # Returns
    ///
    /// A new JsonRpcRequest instance
    pub fn new(id: serde_json::Value, method: String, params: Option<serde_json::Value>) -> Self {
        JsonRpcRequest {
            jsonrpc: String::from("2.0"),
            id,
            method,
            params,
        }
    }
}

/// JSON-RPC 2.0 response structure.
///
/// Represents a successful response from server to client. Contains either
/// a result (on success) or an error (on failure), but never both.
/// The id field correlates this response with the originating request.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct JsonRpcResponse {
    /// JSON-RPC protocol version, must be "2.0"
    pub jsonrpc: String,

    /// Request identifier matching the original request
    pub id: serde_json::Value,

    /// Successful result (mutually exclusive with error)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,

    /// Error information (mutually exclusive with result)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

impl JsonRpcResponse {
    /// Creates a successful JSON-RPC response with a result.
    ///
    /// # Arguments
    ///
    /// * `id` - Request identifier matching the original request
    /// * `result` - Successful result as JSON value
    ///
    /// # Returns
    ///
    /// A new JsonRpcResponse with result set
    pub fn success(id: serde_json::Value, result: serde_json::Value) -> Self {
        JsonRpcResponse {
            jsonrpc: String::from("2.0"),
            id,
            result: Some(result),
            error: None,
        }
    }

    /// Creates an error JSON-RPC response.
    ///
    /// # Arguments
    ///
    /// * `id` - Request identifier matching the original request
    /// * `error` - Error information
    ///
    /// # Returns
    ///
    /// A new JsonRpcResponse with error set
    pub fn error(id: serde_json::Value, error: JsonRpcError) -> Self {
        JsonRpcResponse {
            jsonrpc: String::from("2.0"),
            id,
            result: None,
            error: Some(error),
        }
    }
}

/// JSON-RPC 2.0 error structure.
///
/// Represents an error that occurred during request processing.
/// Includes error code (standard or application-defined), message,
/// and optional additional data for debugging.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct JsonRpcError {
    /// Error code (standard codes: -32768 to -32000)
    pub code: i32,

    /// Human-readable error message
    pub message: String,

    /// Optional additional error data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl JsonRpcError {
    /// Creates a new JSON-RPC error.
    ///
    /// # Arguments
    ///
    /// * `code` - Error code
    /// * `message` - Error message
    /// * `data` - Optional additional data
    ///
    /// # Returns
    ///
    /// A new JsonRpcError instance
    pub fn new(code: i32, message: String, data: Option<serde_json::Value>) -> Self {
        JsonRpcError { code, message, data }
    }

    /// Creates a parse error (-32700).
    pub fn parse_error(message: String) -> Self {
        JsonRpcError::new(-32700, message, None)
    }

    /// Creates an invalid request error (-32600).
    pub fn invalid_request(message: String) -> Self {
        JsonRpcError::new(-32600, message, None)
    }

    /// Creates a method not found error (-32601).
    pub fn method_not_found(method: String) -> Self {
        JsonRpcError::new(-32601, format!("Method not found: {}", method), None)
    }

    /// Creates an internal error (-32603).
    pub fn internal_error(message: String) -> Self {
        JsonRpcError::new(-32603, message, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_rpc_request_serialization() {
        let req = JsonRpcRequest::new(
            serde_json::Value::Number(serde_json::Number::from(1)),
            String::from("initialize"),
            Some(serde_json::json!({"protocolVersion": "2024-11-05"})),
        );

        let json = serde_json::to_string(&req).unwrap();
        std::assert!(json.contains("\"jsonrpc\":\"2.0\""));
        std::assert!(json.contains("\"method\":\"initialize\""));
    }

    #[test]
    fn test_json_rpc_success_response() {
        let resp = JsonRpcResponse::success(
            serde_json::Value::Number(serde_json::Number::from(1)),
            serde_json::json!({"status": "ok"}),
        );

        std::assert_eq!(resp.jsonrpc, "2.0");
        std::assert!(resp.result.is_some());
        std::assert!(resp.error.is_none());
    }

    #[test]
    fn test_json_rpc_error_response() {
        let error = JsonRpcError::method_not_found(String::from("unknown"));
        let resp = JsonRpcResponse::error(
            serde_json::Value::Number(serde_json::Number::from(1)),
            error,
        );

        std::assert!(resp.result.is_none());
        std::assert!(resp.error.is_some());
        std::assert_eq!(resp.error.unwrap().code, -32601);
    }
}
