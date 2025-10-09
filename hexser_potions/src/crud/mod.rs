//! CRUD potions: a minimal in-memory repository for a single entity type.
//!
//! Demonstrates implementing the `Repository<T>` trait with a simple adapter
//! and using it from application code.
//!
//! Revision History
//! - 2025-10-07T11:57:00Z @AI: Migrate to v0.4 Repository/QueryRepository; remove id-centric methods; update API usage.

use hexser::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Item {
  pub id: u64,
  pub name: String,
}

impl HexEntity for Item {
  type Id = u64;
}

pub trait ItemRepository: Repository<Item> {}

#[derive(Default)]
pub struct InMemoryItemRepository {
  pub items: Vec<Item>,
}

impl Repository<Item> for InMemoryItemRepository {
  fn save(&mut self, entity: Item) -> HexResult<()> {
    if let Some(existing) = self.items.iter_mut().find(|e| e.id == entity.id) {
      *existing = entity;
    } else {
      self.items.push(entity);
    }
    Ok(())
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ItemFilter {
  All,
  ById(u64),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItemSortKey {
  Id,
}

impl hexser::ports::repository::QueryRepository<Item> for InMemoryItemRepository {
  type Filter = ItemFilter;
  type SortKey = ItemSortKey;

  fn find_one(&self, filter: &ItemFilter) -> HexResult<Option<Item>> {
    let found = match filter {
      ItemFilter::All => self.items.first().cloned(),
      ItemFilter::ById(id) => self.items.iter().find(|e| e.id == *id).cloned(),
    };
    Ok(found)
  }

  fn find(
    &self,
    filter: &ItemFilter,
    _opts: hexser::ports::repository::FindOptions<ItemSortKey>,
  ) -> HexResult<Vec<Item>> {
    let items: Vec<Item> = match filter {
      ItemFilter::All => self.items.clone(),
      ItemFilter::ById(id) => self.items.iter().filter(|e| e.id == *id).cloned().collect(),
    };
    Ok(items)
  }

  fn delete_where(&mut self, filter: &ItemFilter) -> HexResult<u64> {
    let before = self.items.len();
    match filter {
      ItemFilter::All => self.items.clear(),
      ItemFilter::ById(id) => self.items.retain(|e| e.id != *id),
    }
    Ok((before.saturating_sub(self.items.len())) as u64)
  }
}

impl ItemRepository for InMemoryItemRepository {}

pub fn create<R>(repo: &mut R, id: u64, name: impl Into<String>) -> HexResult<Item>
where
  R: ItemRepository + hexser::ports::repository::QueryRepository<Item, Filter = ItemFilter>,
{
  let item = Item {
    id,
    name: name.into(),
  };
  // naive uniqueness check via QueryRepository
  if <R as hexser::ports::repository::QueryRepository<Item>>::exists(repo, &ItemFilter::ById(id))? {
    return Err(hexser::Hexserror::domain(
      "E_HEXSER_POTIONS_ID_TAKEN",
      "ID already exists",
    ));
  }
  repo.save(item.clone())?;
  Ok(item)
}

pub fn get<R>(repo: &R, id: u64) -> HexResult<Item>
where
  R: ItemRepository + hexser::ports::repository::QueryRepository<Item, Filter = ItemFilter>,
{
  <R as hexser::ports::repository::QueryRepository<Item>>::find_one(repo, &ItemFilter::ById(id))?
    .ok_or_else(|| hexser::Hexserror::not_found("Item", &id.to_string()))
}

pub fn delete<R>(repo: &mut R, id: u64) -> HexResult<()>
where
  R: ItemRepository + hexser::ports::repository::QueryRepository<Item, Filter = ItemFilter>,
{
  let _ = <R as hexser::ports::repository::QueryRepository<Item>>::delete_where(
    repo,
    &ItemFilter::ById(id),
  )?;
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn crud_flow() {
    let mut repo = InMemoryItemRepository::default();
    let a = create(&mut repo, 1, "A").unwrap();
    assert_eq!(a.name, "A");
    let fetched = get(&repo, 1).unwrap();
    assert_eq!(fetched, a);
    delete(&mut repo, 1).unwrap();
    assert!(get(&repo, 1).is_err());
  }
}
