//! Repository trait for persistence abstractions.
//!
//! Repositories provide a collection-like interface for accessing domain entities.
//! They abstract the underlying persistence mechanism, allowing the domain layer
//! to remain independent of infrastructure concerns. Repositories work with
//! aggregates and typically provide CRUD operations plus domain-specific queries.
//!
//! Revision History
//! - 2025-10-01T00:00:00Z @AI: Initial Repository trait definition with generic entity type.

/// Trait for repository ports that abstract persistence operations.
///
/// Repositories provide access to entities as if they were in-memory collections,
/// hiding the complexity of the underlying storage mechanism.
///
/// # Type Parameters
///
/// * `T` - The entity type this repository manages (must implement `Entity`)
///
/// # Example
///
/// ```rust
/// use hex::ports::Repository;
/// use hex::domain::Entity;
/// use hex::HexResult;
///
/// struct User {
///     id: String,
/// }
///
/// impl Entity for User {
///     type Id = String;
/// }
///
/// trait UserRepository: Repository<User> {
///     fn find_by_email(&self, email: &str) -> HexResult<Option<User>>;
/// }
/// ```
pub trait Repository<T>
where
    T: crate::domain::entity::Entity,
{
    /// Find an entity by its unique identifier.
    fn find_by_id(&self, id: &T::Id) -> crate::result::hex_result::HexResult<Option<T>>;

    /// Save an entity to the repository.
    fn save(&mut self, entity: T) -> crate::result::hex_result::HexResult<()>;

    /// Delete an entity by its identifier.
    fn delete(&mut self, id: &T::Id) -> crate::result::hex_result::HexResult<()>;

    /// Find all entities in the repository.
    fn find_all(&self) -> crate::result::hex_result::HexResult<Vec<T>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestEntity {
        id: u64,
        name: String,
    }

    impl crate::domain::entity::Entity for TestEntity {
        type Id = u64;
    }

    struct TestRepository {
        entities: Vec<TestEntity>,
    }

    impl Repository<TestEntity> for TestRepository {
        fn find_by_id(&self, id: &u64) -> crate::result::hex_result::HexResult<Option<TestEntity>> {
            let found = self.entities.iter().find(|e| e.id == *id);
            Result::Ok(found.map(|e| TestEntity {
                id: e.id,
                name: e.name.clone(),
            }))
        }

        fn save(&mut self, entity: TestEntity) -> crate::result::hex_result::HexResult<()> {
            self.entities.push(entity);
            Result::Ok(())
        }

        fn delete(&mut self, id: &u64) -> crate::result::hex_result::HexResult<()> {
            self.entities.retain(|e| e.id != *id);
            Result::Ok(())
        }

        fn find_all(&self) -> crate::result::hex_result::HexResult<Vec<TestEntity>> {
            let all: Vec<TestEntity> = self.entities.iter().map(|e| TestEntity {
                id: e.id,
                name: e.name.clone(),
            }).collect();
            Result::Ok(all)
        }
    }

    #[test]
    fn test_repository_save_and_find() {
        let mut repo = TestRepository { entities: Vec::new() };
        let entity = TestEntity { id: 1, name: String::from("Test") };
        repo.save(entity).unwrap();
        let found = repo.find_by_id(&1).unwrap();
        assert!(found.is_some());
    }
}
