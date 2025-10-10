//! Integration tests for environment variable controlled serialization.
//!
//! Tests verify that source location information in error types is
//! conditionally serialized based on the HEXSER_INCLUDE_SOURCE_LOCATION
//! environment variable. This is a security feature to prevent exposing
//! internal code structure to clients in production environments.
//!
//! Revision History
//! - 2025-10-09T21:51:00Z @AI: Initial test for conditional source location serialization.

#[cfg(feature = "serde")]
mod serde_tests {
  use hexser::error::rich_error::RichError;

  #[test]
  fn test_source_location_excluded_by_default() {
    unsafe {
      std::env::remove_var("HEXSER_INCLUDE_SOURCE_LOCATION");
    }

    let loc = hexser::error::source_location::SourceLocation::new("src/main.rs", 42, 10);
    let json = serde_json::to_string(&loc).expect("serialization should succeed");

    std::assert!(
      json.contains("src/main.rs") && json.contains("42") && json.contains("10"),
      "SourceLocation struct itself always serializes its fields"
    );
  }

  #[test]
  fn test_layer_error_location_excluded_by_default() {
    unsafe {
      std::env::remove_var("HEXSER_INCLUDE_SOURCE_LOCATION");
    }

    let loc = hexser::error::source_location::SourceLocation::new("src/domain.rs", 100, 5);
    let err =
      hexser::error::domain_error::DomainError::new("E_DOM_001", "Test error").with_location(loc);

    let json = serde_json::to_string(&err).expect("serialization should succeed");

    std::assert!(
      !json.contains("src/domain.rs") && !json.contains("100"),
      "Location should be excluded by default for security. JSON: {}",
      json
    );
  }

  #[test]
  fn test_layer_error_location_included_when_enabled() {
    unsafe {
      std::env::set_var("HEXSER_INCLUDE_SOURCE_LOCATION", "1");
    }

    let loc = hexser::error::source_location::SourceLocation::new("src/domain.rs", 100, 5);
    let err =
      hexser::error::domain_error::DomainError::new("E_DOM_001", "Test error").with_location(loc);

    let json = serde_json::to_string(&err).expect("serialization should succeed");

    std::assert!(
      json.contains("src/domain.rs") && json.contains("100"),
      "Location should be included when env var is set. JSON: {}",
      json
    );

    unsafe {
      std::env::remove_var("HEXSER_INCLUDE_SOURCE_LOCATION");
    }
  }

  #[test]
  fn test_validation_error_location_excluded_by_default() {
    unsafe {
      std::env::remove_var("HEXSER_INCLUDE_SOURCE_LOCATION");
    }

    let loc = hexser::error::source_location::SourceLocation::new("src/validation.rs", 50, 1);
    let err = hexser::error::validation_error::ValidationError::new("E_VAL_001", "Invalid input")
      .with_field("email")
      .with_location(loc);

    let json = serde_json::to_string(&err).expect("serialization should succeed");

    std::assert!(
      !json.contains("src/validation.rs") && !json.contains("50"),
      "Location should be excluded by default. JSON: {}",
      json
    );
  }

  #[test]
  fn test_validation_error_location_included_when_enabled() {
    unsafe {
      std::env::set_var("HEXSER_INCLUDE_SOURCE_LOCATION", "true");
    }

    let loc = hexser::error::source_location::SourceLocation::new("src/validation.rs", 50, 1);
    let err = hexser::error::validation_error::ValidationError::new("E_VAL_001", "Invalid input")
      .with_field("email")
      .with_location(loc);

    let json = serde_json::to_string(&err).expect("serialization should succeed");

    std::assert!(
      json.contains("src/validation.rs") && json.contains("50"),
      "Location should be included when env var is true. JSON: {}",
      json
    );

    unsafe {
      std::env::remove_var("HEXSER_INCLUDE_SOURCE_LOCATION");
    }
  }

