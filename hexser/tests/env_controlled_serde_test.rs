//! Integration tests for environment variable controlled serialization.
//!
//! Tests verify that source location information in error types is
//! conditionally serialized based on the HEXSER_INCLUDE_SOURCE_LOCATION
//! environment variable. This is a security feature to prevent exposing
//! internal code structure to clients in production environments.
//!
//! No test here writes the process environment. A test that needs the switch in a given state
//! re-runs this binary in a child process whose environment fixes the switch at start, which is
//! the only way a real program sets it. That removes the `unsafe` `set_var`/`remove_var` calls
//! and the cross-test lock they needed.
//!
//! Revision History
//! - 2026-11-17T16:20:28Z @AI: HEXSER-0.7 D9 — drop all 15 `unsafe` env writes and the
//!   `serial_test` lock; the switch-dependent cases run as one `#[ignore]`d child test that
//!   five parent tests re-run with the switch fixed at process start (unset, "false", "1",
//!   "true", "TRUE"), covering all four location-bearing error types per run.
//! - 2026-07-20T00:00:00Z @AI: Serialize all env-mutating tests with #[serial(hexser_env)] to fix a parallel-thread env race; add /// why on each test.
//! - 2025-10-09T21:51:00Z @AI: Initial test for conditional source location serialization.

#[cfg(feature = "serde")]
mod serde_tests {
  /// The switch under test.
  const SWITCH: &str = "HEXSER_INCLUDE_SOURCE_LOCATION";
  /// Set by a parent to tell the child which polarity to assert: `carries` or `omits`.
  const EXPECT: &str = "HEXSER_ENV_TEST_EXPECT";
  /// The child test's full name inside this binary.
  const CHILD: &str = "serde_tests::child_every_error_serializes_its_location_as_expected";

  /// Re-runs the one child test of this binary in a fresh process whose environment fixes the
  /// switch at start (`Some` sets it, `None` removes it) and tells the child what to expect.
  /// Fails unless exactly that one test ran and passed.
  fn run_child(switch: std::option::Option<&str>, expect: &str) {
    let exe = std::env::current_exe().expect("the test binary knows its own path");
    let mut command = std::process::Command::new(exe);
    command.args(["--ignored", "--exact", "--test-threads=1", CHILD]);
    command.env(EXPECT, expect);
    match switch {
      std::option::Option::Some(value) => {
        command.env(SWITCH, value);
      }
      std::option::Option::None => {
        command.env_remove(SWITCH);
      }
    }
    let output = command
      .output()
      .expect("the test binary re-runs as a child");
    let stdout = std::string::String::from_utf8_lossy(&output.stdout);
    let stderr = std::string::String::from_utf8_lossy(&output.stderr);
    std::assert!(
      output.status.success(),
      "child with {SWITCH}={switch:?} expecting {expect} failed:\n{stdout}\n{stderr}"
    );
    std::assert!(
      stdout.contains("test result: ok. 1 passed;"),
      "child with {SWITCH}={switch:?} did not run exactly one test:\n{stdout}"
    );
  }

  /// The four location-bearing error types, each serialized with a distinct file and line.
  fn four_errors_with_locations() -> std::vec::Vec<(&'static str, &'static str, std::string::String)>
  {
    // DomainError's `with_location` is the RichError trait's, named in full (no `use`).
    let domain = <hexser::error::domain_error::DomainError as hexser::error::rich_error::RichError>::with_location(
      hexser::error::domain_error::DomainError::new("E_DOM_001", "Test error"),
      hexser::error::source_location::SourceLocation::new("src/domain.rs", 100, 5),
    );
    let validation =
      hexser::error::validation_error::ValidationError::new("E_VAL_001", "Invalid input")
        .with_field("email")
        .with_location(hexser::error::source_location::SourceLocation::new(
          "src/validation.rs",
          51,
          1,
        ));
    let not_found =
      hexser::error::not_found_error::NotFoundError::new("User", "user-123").with_location(
        hexser::error::source_location::SourceLocation::new("src/repo.rs", 200, 15),
      );
    let conflict = hexser::error::conflict_error::ConflictError::new("Email already exists")
      .with_existing_id("user-456")
      .with_location(hexser::error::source_location::SourceLocation::new(
        "src/service.rs",
        75,
        8,
      ));
    std::vec![
      (
        "src/domain.rs",
        "100",
        serde_json::to_string(&domain).expect("serializes")
      ),
      (
        "src/validation.rs",
        "51",
        serde_json::to_string(&validation).expect("serializes")
      ),
      (
        "src/repo.rs",
        "200",
        serde_json::to_string(&not_found).expect("serializes")
      ),
      (
        "src/service.rs",
        "75",
        serde_json::to_string(&conflict).expect("serializes")
      ),
    ]
  }

