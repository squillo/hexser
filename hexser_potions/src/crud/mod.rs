//! CRUD potions: a minimal in-memory repository for a single entity type.
//!
//! Demonstrates implementing the `Repository<T>` trait with a simple adapter
//! and using it from application code.

use hexser::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Item {
    pub id: u64,
    pub name: String,
}

impl Entity for Item {
    type Id = u64;
}

pub trait ItemRepository: Repository<Item> {}

#[derive(Default)]
pub struct InMemoryItemRepository {
    pub items: Vec<Item>,
}

impl Repository<Item> for InMemoryItemRepository {
    fn find_by_id(&self, id: &u64) -> HexResult<Option<Item>> {
        Ok(self.items.iter().find(|e| e.id == *id).cloned())
    }

    fn save(&mut self, entity: Item) -> HexResult<()> {
        if let Some(existing) = self.items.iter_mut().find(|e| e.id == entity.id) {
            *existing = entity;
        } else {
            self.items.push(entity);
        }
        Ok(())
    }

    fn delete(&mut self, id: &u64) -> HexResult<()> {
        self.items.retain(|e| e.id != *id);
        Ok(())
    }

    fn find_all(&self) -> HexResult<Vec<Item>> {
        Ok(self.items.clone())
    }
}

impl ItemRepository for InMemoryItemRepository {}

pub fn create<R: ItemRepository>(repo: &mut R, id: u64, name: impl Into<String>) -> HexResult<Item> {
    let item = Item { id, name: name.into() };
    // naive uniqueness check
    if repo.find_by_id(&id)?.is_some() {
        return Err(hexser::Hexserror::domain("E_HEXSER_POTIONS_ID_TAKEN", "ID already exists"));
    }
    repo.save(item.clone())?;
    Ok(item)
}

pub fn get<R: ItemRepository>(repo: &R, id: u64) -> HexResult<Item> {
    repo.find_by_id(&id)?.ok_or_else(|| hexser::Hexserror::not_found("Item", &id.to_string()))
}

pub fn delete<R: ItemRepository>(repo: &mut R, id: u64) -> HexResult<()> {
    repo.delete(&id)
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
