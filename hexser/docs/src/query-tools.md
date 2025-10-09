# Query Tools

Hexser provides the `Query` trait as part of its Command Query Responsibility Segregation (CQRS) implementation. This trait defines read-only operations that retrieve data without causing side effects.

## The Query Trait

The `Query` trait is defined in `hexser::ports::Query` and represents the Query side of CQRS. It provides a clean separation between read operations (queries) and write operations (commands/directives).

```rust
pub trait Query<Params, Result> {
    fn query(&self, params: Params) -> hexser::HexResult<Result>;
}
```

### Type Parameters

- `Params`: The parameters needed to execute the query
- `Result`: The type of data returned by the query

## CQRS Pattern

Command Query Responsibility Segregation (CQRS) is an architectural pattern that separates read and write operations:

- **Commands (Directives)**: Operations that modify state and may have side effects
- **Queries**: Read-only operations that return data without modifying state

This separation allows you to:
- Optimize read and write operations independently
- Use different data models for reads and writes
- Scale read and write workloads separately
- Reason about side effects more clearly

## Query vs HexQuery

Hexser provides two related but distinct concepts:

- **`Query` trait**: A port-level interface for implementing read-only operations. You implement this trait on your query handlers.
- **`HexQuery` derive macro**: Registers a struct as a query in the hexser architecture graph for analysis and visualization.

You can use them together:

```rust
#[derive(HexQuery)]
struct FindUserByEmail {
    email: std::string::String,
}

struct FindUserByEmailHandler {
    repo: std::sync::Arc<dyn crate::ports::UserRepository>,
}

impl hexser::ports::Query<FindUserByEmail, std::option::Option<User>> for FindUserByEmailHandler {
    fn query(&self, params: FindUserByEmail) -> hexser::HexResult<std::option::Option<User>> {
        self.repo.find_by_email(&params.email)
    }
}
```

## Usage Examples

### Simple Query

```rust
struct GetUserCountParams;

struct UserCount {
    total: u64,
}

struct GetUserCountQuery {
    repo: std::sync::Arc<dyn crate::ports::UserRepository>,
}

impl hexser::ports::Query<GetUserCountParams, UserCount> for GetUserCountQuery {
    fn query(&self, _params: GetUserCountParams) -> hexser::HexResult<UserCount> {
        let count = self.repo.count()?;
        std::result::Result::Ok(UserCount { total: count })
    }
}
```

### Query with Parameters

```rust
struct FindUserByIdParams {
    id: std::string::String,
}

struct UserView {
    id: std::string::String,
    email: std::string::String,
    name: std::string::String,
}

struct FindUserByIdQuery {
    repo: std::sync::Arc<dyn crate::ports::UserRepository>,
}

impl hexser::ports::Query<FindUserByIdParams, std::option::Option<UserView>> for FindUserByIdQuery {
    fn query(&self, params: FindUserByIdParams) -> hexser::HexResult<std::option::Option<UserView>> {
        let user = self.repo.find_by_id(&params.id)?;
        std::result::Result::Ok(user.map(|u| UserView {
            id: u.id,
            email: u.email,
            name: u.name,
        }))
    }
}
```

### Using Queries in Application Code

```rust
fn handle_get_user_request(
    query: &impl hexser::ports::Query<FindUserByIdParams, std::option::Option<UserView>>,
    user_id: std::string::String,
) -> hexser::HexResult<std::option::Option<UserView>> {
    let params = FindUserByIdParams { id: user_id };
    query.query(params)
}
```

## Best Practices

1. **Keep queries pure**: Queries should not modify state or have observable side effects beyond caching or logging.

2. **Use meaningful parameter types**: Define dedicated structs for query parameters rather than using tuples or primitive types directly.

3. **Return view models**: Queries should return view models (DTOs) optimized for the specific read use case, not domain entities directly.

4. **Handle not-found cases**: Use `Option<T>` for queries that may not find a result, rather than returning errors for normal not-found scenarios.

5. **Optimize for reads**: Query implementations can use read-optimized data stores, caching, or denormalized views without affecting the domain model.

6. **Test independently**: Write unit tests for queries that verify they return correct data without depending on command/write operations.

## Integration with Repositories

Queries often delegate to repositories for data access:

```rust
struct ListActiveUsersQuery {
    repo: std::sync::Arc<dyn crate::ports::UserRepository>,
}

struct ListActiveUsersParams {
    limit: usize,
    offset: usize,
}

impl hexser::ports::Query<ListActiveUsersParams, std::vec::Vec<UserView>> for ListActiveUsersQuery {
    fn query(&self, params: ListActiveUsersParams) -> hexser::HexResult<std::vec::Vec<UserView>> {
        let users = self.repo.find_active(params.limit, params.offset)?;
        let views = users.into_iter().map(|u| UserView {
            id: u.id,
            email: u.email,
            name: u.name,
        }).collect();
        std::result::Result::Ok(views)
    }
}
```

## See Also

- [Core Concepts](core-concepts.md) - Overview of CQRS in hexser
- [Repositories](core-concepts.md#repositories-generic-queries-filter--options) - Repository pattern and QueryRepository
- [Errors](errors.md) - Error handling in queries
