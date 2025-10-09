//! Mapper trait for data transformation between layers.
//!
//! Mappers transform data between different representations as it crosses
//! architectural boundaries. They convert between domain models, DTOs,
//! database models, or API representations, ensuring clean separation
//! between layers while maintaining data integrity.
//!
//! Revision History
//! - 2025-10-01T00:00:00Z @AI: Initial Mapper trait definition for bidirectional transformation.

/// Trait for mapping data between different representations.
///
/// Mappers transform data as it crosses architectural boundaries,
/// converting between domain models and external representations.
///
/// # Type Parameters
///
/// * `From` - The source data type
/// * `To` - The target data type
///
/// # Example
///
/// ```rust
/// use hexser::adapters::Mapper;
/// use hexser::HexResult;
///
/// struct DomainUser {
///     id: String,
///     email: String,
/// }
///
/// struct DbUserRow {
///     user_id: String,
///     user_email: String,
/// }
///
/// struct UserMapper;
///
/// impl Mapper<DomainUser, DbUserRow> for UserMapper {
///     fn map(&self, from: DomainUser) -> HexResult<DbUserRow> {
///         Ok(DbUserRow {
///             user_id: from.id,
///             user_email: from.email,
///         })
///     }
/// }
///
/// impl Mapper<DbUserRow, DomainUser> for UserMapper {
///     fn map(&self, from: DbUserRow) -> HexResult<DomainUser> {
///         Ok(DomainUser {
///             id: from.user_id,
///             email: from.user_email,
///         })
///     }
/// }
/// ```
pub trait Mapper<From, To> {
  /// Map from one representation to another.
  ///
  /// Returns the mapped value if successful, or an error if the mapping fails.
  fn map(&self, from: From) -> crate::result::hex_result::HexResult<To>;
}

#[cfg(test)]
mod tests {
  use super::*;

  struct SourceType {
    value: i32,
  }

  struct TargetType {
    data: i32,
  }

  struct TestMapper;

  impl Mapper<SourceType, TargetType> for TestMapper {
    fn map(&self, from: SourceType) -> crate::result::hex_result::HexResult<TargetType> {
      Result::Ok(TargetType { data: from.value })
    }
  }

  #[test]
  fn test_mapper_transformation() {
    let mapper = TestMapper;
    let source = SourceType { value: 42 };
    let target = mapper.map(source).unwrap();
    assert_eq!(target.data, 42);
  }
}
