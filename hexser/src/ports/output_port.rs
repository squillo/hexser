//! OutputPort trait for secondary adapters.
//!
//! Output ports define interfaces for interacting with external systems
//! from within the application. They represent dependencies that the application
//! needs, such as databases, message queues, or external APIs. Adapters implement
//! output ports to provide concrete implementations using specific technologies.
//!
//! Revision History
//! - 2025-10-01T00:00:00Z @AI: Initial OutputPort trait definition for secondary ports.

/// Trait for output ports that define interfaces to external systems.
///
/// Output ports are implemented by adapters that provide access to
/// infrastructure concerns like databases, messaging, or external services.
///
/// # Type Parameters
///
/// * `Request` - The type of request sent to the external system
/// * `Response` - The type of response received from the external system
///
/// # Example
///
/// ```rust
/// use hexser::ports::OutputPort;
/// use hexser::HexResult;
///
/// struct EmailRequest {
///     to: String,
///     subject: String,
///     body: String,
/// }
///
/// struct EmailResponse {
///     message_id: String,
/// }
///
/// trait EmailPort: OutputPort<EmailRequest, EmailResponse> {}
/// ```
pub trait OutputPort<Request, Response> {
  /// Send a request to the external system.
  ///
  /// Returns the response if successful, or an error if the operation fails.
  fn send(&self, request: Request) -> crate::result::hex_result::HexResult<Response>;
}

#[cfg(test)]
mod tests {
  use super::*;

  struct TestRequest {
    data: String,
  }

  struct TestResponse {
    status: String,
  }

  struct TestOutputPort;

  impl OutputPort<TestRequest, TestResponse> for TestOutputPort {
    fn send(&self, _request: TestRequest) -> crate::result::hex_result::HexResult<TestResponse> {
      Result::Ok(TestResponse {
        status: String::from("success"),
      })
    }
  }

  #[test]
  fn test_output_port_send() {
    let port = TestOutputPort;
    let request = TestRequest {
      data: String::from("test"),
    };
    let response = port.send(request).unwrap();
    assert_eq!(response.status, "success");
  }
}
