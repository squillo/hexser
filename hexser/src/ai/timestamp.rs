//! Dependency-free UTC RFC3339 timestamp formatting.
//!
//! The AI context embeds a `generated_at` timestamp. That is the crate's only need for a date
//! library, so rather than pull `chrono` (and its `iana-time-zone`/`num-traits` transitive
//! tree) into every `ai`/`mcp` build for one string, we format RFC3339 directly from
//! `std::time::SystemTime` using Howard Hinnant's constant-time days→civil-date algorithm.
//!
//! Revision History
//! - 2026-09-11T00:00:00Z @AI: Read the clock through `crate::clock` so `ai`/`mcp` builds stop trapping on wasm32-unknown-unknown, where `SystemTime::now()` panics.
//! - 2026-07-20T00:00:00Z @AI: Add std-only RFC3339 formatter to remove the chrono dependency.

/// Returns the current UTC time as an RFC3339 / ISO-8601 string with second precision, e.g.
/// `2026-07-20T13:37:00Z`.
///
/// If the system clock is set before the Unix epoch — or the target has no clock at all, as on
/// `wasm32-unknown-unknown` — this returns the epoch itself rather than panicking (a
/// best-effort metadata timestamp is always acceptable).
pub fn now_rfc3339() -> std::string::String {
  format_rfc3339_utc(crate::clock::unix_secs())
}

/// Formats a count of seconds since the Unix epoch as a UTC RFC3339 string.
fn format_rfc3339_utc(secs: u64) -> std::string::String {
  let days = (secs / 86_400) as i64;
  let time_of_day = secs % 86_400;
  let hour = time_of_day / 3_600;
  let minute = (time_of_day % 3_600) / 60;
  let second = time_of_day % 60;
  let (year, month, day) = civil_from_days(days);
  std::format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z")
}

/// Converts a count of days since 1970-01-01 into a `(year, month, day)` civil date.
///
/// Howard Hinnant's public-domain algorithm (`http://howardhinnant.github.io/date_algorithms.html`),
/// valid for the full range of proleptic Gregorian dates.
fn civil_from_days(z: i64) -> (i64, u32, u32) {
  let z = z + 719_468;
  let era = (if z >= 0 { z } else { z - 146_096 }) / 146_097;
  let doe = (z - era * 146_097) as u64; // [0, 146096]
  let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365; // [0, 399]
  let year = yoe as i64 + era * 400;
  let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // [0, 365]
  let mp = (5 * doy + 2) / 153; // [0, 11]
  let day = (doy - (153 * mp + 2) / 5 + 1) as u32; // [1, 31]
  let month = if mp < 10 { mp + 3 } else { mp - 9 } as u32; // [1, 12]
  let year = if month <= 2 { year + 1 } else { year };
  (year, month, day)
}

#[cfg(test)]
mod tests {
  use super::*;

  /// why: the Unix epoch must format to the canonical RFC3339 string; anchors the civil-date
  /// math at day 0.
  #[test]
  fn test_epoch_formats_canonically() {
    std::assert_eq!(format_rfc3339_utc(0), "1970-01-01T00:00:00Z");
  }

  /// why: a well-known non-trivial epoch second (1_700_000_000 = 2023-11-14T22:13:20Z) must
  /// format correctly, exercising year/month/day + time-of-day together across many years and
  /// leap years since 1970.
  #[test]
  fn test_known_timestamp_formats_correctly() {
    std::assert_eq!(format_rfc3339_utc(1_700_000_000), "2023-11-14T22:13:20Z");
  }

  /// why: a leap-day boundary (2020-02-29) must be handled, guarding the leap-year branch of
  /// the algorithm. 1582934400 = 2020-02-29T00:00:00Z.
  #[test]
  fn test_leap_day_formats_correctly() {
    std::assert_eq!(format_rfc3339_utc(1_582_934_400), "2020-02-29T00:00:00Z");
  }

  /// why: the live clock must produce a syntactically valid RFC3339 UTC string (20xx year,
  /// trailing Z) so downstream consumers can parse `generated_at`.
  #[test]
  fn test_now_is_well_formed() {
    let ts = now_rfc3339();
    std::assert!(ts.ends_with('Z'), "must be UTC (Z): {ts}");
    std::assert!(ts.starts_with("20"), "expected a 21st-century year: {ts}");
    std::assert_eq!(ts.len(), 20, "YYYY-MM-DDTHH:MM:SSZ is 20 chars: {ts}");
    // The date and time separators must be where RFC3339 requires them.
    std::assert_eq!(&ts[4..5], "-");
    std::assert_eq!(&ts[10..11], "T");
    std::assert_eq!(&ts[19..20], "Z");
  }
}
