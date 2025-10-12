//! Application trait for marking top-level entry points.
//!
//! The Application trait represents the highest level of the hexagonal architecture,
//! marking entry points and coordinating system lifecycle. Applications orchestrate
//! the initialization of adapters, ports, and domain services, and manage the
//! application lifecycle from startup to shutdown.
//!
//! This trait follows the zero-boilerplate philosophy by providing sensible defaults
//! for all methods, allowing implementors to override only what they need.
//!
//! Revision History
//! - 2025-10-12T17:48:00Z @AI: Initial Application trait definition for marking entry points.

/// Trait for marking application entry points and coordinating lifecycle.
///
/// Applications represent the top level of the hexagonal architecture. They are
/// responsible for:
/// - Initializing the system (dependency injection, configuration loading)
/// - Running the application logic (starting servers, processing events)
/// - Shutting down gracefully (closing connections, flushing buffers)
///
/// All methods have default implementations that do nothing, following the
/// zero-boilerplate philosophy. Implementors override only the methods they need.
///
/// # Architecture Layer
///
/// The Application trait belongs to the **Application layer** in the DPAI model,
/// coordinating between Domain, Ports, and Adapters without containing business logic.
///
/// # Example
///
/// ```rust
/// use hexser::Application;
/// use hexser::HexResult;
///
/// struct WebApplication {
///     port: u16,
/// }
///
/// impl Application for WebApplication {
///     fn name(&self) -> &str {
///         "WebApplication"
///     }
///
///     fn initialize(&mut self) -> HexResult<()> {
///         std::println!("Initializing web server on port {}", self.port);
///         std::result::Result::Ok(())
///     }
///
///     fn run(&mut self) -> HexResult<()> {
///         std::println!("Starting web server...");
///         // Server startup logic here
///         std::result::Result::Ok(())
///     }
///
///     fn shutdown(&mut self) -> HexResult<()> {
///         std::println!("Shutting down web server...");
///         std::result::Result::Ok(())
///     }
/// }
///
/// fn main() -> HexResult<()> {
///     let mut app = WebApplication { port: 8080 };
///     app.initialize()?;
///     app.run()?;
///     app.shutdown()?;
///     std::result::Result::Ok(())
/// }
/// ```
///
/// # Minimal Example
///
/// ```rust
/// use hexser::Application;
///
/// struct MinimalApp;
///
/// impl Application for MinimalApp {
///     fn name(&self) -> &str {
///         "MinimalApp"
///     }
/// }
///
/// // All lifecycle methods have default no-op implementations
/// ```
pub trait Application {
  /// Returns the name of the application.
  ///
  /// This is the only required method, as every application should have
  /// a name for identification, logging, and debugging purposes.
  ///
  /// # Example
  ///
  /// ```rust
  /// use hexser::Application;
  ///
  /// struct MyApp;
  ///
  /// impl Application for MyApp {
  ///     fn name(&self) -> &str {
  ///         "MyApp"
  ///     }
  /// }
  ///
  /// let app = MyApp;
  /// std::assert_eq!(app.name(), "MyApp");
  /// ```
  fn name(&self) -> &str;

  /// Initialize the application.
  ///
  /// This method is called once at startup before `run()`. Use it to:
  /// - Load configuration
  /// - Set up dependency injection containers
  /// - Initialize adapters and ports
  /// - Validate system state
  ///
  /// Default implementation does nothing and returns `Ok(())`.
  ///
  /// # Errors
  ///
  /// Return an error if initialization fails (configuration invalid,
  /// required resources unavailable, etc.).
  fn initialize(&mut self) -> crate::result::hex_result::HexResult<()> {
    Result::Ok(())
  }

  /// Run the application.
  ///
  /// This method contains the main application logic. It may:
  /// - Start web servers or event loops
  /// - Process incoming requests
  /// - Execute batch operations
  /// - Run until interrupted
  ///
  /// Default implementation does nothing and returns `Ok(())`.
  ///
  /// # Errors
  ///
  /// Return an error if the application encounters a fatal error during execution.
  fn run(&mut self) -> crate::result::hex_result::HexResult<()> {
    Result::Ok(())
  }

  /// Shut down the application gracefully.
  ///
  /// This method is called when the application is terminating. Use it to:
  /// - Close database connections
  /// - Flush buffers
  /// - Save state
  /// - Release resources
  ///
  /// Default implementation does nothing and returns `Ok(())`.
  ///
  /// # Errors
  ///
  /// Return an error if shutdown operations fail (data loss, cleanup failure, etc.).
  fn shutdown(&mut self) -> crate::result::hex_result::HexResult<()> {
    Result::Ok(())
  }