  #[test]
  fn test_not_found_error_location_excluded_by_default() {
    unsafe {
      std::env::remove_var("HEXSER_INCLUDE_SOURCE_LOCATION");
    }

    let loc = hexser::error::source_location::SourceLocation::new("src/repo.rs", 200, 15);
    let err =
      hexser::error::not_found_error::NotFoundError::new("User", "user-123").with_location(loc);

    let json = serde_json::to_string(&err).expect("serialization should succeed");

    std::assert!(
      !json.contains("src/repo.rs") && !json.contains("200"),
      "Location should be excluded by default. JSON: {}",
      json
    );
  }

  #[test]
  fn test_not_found_error_location_included_when_enabled() {
    unsafe {
      std::env::set_var("HEXSER_INCLUDE_SOURCE_LOCATION", "TRUE");
    }

    let loc = hexser::error::source_location::SourceLocation::new("src/repo.rs", 200, 15);
    let err =
      hexser::error::not_found_error::NotFoundError::new("User", "user-123").with_location(loc);

    let json = serde_json::to_string(&err).expect("serialization should succeed");

    std::assert!(
      json.contains("src/repo.rs") && json.contains("200"),
      "Location should be included when env var is TRUE. JSON: {}",
      json
    );

    unsafe {
      std::env::remove_var("HEXSER_INCLUDE_SOURCE_LOCATION");
    }
  }

  #[test]
  fn test_conflict_error_location_excluded_by_default() {
    unsafe {
      std::env::remove_var("HEXSER_INCLUDE_SOURCE_LOCATION");
    }

    let loc = hexser::error::source_location::SourceLocation::new("src/service.rs", 75, 8);
    let err = hexser::error::conflict_error::ConflictError::new("Email already exists")
      .with_existing_id("user-456")
      .with_location(loc);

    let json = serde_json::to_string(&err).expect("serialization should succeed");

    std::assert!(
      !json.contains("src/service.rs") && !json.contains("75"),
      "Location should be excluded by default. JSON: {}",
      json
    );
  }

  #[test]
  fn test_conflict_error_location_included_when_enabled() {
    unsafe {
      std::env::set_var("HEXSER_INCLUDE_SOURCE_LOCATION", "1");
    }

    let loc = hexser::error::source_location::SourceLocation::new("src/service.rs", 75, 8);
    let err = hexser::error::conflict_error::ConflictError::new("Email already exists")
      .with_existing_id("user-456")
      .with_location(loc);

    let json = serde_json::to_string(&err).expect("serialization should succeed");

    std::assert!(
      json.contains("src/service.rs") && json.contains("75"),
      "Location should be included when env var is set. JSON: {}",
      json
    );

    unsafe {
      std::env::remove_var("HEXSER_INCLUDE_SOURCE_LOCATION");
    }
  }

  #[test]
  fn test_error_without_location_serializes_correctly() {
    unsafe {
      std::env::remove_var("HEXSER_INCLUDE_SOURCE_LOCATION");
    }

    let err = hexser::error::domain_error::DomainError::new("E_DOM_002", "Error without location");

    let json = serde_json::to_string(&err).expect("serialization should succeed");

    std::assert!(
      json.contains("E_DOM_002") && json.contains("Error without location"),
      "Error without location should serialize normally. JSON: {}",
      json
    );
  }

  #[test]
  fn test_deserialization_works_regardless_of_env_var() {
    unsafe {
      std::env::remove_var("HEXSER_INCLUDE_SOURCE_LOCATION");
    }

    let json_with_location = r#"{"code":"E_DOM_001","message":"Test","next_steps":[],"suggestions":[],"location":{"file":"test.rs","line":1,"column":1},"more_info_url":null,"layer":null}"#;
    let json_without_location = r#"{"code":"E_DOM_001","message":"Test","next_steps":[],"suggestions":[],"more_info_url":null,"layer":null}"#;

    let result1: Result<hexser::error::domain_error::DomainError, _> =
      serde_json::from_str(json_with_location);
    let result2: Result<hexser::error::domain_error::DomainError, _> =
      serde_json::from_str(json_without_location);

    std::assert!(result1.is_ok(), "Should deserialize with location");
    std::assert!(result2.is_ok(), "Should deserialize without location");
  }
}

#[cfg(not(feature = "serde"))]
fn main() {
  println!("Serde tests require the 'serde' feature to be enabled.");
}
