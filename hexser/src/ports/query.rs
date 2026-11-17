//! Query trait for read-only operations (CQRS pattern).
//!
//! Queries represent read-only operations that retrieve data without causing
//! side effects. This trait supports Command Query Responsibility Segregation (CQRS)
//! by clearly separating read operations from write operations (commands).
//! Queries should be optimized for reading and may use different data models.
//!
//! Revision History
//! - 2026-11-17T16:31:10Z @AI: HEXSER-0.7 D7 — `#[deprecated]`, routed to `QueryHandler` (two read doors with one shape, 0 impls in the family). Unit tests keep exercising it under `#[allow(deprecated)]`.
//! - 2025-10-01T00:00:00Z @AI: Initial Query trait definition for CQRS read operations.

/// Trait for queries that perform read-only operations.
///
/// Queries retrieve data without modifying state, supporting the Query side
/// of CQRS. They can be optimized independently from commands.
///
/// # Type Parameters
///
/// * `Params` - The parameters needed to execute the query
/// * `Result` - The type of data returned by the query
///
/// # Example
///
/// ```rust
/// use hexser::ports::Query;
/// use hexser::HexResult;
///
/// struct FindUserByEmailParams {
///     email: String,
/// }
///
/// struct UserView {
///     id: String,
///     email: String,
///     name: String,
/// }
///
/// trait FindUserByEmail: Query<FindUserByEmailParams, Option<UserView>> {}
/// ```
#[deprecated(
  since = "0.7.0",
  note = "use `hexser::application::QueryHandler`, which has the same shape. See MIGRATING_0.6_TO_0.7.md; removal is left to a later version."
)]
pub trait Query<Params, Result> {
  /// Execute the query with the given parameters.
  ///
  /// Returns the query result if successful, or an error if the query fails.
  fn query(&self, params: Params) -> crate::result::hex_result::HexResult<Result>;
}

#[cfg(test)]
#[allow(deprecated)]
mod tests {
  use super::*;

  struct TestQueryParams {
    id: u64,
  }

  struct TestQueryResult {
    name: String,
  }

  struct TestQuery;

  impl Query<TestQueryParams, Option<TestQueryResult>> for TestQuery {
    fn query(
      &self,
      params: TestQueryParams,
    ) -> crate::result::hex_result::HexResult<Option<TestQueryResult>> {
      if params.id == 1 {
        Result::Ok(Some(TestQueryResult {
          name: String::from("Found"),
        }))
      } else {
        Result::Ok(None)
      }
    }
  }

  #[test]
  fn test_query_found() {
    let query = TestQuery;
    let params = TestQueryParams { id: 1 };
    let result = query.query(params).unwrap();
    assert!(result.is_some());
  }

  #[test]
  fn test_query_not_found() {
    let query = TestQuery;
    let params = TestQueryParams { id: 999 };
    let result = query.query(params).unwrap();
    assert!(result.is_none());
  }
}
