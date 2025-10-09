//! Repository trait for persistence abstractions.
//!
//! Repositories provide a collection-like interface for accessing domain entities.
//! They abstract the underlying persistence mechanism, allowing the domain layer
//! to remain independent of infrastructure concerns. Repositories work with
//! aggregates and typically provide CRUD operations plus domain-specific queries.
//!
//! Revision History
//! - 2025-10-01T00:00:00Z @AI: Initial Repository trait definition with generic entity type.
//! - 2025-10-06T00:00:00Z @AI: Introduced filter-based generic query API (separate QueryRepository trait), sorting and pagination.
//! - 2025-10-06T17:22:00Z @AI: Tests: add justifications; remove super import; fully qualify paths per no-use rule.
//! - 2025-10-07T10:00:00Z @AI: Decouple QueryRepository from ID-centric Repository to enable generic, filter-first repositories.
//! - 2025-10-07T10:59:00Z @AI: Remove deprecated id-centric methods; focus Repository on save only; update tests for v0.4.

/// Generic query options for fetching collections.
#[derive(Debug, Clone)]
pub struct FindOptions<K> {
  pub sort: Option<Vec<Sort<K>>>,
  pub limit: Option<u32>,
  pub offset: Option<u64>,
}

impl<K> Default for FindOptions<K> {
  fn default() -> Self {
    Self {
      sort: None,
      limit: None,
      offset: None,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
  Asc,
  Desc,
}

#[derive(Debug, Clone)]
pub struct Sort<K> {
  pub key: K,
  pub direction: Direction,
}

/// Trait for repository ports that abstract persistence save operations (v0.4+).
///
/// Starting in v0.4, id-centric methods were removed in favor of the generic,
/// filter-based `QueryRepository` API. This trait now focuses solely on the
/// write-side persistence concern of saving aggregates. For read operations and
/// deletions by criteria, implement `QueryRepository` on the same adapter.
///
/// # Type Parameters
///
/// * `T` - The entity type this repository manages (must implement `Entity`)
pub trait Repository<T>
where
  T: crate::domain::entity::Entity,
{
  /// Save an entity to the repository.
  fn save(&mut self, entity: T) -> crate::result::hex_result::HexResult<()>;
}

/// Generic query-capable repository port for expressive, domain-owned filters.
pub trait QueryRepository<T>
where
  T: crate::domain::entity::Entity,
{
  /// Domain-owned filter type the adapter understands.
  type Filter;

  /// Domain-owned sort key type (e.g., enum of sortable fields).
  type SortKey;

  /// Fetch a single entity matching a filter (ideally unique).
  fn find_one(&self, filter: &Self::Filter) -> crate::result::hex_result::HexResult<Option<T>>;

  /// Fetch many entities matching `filter` with optional sort/pagination.
  fn find(
    &self,
    filter: &Self::Filter,
    options: FindOptions<Self::SortKey>,
  ) -> crate::result::hex_result::HexResult<Vec<T>>;

  /// Check existence of at least one entity matching `filter`.
  fn exists(&self, filter: &Self::Filter) -> crate::result::hex_result::HexResult<bool> {
    Ok(self.find_one(filter)?.is_some())
  }

  /// Count entities matching `filter`.
  fn count(&self, filter: &Self::Filter) -> crate::result::hex_result::HexResult<u64> {
    Ok(self.find(filter, FindOptions::default())?.len() as u64)
  }

  /// Delete by filter; returns number of removed entities.
  fn delete_where(&mut self, _filter: &Self::Filter) -> crate::result::hex_result::HexResult<u64> {
    // Default no-op for backward compatibility in simple adapters.
    Ok(0)
  }
}

#[cfg(test)]
mod tests {
  // Note: Per NO `use` STATEMENTS rule, tests reference items via fully qualified paths.
  // This ensures clarity for multi-agent analysis and avoids ambiguous imports.

  #[derive(Clone, Debug)]
  struct TestEntity {
    id: u64,
    name: String,
  }

  impl crate::domain::entity::Entity for TestEntity {
    type Id = u64;
  }

  #[derive(Clone, Debug)]
  enum TestFilter {
    ById(u64),
    NameEquals(String),
    All,
    And(Vec<TestFilter>),
  }

  #[derive(Clone, Copy, Debug, PartialEq, Eq)]
  enum TestSortKey {
    Id,
    Name,
  }

  #[derive(Default)]
  struct TestRepository {
    entities: Vec<TestEntity>,
  }

  impl crate::ports::repository::Repository<TestEntity> for TestRepository {
    fn save(&mut self, entity: TestEntity) -> crate::result::hex_result::HexResult<()> {
      if let Some(i) = self.entities.iter().position(|e| e.id == entity.id) {
        self.entities[i] = entity;
      } else {
        self.entities.push(entity);
      }
      Ok(())
    }
  }

  impl crate::ports::repository::QueryRepository<TestEntity> for TestRepository {
    type Filter = TestFilter;
    type SortKey = TestSortKey;

    fn find_one(
      &self,
      filter: &Self::Filter,
    ) -> crate::result::hex_result::HexResult<Option<TestEntity>> {
      Ok(
        self
          .entities
          .iter()
          .find(|e| matches_filter(e, filter))
          .cloned(),
      )
    }

    fn find(
      &self,
      filter: &Self::Filter,
      options: crate::ports::repository::FindOptions<Self::SortKey>,
    ) -> crate::result::hex_result::HexResult<Vec<TestEntity>> {
      let mut items: Vec<_> = self
        .entities
        .iter()
        .filter(|e| matches_filter(e, filter))
        .cloned()
        .collect();

      if let Some(sorts) = options.sort {
        for s in sorts.into_iter().rev() {
          match (s.key, s.direction) {
            (TestSortKey::Id, crate::ports::repository::Direction::Asc) => {
              items.sort_by_key(|e| e.id)
            }
            (TestSortKey::Id, crate::ports::repository::Direction::Desc) => {
              items.sort_by_key(|e| std::cmp::Reverse(e.id))
            }
            (TestSortKey::Name, crate::ports::repository::Direction::Asc) => {
              items.sort_by(|a, b| a.name.cmp(&b.name))
            }
            (TestSortKey::Name, crate::ports::repository::Direction::Desc) => {
              items.sort_by(|a, b| b.name.cmp(&a.name))
            }
          }
        }
      }

      let offset = options.offset.unwrap_or(0) as usize;
      let limit = options
        .limit
        .map(|l| l as usize)
        .unwrap_or_else(|| items.len().saturating_sub(offset));
      let end = offset.saturating_add(limit).min(items.len());
      Ok(
        items
          .into_iter()
          .skip(offset)
          .take(end.saturating_sub(offset))
          .collect(),
      )
    }

    fn delete_where(&mut self, filter: &Self::Filter) -> crate::result::hex_result::HexResult<u64> {
      let before = self.entities.len();
      self.entities.retain(|e| !matches_filter(e, filter));
      Ok((before - self.entities.len()) as u64)
    }
  }

  fn matches_filter(e: &TestEntity, f: &TestFilter) -> bool {
    match f {
      TestFilter::ById(id) => e.id == *id,
      TestFilter::NameEquals(n) => &e.name == n,
      TestFilter::All => true,
      TestFilter::And(fs) => fs.iter().all(|x| matches_filter(e, x)),
    }
  }

  #[test]
  fn test_repository_save_and_find_new_api() {
    // Test: Validates new QueryRepository API (find_one/find with sorting & pagination) and legacy compatibility.
    // Justification: Ensures migration path is safe; verifies filter matching, stable sorting, and paging behavior.
    let mut repo = TestRepository {
      entities: Vec::new(),
    };
    <TestRepository as crate::ports::repository::Repository<TestEntity>>::save(
      &mut repo,
      TestEntity {
        id: 2,
        name: String::from("B"),
      },
    )
    .unwrap();
    <TestRepository as crate::ports::repository::Repository<TestEntity>>::save(
      &mut repo,
      TestEntity {
        id: 1,
        name: String::from("A"),
      },
    )
    .unwrap();

    // find_one by filter
    let found =
      <TestRepository as crate::ports::repository::QueryRepository<TestEntity>>::find_one(
        &repo,
        &TestFilter::ById(1),
      )
      .unwrap();
    assert!(found.is_some());

    // find with sort and pagination
    let opts = crate::ports::repository::FindOptions {
      sort: Some(vec![crate::ports::repository::Sort {
        key: TestSortKey::Name,
        direction: crate::ports::repository::Direction::Asc,
      }]),
      limit: Some(1),
      offset: Some(0),
    };
    let page = <TestRepository as crate::ports::repository::QueryRepository<TestEntity>>::find(
      &repo,
      &TestFilter::All,
      opts,
    )
    .unwrap();
    assert_eq!(page.len(), 1);
    assert_eq!(page[0].name, "A");

    // delete by filter and re-check via QueryRepository
    let removed =
      <TestRepository as crate::ports::repository::QueryRepository<TestEntity>>::delete_where(
        &mut repo,
        &TestFilter::ById(2),
      )
      .unwrap();
    assert_eq!(removed, 1);
    let none = <TestRepository as crate::ports::repository::QueryRepository<TestEntity>>::find_one(
      &repo,
      &TestFilter::ById(2),
    )
    .unwrap();
    assert!(none.is_none());
  }
}