  /// why: the one place the real process environment is read end to end. It runs only inside
  /// a child that its parent started with the switch fixed, and asserts the parent's polarity
  /// for all four location-bearing error types.
  #[test]
  #[ignore = "runs only as a child of the parent tests below, which fix the switch at process start"]
  fn child_every_error_serializes_its_location_as_expected() {
    let expect = std::env::var(EXPECT).unwrap_or_default();
    std::assert!(
      expect == "carries" || expect == "omits",
      "run through a parent test (it sets {EXPECT}); got {expect:?}"
    );
    for (file, line, json) in four_errors_with_locations() {
      let carries = json.contains(file) && json.contains(line);
      let mentions = json.contains(file) || json.contains("\"location\"");
      if expect == "carries" {
        std::assert!(carries, "the location must be serialized. JSON: {json}");
      } else {
        std::assert!(
          !mentions,
          "the location must not be serialized. JSON: {json}"
        );
      }
    }
  }

  /// why: with the switch unset, no error leaks its source path or line (the production
  /// default).
  #[test]
  fn every_error_omits_its_location_when_the_switch_is_unset() {
    run_child(std::option::Option::None, "omits");
  }

  /// why: a value other than the opt-ins must not switch it on; the must-fail arm against a
  /// reader that accepts any non-empty value.
  #[test]
  fn every_error_omits_its_location_when_the_switch_is_false() {
    run_child(std::option::Option::Some("false"), "omits");
  }

  /// why: "1" is a documented opt-in, and every error then carries its location.
  #[test]
  fn every_error_carries_its_location_when_the_switch_is_1() {
    run_child(std::option::Option::Some("1"), "carries");
  }

  /// why: "true" is a documented opt-in.
  #[test]
  fn every_error_carries_its_location_when_the_switch_is_true() {
    run_child(std::option::Option::Some("true"), "carries");
  }

  /// why: the opt-in is case-insensitive end to end, through serialization.
  #[test]
  fn every_error_carries_its_location_when_the_switch_is_upper_case_true() {
    run_child(std::option::Option::Some("TRUE"), "carries");
  }

  /// why: the raw SourceLocation struct always serializes its own fields; the switch governs
  /// only whether *errors* embed a location, not the struct's own Serialize impl.
  #[test]
  fn a_source_location_itself_always_serializes_its_fields() {
    let loc = hexser::error::source_location::SourceLocation::new("src/main.rs", 42, 10);
    let json = serde_json::to_string(&loc).expect("serialization should succeed");
    std::assert!(
      json.contains("src/main.rs") && json.contains("42") && json.contains("10"),
      "SourceLocation struct itself always serializes its fields"
    );
  }

  /// why: an error carrying no location still serializes its code and message normally (the
  /// skip logic must not drop unrelated fields), whatever the switch says.
  #[test]
  fn an_error_without_a_location_serializes_its_code_and_message() {
    let err = hexser::error::domain_error::DomainError::new("E_DOM_002", "Error without location");
    let json = serde_json::to_string(&err).expect("serialization should succeed");
    std::assert!(
      json.contains("E_DOM_002") && json.contains("Error without location"),
      "Error without location should serialize normally. JSON: {json}"
    );
  }

  /// why: deserialization accepts payloads with and without a `location` field, independent
  /// of the switch, so errors serialized under either setting round-trip.
  #[test]
  fn deserialization_accepts_a_payload_with_or_without_a_location() {
    let json_with_location = r#"{"code":"E_DOM_001","message":"Test","next_steps":[],"suggestions":[],"location":{"file":"test.rs","line":1,"column":1},"more_info_url":null,"layer":null}"#;
    let json_without_location = r#"{"code":"E_DOM_001","message":"Test","next_steps":[],"suggestions":[],"more_info_url":null,"layer":null}"#;

    let result1: std::result::Result<hexser::error::domain_error::DomainError, _> =
      serde_json::from_str(json_with_location);
    let result2: std::result::Result<hexser::error::domain_error::DomainError, _> =
      serde_json::from_str(json_without_location);

    std::assert!(result1.is_ok(), "Should deserialize with location");
    std::assert!(result2.is_ok(), "Should deserialize without location");
  }
}

#[cfg(not(feature = "serde"))]
fn main() {
  // Test harness stub for the no-serde configuration; stdout is acceptable here.
  #[allow(clippy::disallowed_macros)]
  {
    println!("Serde tests require the 'serde' feature to be enabled.");
  }
}
