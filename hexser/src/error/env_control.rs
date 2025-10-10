//! Environment variable control for conditional error field serialization.
//!
//! Provides runtime control over which error fields are serialized based on
//! environment variables. This allows production environments to hide sensitive
//! debugging information (like source file paths and line numbers) while
//! development environments can include them for easier debugging.
//!
//! Revision History
//! - 2025-10-09T21:51:00Z @AI: Initial implementation for conditional source location serialization.

/// Checks if source location information should be included in serialization.
///
/// Returns true if the HEXSER_INCLUDE_SOURCE_LOCATION environment variable
/// is set to "1" or "true" (case-insensitive). Returns false otherwise.
///
/// # Security Note
///
/// By default (when the environment variable is not set), source location
/// information is excluded from serialization to prevent exposing internal
/// code structure to clients. This is the recommended setting for production.
///
/// # Examples
///
/// ```
/// // By default, returns false (secure by default)
/// let result = hexser::error::env_control::should_include_source_location();
/// std::assert_eq!(result, false);
/// ```
pub fn should_include_source_location() -> bool {
  std::env::var("HEXSER_INCLUDE_SOURCE_LOCATION")
    .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
    .unwrap_or(false)
}

/// Helper function for serde skip_serializing_if attribute.
///
/// Returns true if the Option<SourceLocation> should be skipped during serialization.
/// This happens when either the value is None OR when source location should not be included.
///
/// # Examples
///
/// ```
/// // Used with serde attribute:
/// // #[cfg_attr(feature = "serde", serde(skip_serializing_if = "crate::error::env_control::should_skip_location"))]
/// // pub location: Option<crate::error::source_location::SourceLocation>,
/// ```
#[cfg(feature = "serde")]
pub fn should_skip_location(
  location: &Option<crate::error::source_location::SourceLocation>,
) -> bool {
  location.is_none() || !should_include_source_location()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_should_include_source_location_default() {
    // Clear the environment variable to test default behavior
    unsafe {
      std::env::remove_var("HEXSER_INCLUDE_SOURCE_LOCATION");
    }
    std::assert_eq!(should_include_source_location(), false);
  }

  #[test]
  fn test_should_include_source_location_enabled_with_1() {
    unsafe {
      std::env::set_var("HEXSER_INCLUDE_SOURCE_LOCATION", "1");
    }
    std::assert_eq!(should_include_source_location(), true);
    unsafe {
      std::env::remove_var("HEXSER_INCLUDE_SOURCE_LOCATION");
    }
  }

  #[test]
  fn test_should_include_source_location_enabled_with_true() {
    unsafe {
      std::env::set_var("HEXSER_INCLUDE_SOURCE_LOCATION", "true");
    }
    std::assert_eq!(should_include_source_location(), true);
    unsafe {
      std::env::remove_var("HEXSER_INCLUDE_SOURCE_LOCATION");
    }
  }

  #[test]
  fn test_should_include_source_location_enabled_with_true_uppercase() {
    unsafe {
      std::env::set_var("HEXSER_INCLUDE_SOURCE_LOCATION", "TRUE");
    }
    std::assert_eq!(should_include_source_location(), true);
    unsafe {
      std::env::remove_var("HEXSER_INCLUDE_SOURCE_LOCATION");
    }
  }

  #[test]
  fn test_should_include_source_location_disabled_with_other_value() {
    unsafe {
      std::env::set_var("HEXSER_INCLUDE_SOURCE_LOCATION", "false");
    }
    std::assert_eq!(should_include_source_location(), false);
    unsafe {
      std::env::remove_var("HEXSER_INCLUDE_SOURCE_LOCATION");
    }
  }

  #[test]
  #[cfg(feature = "serde")]
  fn test_should_skip_location_when_none() {
    unsafe {
      std::env::remove_var("HEXSER_INCLUDE_SOURCE_LOCATION");
    }
    let location: Option<crate::error::source_location::SourceLocation> = None;
    std::assert_eq!(should_skip_location(&location), true);
  }

  #[test]
  #[cfg(feature = "serde")]
  fn test_should_skip_location_when_some_but_disabled() {
    unsafe {
      std::env::remove_var("HEXSER_INCLUDE_SOURCE_LOCATION");
    }
    let location = Some(crate::error::source_location::SourceLocation::new(
      "test.rs", 1, 1,
    ));
    std::assert_eq!(should_skip_location(&location), true);
  }

  #[test]
  #[cfg(feature = "serde")]
  fn test_should_not_skip_location_when_some_and_enabled() {
    unsafe {
      std::env::set_var("HEXSER_INCLUDE_SOURCE_LOCATION", "1");
    }
    let location = Some(crate::error::source_location::SourceLocation::new(
      "test.rs", 1, 1,
    ));
    std::assert_eq!(should_skip_location(&location), false);
    unsafe {
      std::env::remove_var("HEXSER_INCLUDE_SOURCE_LOCATION");
    }
  }
}
