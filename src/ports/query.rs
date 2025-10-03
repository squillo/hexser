//! Query trait for read-only operations (CQRS pattern).
//!
//! Queries represent read-only operations that retrieve data without causing
//! side effects. This trait supports Command Query Responsibility Segregation (CQRS)
//! by clearly separating read operations from write operations (commands).
//! Queries should be optimized for reading and may use different data models.
//!
//! Revision History
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
/// use hexer::ports::Query;
/// use hexer::HexResult;
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
pub trait Query<Params, Result> {
    /// Execute the query with the given parameters.
    ///
    /// Returns the query result if successful, or an error if the query fails.
    fn query(&self, params: Params) -> crate::result::hex_result::HexResult<Result>;
}

#[cfg(test)]
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
        fn query(&self, params: TestQueryParams) -> crate::result::hex_result::HexResult<Option<TestQueryResult>> {
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
