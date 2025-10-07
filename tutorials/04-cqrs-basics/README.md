# Tutorial 04: CQRS Basics (20 minutes)

## Goal
Learn Command Query Responsibility Segregation (CQRS) pattern using directives and queries.

## Key Concepts
- **CQRS** separates reads (queries) from writes (commands)
- **Directives** handle write operations (commands)
- **Queries** handle read operations
- **QueryHandlers** execute queries
- **DirectiveHandlers** execute directives

## Prerequisites
- Completed Tutorials 01-03
- Understanding of command vs query operations

## The CQRS Pattern


# Tutorial 04: CQRS Basics (20 minutes)

## Goal
Learn Command Query Responsibility Segregation (CQRS) using directives (writes) and queries (reads) with the generic repository API.

## Key Concepts
- CQRS separates reads (queries) from writes (directives)
- Directives mutate state; Queries read state
- QueryRepository provides filter-based reads with sorting and pagination

## Prerequisites
- Completed Tutorials 01-03
- Understanding of command vs query operations

## The CQRS Pattern
In hexser, we model writes with Directives and reads with Queries. Repositories are read/write adapters; for reads, prefer the filter-based API:

```rust
use hexser::ports::repository::{QueryRepository, FindOptions, Sort, Direction};

// Suppose we already have these domain types
struct User { id: String, email: String, created_at: u64 }
#[derive(Clone, Debug)]
enum UserFilter { All, ByEmail(String) }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum UserSortKey { CreatedAt }

// A simple Query object
struct ListUsers { limit: u32, offset: u64 }

// A handler using a generic repository (adapter supplies Filter/SortKey)
struct ListUsersHandler<R> { repo: R }

impl<R> ListUsersHandler<R>
where
    R: QueryRepository<User>,
{
    fn execute(&self, q: ListUsers) -> hexser::HexResult<Vec<User>> {
        let opts = FindOptions { sort: Some(vec![Sort { key: UserSortKey::CreatedAt, direction: Direction::Desc }]), limit: Some(q.limit), offset: Some(q.offset) };
        <R as QueryRepository<User>>::find(&self.repo, &UserFilter::All, opts)
    }
}

// For unique reads:
// let found = <R as QueryRepository<User>>::find_one(&self.repo, &UserFilter::ByEmail("a@b.com".into()))?;
```

Notes:
- Keep filters and sort keys small and domain-centric (e.g., ByEmail, Active, CreatedAt)
- Avoid adapter-specific DSLs in filters; adapters translate filters to storage queries
- For pagination UIs, combine find with count for total rows

Next: Implement a Directive (e.g., SignUpUser) and pair it with a Query to list users.
