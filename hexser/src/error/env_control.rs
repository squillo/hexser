//! Environment variable control for conditional error field serialization.
//!
//! Provides runtime control over which error fields are serialized based on
//! environment variables. This allows production environments to hide sensitive
//! debugging information (like source file paths and line numbers) while
//! development environments can include them for easier debugging.
//!
//! Revision History
//! - 2026-07-20T00:00:00Z @AI: Serialize env-mutating tests with #[serial(hexser_env)]; fix rustdoc (backtick generics, text code block); add /// why on tests.
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
/// Returns true if the `Option<SourceLocation>` should be skipped during serialization.
/// This happens when either the value is None OR when source location should not be included.
///
/// # Examples
///
/// ```text
/// // Used with serde attribute:
/// #[cfg_attr(feature = "serde", serde(skip_serializing_if = "crate::error::env_control::should_skip_location"))]
/// pub location: Option<crate::error::source_location::SourceLocation>,
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

  // These tests mutate the process-global HEXSER_INCLUDE_SOURCE_LOCATION variable while
  // `should_include_source_location` reads it. Cargo runs tests in parallel threads within
  // a binary, so they must be serialized against each other (and any other reader of this
  // var) to avoid interleaving that flips the observed value. `#[serial(hexser_env)]` locks
  // all env-touching tests in this process onto one key.

  /// why: with the var unset, the crate must default to NOT leaking source locations
  /// (privacy-safe default); guards the `unwrap_or(false)` fallback.
  #[test]
  #[serial_test::serial(hexser_env)]
  fn test_should_include_source_location_default() {
    unsafe {
      std::env::remove_var("HEXSER_INCLUDE_SOURCE_LOCATION");
    }
    std::assert_eq!(should_include_source_location(), false);
  }

  /// why: "1" is the documented opt-in value and must enable inclusion.
  #[test]
  #[serial_test::serial(hexser_env)]
  fn test_should_include_source_location_enabled_with_1() {
    unsafe {
      std::env::set_var("HEXSER_INCLUDE_SOURCE_LOCATION", "1");
    }
    std::assert_eq!(should_include_source_location(), true);
    unsafe {
      std::env::remove_var("HEXSER_INCLUDE_SOURCE_LOCATION");
    }
  }

  /// why: "true" is the documented opt-in value and must enable inclusion.
  #[test]
  #[serial_test::serial(hexser_env)]
  fn test_should_include_source_location_enabled_with_true() {
    unsafe {
      std::env::set_var("HEXSER_INCLUDE_SOURCE_LOCATION", "true");
    }
    std::assert_eq!(should_include_source_location(), true);
    unsafe {
      std::env::remove_var("HEXSER_INCLUDE_SOURCE_LOCATION");
    }
  }

  /// why: matching must be case-insensitive so "TRUE" also opts in (documented behavior).
  #[test]
  #[serial_test::serial(hexser_env)]
  fn test_should_include_source_location_enabled_with_true_uppercase() {
    unsafe {
      std::env::set_var("HEXSER_INCLUDE_SOURCE_LOCATION", "TRUE");
    }
    std::assert_eq!(should_include_source_location(), true);
    unsafe {
      std::env::remove_var("HEXSER_INCLUDE_SOURCE_LOCATION");
    }
  }

  /// why: any value other than the accepted opt-ins (e.g. "false") must NOT enable
  /// inclusion, so a typo can never accidentally leak locations.
  #[test]
  #[serial_test::serial(hexser_env)]
  fn test_should_include_source_location_disabled_with_other_value() {
    unsafe {
      std::env::set_var("HEXSER_INCLUDE_SOURCE_LOCATION", "false");
    }
    std::assert_eq!(should_include_source_location(), false);
    unsafe {
      std::env::remove_var("HEXSER_INCLUDE_SOURCE_LOCATION");
    }
  }

  /// why: a `None` location is always skipped regardless of the env toggle.
  #[test]
  #[cfg(feature = "serde")]
  #[serial_test::serial(hexser_env)]
  fn test_should_skip_location_when_none() {
    unsafe {
      std::env::remove_var("HEXSER_INCLUDE_SOURCE_LOCATION");
    }
    let location: Option<crate::error::source_location::SourceLocation> = None;
    std::assert_eq!(should_skip_location(&location), true);
  }

  /// why: a present location must still be skipped when inclusion is disabled (default),
  /// so serialized errors don't leak file paths by accident.
  #[test]
  #[cfg(feature = "serde")]
  #[serial_test::serial(hexser_env)]
  fn test_should_skip_location_when_some_but_disabled() {
    unsafe {
      std::env::remove_var("HEXSER_INCLUDE_SOURCE_LOCATION");
    }
    let location = Some(crate::error::source_location::SourceLocation::new(
      "test.rs", 1, 1,
    ));
    std::assert_eq!(should_skip_location(&location), true);
  }

  /// why: a present location must be serialized (not skipped) only when inclusion is
  /// explicitly enabled.
  #[test]
  #[cfg(feature = "serde")]
  #[serial_test::serial(hexser_env)]
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
