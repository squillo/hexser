# Migration Guide: Repository Queries

This guide helps migrate from id-based repository methods to the new filter-oriented QueryRepository API.

## What changed
- Added QueryRepository<T> with associated types:
  - type Filter
  - type SortKey
  - methods: find_one, find, exists, count, delete_where
- Legacy methods on Repository<T> are deprecated (but available):
  - find_by_id, find_all, delete(id)

## Minimal migration steps
1. Define domain-owned filter and sort key types for each aggregate you query.

```rust
#[derive(Clone, Debug)]
pub enum UserFilter { ById(String), ByEmail(String), All }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UserSortKey { CreatedAt, Email }
```

2. Implement QueryRepository for your adapters.

```rust
impl QueryRepository<User> for InMemoryUserRepository {
    type Filter = UserFilter;
    type SortKey = UserSortKey;

    fn find_one(&self, f: &Self::Filter) -> HexResult<Option<User>> { /* match filter */ }
    fn find(&self, f: &Self::Filter, opts: FindOptions<Self::SortKey>) -> HexResult<Vec<User>> { /* filter + sort + page */ }
}
```

3. Replace usages:
- find_by_id(id) -> find_one(&Filter::ById(id))
- find_all() -> find(&Filter::All, FindOptions::default())
- Add pagination/sorting with FindOptions { sort, limit, offset }

4. Optional: delete by filter
- Replace delete(id) with delete_where(&Filter::ById(id)) when appropriate.

## Tips
- Keep filters small and composable; avoid storage-specific DSLs
- Use exists and count for efficient UIs without fetching full pages
- Maintain legacy methods temporarily while refactoring call sites

See hex/README.md for a full example and tutorials/04-cqrs-basics for CQRS usage.
