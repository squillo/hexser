//! QueryHandler trait for executing queries.
//!
//! Query handlers execute read-only operations, retrieving data without
//! modifying state. They support the Query side of CQRS and can be optimized
//! independently from command handlers, potentially using different data stores
//! or denormalized views for better read performance.
//!
//! Revision History
//! - 2025-10-01T00:00:00Z @AI: Initial QueryHandler trait definition for query execution.

/// Trait for handlers that execute queries.
///
/// Query handlers retrieve data without side effects, supporting read
/// operations in the CQRS pattern.
///
/// # Type Parameters
///
/// * `Q` - The query type this handler processes
/// * `R` - The result type returned by the query
///
/// # Example
///
/// ```rust
/// use hexser::application::QueryHandler;
/// use hexser::HexResult;
///
/// struct FindUserQuery {
///     user_id: String,
/// }
///
/// struct UserView {
///     id: String,
///     email: String,
/// }
///
/// struct FindUserHandler;
///
/// impl QueryHandler<FindUserQuery, Option<UserView>> for FindUserHandler {
///     fn handle(&self, query: FindUserQuery) -> HexResult<Option<UserView>> {
///         // Execute the query
///         Ok(None)
///     }
/// }
/// ```
pub trait QueryHandler<Q, R> {
  /// Handle the execution of a query.
  ///
  /// Returns the query result if successful, or an error if the query fails.
  fn handle(&self, query: Q) -> crate::result::hex_result::HexResult<R>;
}

#[cfg(test)]
mod tests {
  use super::*;

  struct TestQuery {
    id: u64,
  }

  struct TestResult {
    data: String,
  }

  struct TestQueryHandler;

  impl QueryHandler<TestQuery, Option<TestResult>> for TestQueryHandler {
    fn handle(&self, query: TestQuery) -> crate::result::hex_result::HexResult<Option<TestResult>> {
      if query.id == 1 {
        Result::Ok(Some(TestResult {
          data: String::from("Found"),
        }))
      } else {
        Result::Ok(None)
      }
    }
  }

  #[test]
  fn test_query_handler_found() {
    let handler = TestQueryHandler;
    let query = TestQuery { id: 1 };
    let result = handler.handle(query).unwrap();
    assert!(result.is_some());
  }

  #[test]
  fn test_query_handler_not_found() {
    let handler = TestQueryHandler;
    let query = TestQuery { id: 999 };
    let result = handler.handle(query).unwrap();
    assert!(result.is_none());
  }
}
