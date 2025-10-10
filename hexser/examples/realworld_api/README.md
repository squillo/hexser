# RealWorld API - Hexagonal Architecture Example

A complete, production-quality implementation of the [RealWorld API specification](https://realworld-docs.netlify.app/introduction) using the hexser framework. This example demonstrates best practices for building clean, maintainable applications with hexagonal architecture, CQRS, and domain-driven design.

## Overview

This example serves as a canonical reference implementation showcasing:

- **Hexagonal Architecture (DPAI)**: Strict separation of Domain, Ports, Application, and Adapters
- **CQRS Pattern**: Command Query Responsibility Segregation with Directives and Queries
- **Repository Pattern**: Domain-owned Filter and SortKey enums for type-safe querying
- **Rich Error Handling**: Actionable error messages with Hexserror
- **Zero Use Statements**: All types fully qualified for maximum clarity
- **Thread-Safe Adapters**: In-memory storage with Arc<Mutex<>> for concurrency
- **REST API with axum**: Complete HTTP server with JWT authentication
- **Comprehensive Testing**: 74+ tests covering all layers

## Project Structure

```
realworld_api/
├── src/
│   ├── domain/           # Pure business logic (no dependencies)
│   │   ├── user.rs       # User entity with follow/unfollow
│   │   ├── article.rs    # Article entity with favorites
│   │   ├── comment.rs    # Comment entity
│   │   └── tag.rs        # Tag value object
│   │
│   ├── ports/            # Abstract interfaces (traits)
│   │   ├── user_repository.rs       # UserRepository + filters
│   │   ├── article_repository.rs    # ArticleRepository + filters
│   │   ├── comment_repository.rs    # CommentRepository + filters
│   │   └── tag_repository.rs        # TagRepository trait
│   │
│   ├── application/      # Use case orchestration
│   │   ├── user/         # User directives & queries
│   │   │   ├── register.rs      # RegisterUserDirective
│   │   │   ├── login.rs         # LoginUserQuery
│   │   │   ├── get_current.rs   # GetCurrentUserQuery
│   │   │   └── update.rs        # UpdateUserDirective
│   │   │
│   │   ├── article/      # Article directives & queries
│   │   │   ├── create.rs        # CreateArticleDirective
│   │   │   ├── list.rs          # ListArticlesQuery
│   │   │   ├── get.rs           # GetArticleQuery
│   │   │   ├── update.rs        # UpdateArticleDirective
│   │   │   ├── delete.rs        # DeleteArticleDirective
│   │   │   ├── feed.rs          # GetArticleFeedQuery
│   │   │   └── favorite.rs      # Favorite/Unfavorite directives
│   │   │
│   │   ├── comment/      # Comment directives & queries
│   │   │   ├── add.rs           # AddCommentDirective
│   │   │   ├── get.rs           # GetCommentsQuery
│   │   │   └── delete.rs        # DeleteCommentDirective
│   │   │
│   │   ├── profile/      # Profile directives & queries
│   │   │   ├── get.rs           # GetProfileQuery
│   │   │   ├── follow.rs        # FollowUserDirective
│   │   │   └── unfollow.rs      # UnfollowUserDirective
│   │   │
│   │   └── tag/          # Tag queries
│   │       └── get_all.rs       # GetAllTagsQuery
│   │
│   ├── adapters/         # Concrete implementations
│   │   ├── in_memory_db/ # In-memory persistence adapters
│   │   │   ├── user_adapter.rs
│   │   │   ├── article_adapter.rs
│   │   │   ├── comment_adapter.rs
│   │   │   └── tag_adapter.rs
│   │   │
│   │   └── web/          # HTTP/REST adapters (axum)
│   │       ├── auth.rs           # JWT authentication middleware
│   │       ├── user_routes.rs    # User & auth endpoints
│   │       ├── article_routes.rs # Article CRUD endpoints
│   │       ├── comment_routes.rs # Comment endpoints
│   │       ├── profile_routes.rs # Profile & follow endpoints
│   │       ├── tag_routes.rs     # Tag endpoints
│   │       └── routes.rs         # Main router configuration
│   │
│   ├── main.rs           # Demonstration mode entry point
│   ├── lib.rs            # Library exports
│   └── bin/
│       └── web_server.rs # HTTP server entry point
│
├── Cargo.toml
└── README.md (this file)
```

## Architecture Visualization

The RealWorld API architecture can be automatically visualized using hexser's built-in graph introspection capabilities. The architecture diagram shows all components registered with hexser's derive macros:

```mermaid
graph TD
  NodeId(10412492039965543270)["realworld_api::domain::article::Article\n(Entity)"]
  NodeId(10347384604705785648)["realworld_api::adapters::in_memory_db::user_adapter::InMemoryUserRepository\n(Adapter)"]
  NodeId(7785202712665265961)["realworld_api::ports::comment_repository::CommentFilter\n(Repository)"]
  NodeId(14618512971669648406)["realworld_api::domain::tag::Tag\n(Entity)"]
  NodeId(10644884200148206392)["realworld_api::adapters::in_memory_db::comment_adapter::InMemoryCommentRepository\n(Adapter)"]
  NodeId(528349728044715595)["realworld_api::ports::article_repository::ArticleFilter\n(Repository)"]
  NodeId(4722985126513128954)["realworld_api::adapters::in_memory_db::article_adapter::InMemoryArticleRepository\n(Adapter)"]
  NodeId(7498128437058494122)["realworld_api::adapters::in_memory_db::tag_adapter::InMemoryTagRepository\n(Adapter)"]
  NodeId(17104016576760588564)["realworld_api::ports::comment_repository::CommentSortKey\n(Repository)"]
  NodeId(20562250797671452)["realworld_api::domain::user::User\n(Entity)"]
  NodeId(6036380904514364673)["realworld_api::ports::user_repository::UserFilter\n(Repository)"]
  NodeId(14733129128909500908)["realworld_api::ports::user_repository::UserSortKey\n(Repository)"]
  NodeId(17435541042506597494)["realworld_api::ports::article_repository::ArticleSortKey\n(Repository)"]
  NodeId(4583760324640920868)["realworld_api::domain::comment::Comment\n(Entity)"]
```

### Regenerating the Diagram

To regenerate the architecture diagram after making changes, use the provided pipeline script:

```bash
cd hexser/examples/realworld_api
./regenerate_diagram.sh
```

Or run the generator directly:

```bash
cargo run --bin generate_architecture_diagram
```

This will:
1. Collect all registered hexser components via introspection
2. Generate a Mermaid diagram showing the architecture
3. Generate an AI Agent Pack JSON with architecture metadata
4. Save the Mermaid diagram to `architecture_diagram.mmd`
5. Save the AI pack JSON to `architecture_ai_pack.json`
6. Display statistics about your architecture layers (4 entities, 6 ports, 4 adapters)

Both outputs automatically update to reflect any structural changes in the codebase, making them living documentation artifacts. The pipeline script (`regenerate_diagram.sh`) provides a quick, continuous way to rebuild both the visualization and AI-readable metadata as your architecture evolves.

The `architecture_ai_pack.json` file contains comprehensive architecture metadata in JSON format, designed for consumption by AI assistants and external tools. It includes component relationships, layer information, and architectural patterns.

## Features

### User Management & Authentication
- **POST /api/users** - User registration with validation
- **POST /api/users/login** - Authentication with JWT token generation
- **GET /api/user** - Get current authenticated user (requires auth)
- **PUT /api/user** - Update user profile (requires auth)

### Profile Operations
- **GET /api/profiles/:username** - View user profile
- **POST /api/profiles/:username/follow** - Follow a user (requires auth)
- **DELETE /api/profiles/:username/follow** - Unfollow a user (requires auth)

### Article Management
- **GET /api/articles** - List articles with filtering (by tag, author, favorited)
- **POST /api/articles** - Create article (requires auth)
- **GET /api/articles/feed** - Get personalized feed from followed users (requires auth)
- **GET /api/articles/:slug** - Get single article by slug
- **PUT /api/articles/:slug** - Update article (requires auth, author only)
- **DELETE /api/articles/:slug** - Delete article (requires auth, author only)
- **POST /api/articles/:slug/favorite** - Favorite an article (requires auth)
- **DELETE /api/articles/:slug/favorite** - Unfavorite an article (requires auth)

### Comments
- **GET /api/articles/:slug/comments** - Get all comments for an article
- **POST /api/articles/:slug/comments** - Add comment to article (requires auth)
- **DELETE /api/articles/:slug/comments/:id** - Delete comment (requires auth, author only)

### Tags
- **GET /api/tags** - Get all unique tags used in articles

## Running the Example

### Mode 1: Demonstration Mode

Runs a scripted demonstration showing all architectural patterns and use cases:

```bash
cd hexser/examples/realworld_api
cargo run --bin realworld_api
```

This mode:
- Demonstrates hexagonal architecture patterns
- Shows CQRS with Directives and Queries
- Executes example workflows (register, login, create article, etc.)
- Validates domain logic and business rules
- Outputs detailed logging of operations

### Mode 2: Web Server Mode

Starts a full HTTP REST API server on port 3000:

```bash
cd hexser/examples/realworld_api
cargo run --bin web_server
```

Or from the workspace root:

```bash
cargo run --example realworld_api --bin web_server
```

The server will start at `http://0.0.0.0:3000` with all RealWorld API endpoints available.

#### Example API Usage

**Register a user:**
```bash
curl -X POST http://localhost:3000/api/users \
  -H "Content-Type: application/json" \
  -d '{
    "user": {
      "email": "jake@example.com",
      "username": "jake",
      "password": "password123"
    }
  }'
```

**Login:**
```bash
curl -X POST http://localhost:3000/api/users/login \
  -H "Content-Type: application/json" \
  -d '{
    "user": {
      "email": "jake@example.com",
      "password": "password123"
    }
  }'
```

**Create an article (with auth):**
```bash
curl -X POST http://localhost:3000/api/articles \
  -H "Content-Type: application/json" \
  -H "Authorization: Token YOUR_JWT_TOKEN" \
  -d '{
    "article": {
      "title": "How to train your dragon",
      "description": "Ever wonder how?",
      "body": "Very carefully.",
      "tagList": ["dragons", "training"]
    }
  }'
```

**List articles:**
```bash
curl http://localhost:3000/api/articles
```

**Get article by slug:**
```bash
curl http://localhost:3000/api/articles/how-to-train-your-dragon
```

## Running Tests

Execute all 74+ tests covering domain logic, application handlers, and adapter implementations:

```bash
cd hexser/examples/realworld_api
cargo test
```

Tests include:
- Domain entity behavior tests
- Directive and Query validation tests
- Repository adapter tests
- Handler integration tests
- JWT authentication tests

All tests use fully qualified paths and demonstrate best practices.

## Architecture Patterns Demonstrated

### 1. Hexagonal Architecture (DPAI)

**Domain Layer** - Pure business logic, no external dependencies:
```rust
// domain/user.rs
impl User {
    pub fn follow(&mut self, user_id: std::string::String) {
        if !self.followed_users.contains(&user_id) {
            self.followed_users.push(user_id);
        }
    }
}
```

**Ports Layer** - Abstract interfaces with domain-owned filters:
```rust
// ports/user_repository.rs
pub enum UserFilter {
    ById(std::string::String),
    ByEmail(std::string::String),
    ByUsername(std::string::String),
}

pub trait UserRepository:
    hexser::ports::Repository<crate::domain::user::User>
    + hexser::ports::repository::QueryRepository<
        crate::domain::user::User,
        Filter = UserFilter,
        SortKey = UserSortKey,
    >
{}
```

**Application Layer** - Use case orchestration with CQRS:
```rust
// application/user/register.rs
impl hexser::DirectiveHandler<RegisterUserDirective> for RegisterUserHandler<R> {
    fn handle(&self, directive: RegisterUserDirective) -> hexser::HexResult<()> {
        hexser::Directive::validate(&directive)?;
        // Business logic here
    }
}
```

**Adapters Layer** - Concrete implementations:
```rust
// adapters/in_memory_db/user_adapter.rs
impl hexser::ports::Repository<crate::domain::user::User> for InMemoryUserRepository {
    fn save(&mut self, user: crate::domain::user::User) -> hexser::HexResult<()> {
        // Implementation
    }
}
```

### 2. CQRS Pattern

**Commands (Directives)** - Write operations:
- `RegisterUserDirective` - Creates new users
- `CreateArticleDirective` - Creates articles
- `FollowUserDirective` - Modifies relationships

**Queries** - Read operations:
- `LoginUserQuery` - Authenticates and returns user data
- `ListArticlesQuery` - Retrieves filtered article lists
- `GetProfileQuery` - Returns profile information

### 3. Repository Pattern with Domain-Owned Filters

Instead of exposing implementation details, filters are domain concepts:

```rust
let filter = crate::ports::article_repository::ArticleFilter::And(vec![
    ArticleFilter::ByTag(std::string::String::from("rust")),
    ArticleFilter::ByAuthor(author_id),
]);

let articles = hexser::ports::repository::QueryRepository::find(
    &*repo,
    &filter,
    hexser::ports::repository::FindOptions {
        sort: std::option::Option::Some(vec![
            hexser::ports::repository::Sort {
                key: ArticleSortKey::CreatedAt,
                direction: hexser::ports::repository::Direction::Desc,
            }
        ]),
        limit: std::option::Option::Some(20),
        offset: std::option::Option::Some(0),
    }
)?;
```

### 4. Rich Error Handling

All errors include actionable context:

```rust
if user.password_hash != password_hash {
    return std::result::Result::Err(
        hexser::Hexserror::validation("Invalid password")
            .with_field("password")
            .with_next_step("Check your password and try again")
    );
}
```

### 5. Zero Use Statements

All types are fully qualified for maximum clarity and multi-agent analysis:

```rust
pub struct User {
    pub id: std::string::String,
    pub email: std::string::String,
    pub username: std::string::String,
    pub followed_users: std::vec::Vec<std::string::String>,
}
```

## Key Design Decisions

### Why In-Memory Adapters?

The in-memory adapters make this example:
- **Self-contained**: No database setup required
- **Fast**: Instant test execution
- **Clear**: Easy to understand adapter patterns
- **Portable**: Runs anywhere Rust runs

In production, replace with PostgreSQL, MongoDB, or any other persistence adapter.

### Why No Async Domain Logic?

Domain logic is pure and synchronous. Only adapters (web, database) are async:
- Domain entities remain simple and testable
- Async concerns stay at the edges (hexagonal principle)
- Easy to reason about business rules

### Why JWT in the Adapter?

Authentication is an infrastructure concern:
- JWT implementation in `adapters/web/auth.rs`
- Domain only knows about user IDs
- Easy to swap JWT for OAuth, sessions, etc.

## Development Guidelines

This example follows strict coding standards:

1. **NO `use` statements** - All paths fully qualified
2. **One item per file** - Each struct/enum/fn in its own file
3. **File-level documentation** - Every file has `//!` doc comments
4. **Revision history** - Track all changes with timestamps
5. **In-file tests** - Tests colocated with implementation
6. **Function length** - Maximum 50 lines of code
7. **Validation** - All directives implement validation
8. **Error context** - All errors include next steps

## Extending the Example

### Adding a New Feature

1. **Domain**: Define entity or value object in `src/domain/`
2. **Ports**: Create repository trait with filters in `src/ports/`
3. **Application**: Implement directives/queries in `src/application/`
4. **Adapters**: 
   - Add in-memory implementation in `src/adapters/in_memory_db/`
   - Add HTTP routes in `src/adapters/web/`
5. **Tests**: Add tests in each file's `#[cfg(test)]` module

### Swapping Adapters

Replace in-memory with real database:

1. Create new adapter module: `src/adapters/postgres/`
2. Implement same repository traits
3. Update `main.rs` or `web_server.rs` to use new adapter
4. No changes needed to domain, ports, or application layers!

## Additional Resources

- [hexser Documentation](../../README.md)
- [RealWorld API Spec](https://realworld-docs.netlify.app/introduction)
- [Hexagonal Architecture](https://alistair.cockburn.us/hexagonal-architecture/)
- [CQRS Pattern](https://martinfowler.com/bliki/CQRS.html)

## Revision History

- 2025-10-10T10:02:00Z @AI: Initial README creation for realworld_api example.