  /// Execute the full application lifecycle: initialize, run, shutdown.
  ///
  /// This convenience method calls `initialize()`, `run()`, and `shutdown()`
  /// in sequence. If any step fails, subsequent steps are skipped and the
  /// error is returned.
  ///
  /// This is the recommended way to run an application from `main()`.
  ///
  /// # Example
  ///
  /// ```rust
  /// use hexser::Application;
  /// use hexser::HexResult;
  ///
  /// struct MyApp;
  ///
  /// impl Application for MyApp {
  ///     fn name(&self) -> &str {
  ///         "MyApp"
  ///     }
  /// }
  ///
  /// fn main() -> HexResult<()> {
  ///     let mut app = MyApp;
  ///     app.execute()
  /// }
  /// ```
  fn execute(&mut self) -> crate::result::hex_result::HexResult<()> {
    self.initialize()?;
    let run_result = self.run();
    let shutdown_result = self.shutdown();

    // Return run error if it occurred, otherwise return shutdown error
    run_result?;
    shutdown_result
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct TestApplication {
    name: &'static str,
    initialized: bool,
    ran: bool,
    shut_down: bool,
  }

  impl TestApplication {
    fn new(name: &'static str) -> Self {
      Self {
        name,
        initialized: false,
        ran: false,
        shut_down: false,
      }
    }
  }

  impl Application for TestApplication {
    fn name(&self) -> &str {
      self.name
    }

    fn initialize(&mut self) -> crate::result::hex_result::HexResult<()> {
      self.initialized = true;
      Result::Ok(())
    }

    fn run(&mut self) -> crate::result::hex_result::HexResult<()> {
      self.ran = true;
      Result::Ok(())
    }

    fn shutdown(&mut self) -> crate::result::hex_result::HexResult<()> {
      self.shut_down = true;
      Result::Ok(())
    }
  }

  #[test]
  fn test_application_name() {
    // Test: Validates that the name method returns the correct application name.
    // Justification: This is the only required method and must work correctly
    // for application identification.
    let app = TestApplication::new("TestApp");
    assert_eq!(app.name(), "TestApp");
  }

  #[test]
  fn test_application_lifecycle() {
    // Test: Validates that the full lifecycle (initialize, run, shutdown) executes correctly.
    // Justification: This verifies the primary use case where an application
    // goes through all lifecycle phases.
    let mut app = TestApplication::new("TestApp");

    assert!(!app.initialized);
    assert!(!app.ran);
    assert!(!app.shut_down);

    let result = app.execute();
    assert!(result.is_ok());

    assert!(app.initialized);
    assert!(app.ran);
    assert!(app.shut_down);
  }

  #[test]
  fn test_initialize_only() {
    // Test: Validates that initialize can be called independently.
    // Justification: Applications may need to initialize without running,
    // for example in testing scenarios.
    let mut app = TestApplication::new("TestApp");
    let result = app.initialize();
    assert!(result.is_ok());
    assert!(app.initialized);
    assert!(!app.ran);
    assert!(!app.shut_down);
  }

  #[test]
  fn test_shutdown_only() {
    // Test: Validates that shutdown can be called independently.
    // Justification: Applications may need emergency shutdown without
    // going through the full lifecycle.
    let mut app = TestApplication::new("TestApp");
    let result = app.shutdown();
    assert!(result.is_ok());
    assert!(!app.initialized);
    assert!(!app.ran);
    assert!(app.shut_down);
  }

  struct FailingApplication {
    fail_at: &'static str,
  }

  impl Application for FailingApplication {
    fn name(&self) -> &str {
      "FailingApp"
    }

    fn initialize(&mut self) -> crate::result::hex_result::HexResult<()> {
      if self.fail_at == "initialize" {
        return Result::Err(crate::error::hex_error::Hexserror::validation(
          "Initialization failed",
        ));
      }
      Result::Ok(())
    }

    fn run(&mut self) -> crate::result::hex_result::HexResult<()> {
      if self.fail_at == "run" {
        return Result::Err(crate::error::hex_error::Hexserror::validation("Run failed"));
      }
      Result::Ok(())
    }

    fn shutdown(&mut self) -> crate::result::hex_result::HexResult<()> {
      if self.fail_at == "shutdown" {
        return Result::Err(crate::error::hex_error::Hexserror::validation(
          "Shutdown failed",
        ));
      }
      Result::Ok(())
    }
  }

  #[test]
  fn test_execute_fails_on_initialize() {
    // Test: Validates that execute stops and returns error if initialize fails.
    // Justification: This ensures that failures during initialization prevent
    // the application from running in an invalid state.
    let mut app = FailingApplication {
      fail_at: "initialize",
    };
    let result = app.execute();
    assert!(result.is_err());
  }

  #[test]
  fn test_execute_fails_on_run() {
    // Test: Validates that execute attempts shutdown even if run fails.
    // Justification: This verifies that shutdown is called for cleanup
    // even when the run phase fails.
    let mut app = FailingApplication { fail_at: "run" };
    let result = app.execute();
    assert!(result.is_err());
  }

  #[test]
  fn test_execute_fails_on_shutdown() {
    // Test: Validates that execute returns error if shutdown fails.
    // Justification: Shutdown failures indicate resource cleanup problems
    // and should be reported to the caller.
    let mut app = FailingApplication {
      fail_at: "shutdown",
    };
    let result = app.execute();
    assert!(result.is_err());
  }

  struct MinimalApplication;

  impl Application for MinimalApplication {
    fn name(&self) -> &str {
      "MinimalApp"
    }
  }

  #[test]
  fn test_minimal_application_with_defaults() {
    // Test: Validates that an application with only name() implemented works correctly.
    // Justification: This verifies the zero-boilerplate philosophy where default
    // implementations allow minimal trait implementations.
    let mut app = MinimalApplication;
    assert_eq!(app.name(), "MinimalApp");

    let result = app.execute();
    assert!(result.is_ok());
  }
}
