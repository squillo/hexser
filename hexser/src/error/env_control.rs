//! Environment variable control for conditional error field serialization.
//!
//! Provides runtime control over which error fields are serialized based on
//! environment variables. This allows production environments to hide sensitive
//! debugging information (like source file paths and line numbers) while
//! development environments can include them for easier debugging.
//!
//! The switch is read ONCE per process, at first use: a real program fixes its environment at
//! start, and since edition 2024 changing it at run time takes `unsafe`. The rule itself is the
//! pure, private `includes_location`, tested without touching the environment.
//!
//! Revision History
//! - 2026-11-17T16:20:28Z @AI: HEXSER-0.7 D9 — read the switch once per process (`OnceLock`) instead of on every serialized error; split the rule into pure `includes_location` and `skip_location_when` so the tests need no `unsafe` env writes (13 removed) and no `serial_test` lock.
//! - 2026-07-20T00:00:00Z @AI: Serialize env-mutating tests with #[serial(hexser_env)]; fix rustdoc (backtick generics, text code block); add /// why on tests.
//! - 2025-10-09T21:51:00Z @AI: Initial implementation for conditional source location serialization.

/// Checks if source location information should be included in serialization.
///
/// Returns true if the HEXSER_INCLUDE_SOURCE_LOCATION environment variable
/// is set to "1" or "true" (case-insensitive). Returns false otherwise.
///
/// The variable is read once per process, at the first call; a later change to the
/// environment is not seen.
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
  static INCLUDE: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
  *INCLUDE.get_or_init(|| {
    includes_location(
      std::env::var("HEXSER_INCLUDE_SOURCE_LOCATION")
        .ok()
        .as_deref(),
    )
  })
}

/// The switch's rule: on for `"1"` or `"true"` in any letter case, off for anything else and
/// when unset. Pure, so it is tested without touching the process environment.
fn includes_location(value: std::option::Option<&str>) -> bool {
  match value {
    std::option::Option::Some(v) => v == "1" || v.eq_ignore_ascii_case("true"),
    std::option::Option::None => false,
  }
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
  skip_location_when(location, should_include_source_location())
}

/// Skip a location that is absent, or present while the switch is off. Pure.
#[cfg(feature = "serde")]
fn skip_location_when(
  location: &std::option::Option<crate::error::source_location::SourceLocation>,
  include: bool,
) -> bool {
  location.is_none() || !include
}

#[cfg(test)]
mod tests {
  // These tests read no process environment and write none: the switch's rule is a pure
  // function of the variable's value, so it is tested as one. The end-to-end reading of the
  // real environment is tests/env_controlled_serde_test.rs, which re-runs its own binary with
  // the variable fixed at process start.

  /// why: "1" and "true" in any letter case are the documented opt-ins; each must turn the
  /// switch on.
  #[test]
  fn the_switch_is_on_only_for_1_or_true_in_any_case() {
    for value in ["1", "true", "TRUE", "True", "tRuE"] {
      std::assert!(
        super::includes_location(std::option::Option::Some(value)),
        "{value:?} must turn the switch on"
      );
    }
  }

  /// why: unset, empty and every other value keep source locations out of serialized errors
  /// (the privacy-safe default), so a typo or padding never leaks a file path.
  #[test]
  fn the_switch_is_off_when_unset_or_for_any_other_value() {
    std::assert!(!super::includes_location(std::option::Option::None));
    for value in ["", "0", "false", "yes", "on", " 1", "1 ", "truee"] {
      std::assert!(
        !super::includes_location(std::option::Option::Some(value)),
        "{value:?} must leave the switch off"
      );
    }
  }

  /// why: an error with no location has nothing to serialize, whatever the switch says.
  #[test]
  #[cfg(feature = "serde")]
  fn a_missing_location_is_always_skipped() {
    std::assert!(super::skip_location_when(&std::option::Option::None, true));
    std::assert!(super::skip_location_when(&std::option::Option::None, false));
  }

  /// why: a present location is serialized only when the switch is on, so serialized errors
  /// do not leak file paths by default.
  #[test]
  #[cfg(feature = "serde")]
  fn a_present_location_is_skipped_unless_the_switch_is_on() {
    let location = std::option::Option::Some(crate::error::source_location::SourceLocation::new(
      "test.rs", 1, 1,
    ));
    std::assert!(super::skip_location_when(&location, false));
    std::assert!(!super::skip_location_when(&location, true));
  }
}
