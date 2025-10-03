//! CQRS pattern example with hex.
//!
//! This example demonstrates Command Query Responsibility Segregation (CQRS)
//! using hex's Directive and Query traits. It shows how to separate write
//! operations (directives) from read operations (queries).
//!
//! Run with: `cargo run --example cqrs_pattern`

use hexer::{Directive, DirectiveHandler, QueryHandler};

fn main() -> hexer::HexResult<()> {
    println!("=== CQRS Pattern Example ===\n");

    // Create repositories
    let write_repo = InMemoryUserRepository::new();
    let query_repo = InMemoryUserQueryRepository::new();

    // Write side: Create user via directive
    let create_handler = CreateUserHandler {
        repository: write_repo,
    };

    let create_directive = CreateUserDirective {
        email: String::from("alice@example.com"),
        name: String::from("Alice"),
    };

    println!("Creating user via directive...");
    create_handler.handle(create_directive)?;
    println!("✓ User created");

    // Read side: Query user
    let query_handler = FindUserByEmailHandler {
        repository: &query_repo,
    };

    let find_query = FindUserByEmailQuery {
        email: String::from("alice@example.com"),
    };

    println!("\nQuerying user...");
    if let Some(user_view) = query_handler.handle(find_query)? {
        println!("✓ Found user: {} ({})", user_view.name, user_view.email);
    }

    println!("\n✅ CQRS example completed!");

    Ok(())
}

// Domain: User entity
#[derive(Clone)]
struct User {
    id: String,
    email: String,
    name: String,
}

impl hexer::domain::Entity for User {
    type Id = String;
}

// Write side: Directive
struct CreateUserDirective {
    email: String,
    name: String,
}

impl hexer::application::Directive for CreateUserDirective {
    fn validate(&self) -> hexer::HexResult<()> {
        if !self.email.contains('@') {
            return Err(hexer::HexError::validation("Invalid email format")
                .with_field("email"));
        }
        if self.name.is_empty() {
            return Err(hexer::HexError::validation("Name cannot be empty")
                .with_field("name"));
        }
        Ok(())
    }
}

// Write side: Directive handler
struct CreateUserHandler {
    repository: InMemoryUserRepository,
}

impl hexer::application::DirectiveHandler<CreateUserDirective>
    for CreateUserHandler {
    fn handle(&self, directive: CreateUserDirective) -> hexer::HexResult<()> {
        directive.validate()?;

        let user = User {
            id: String::from("1"),
            email: directive.email,
            name: directive.name,
        };

        // In a real implementation, this would use interior mutability
        // or the handler would own the repository
        println!("Would save user: {} <{}>", user.name, user.email);
        Ok(())
    }
}

// Read side: Query
struct FindUserByEmailQuery {
    email: String,
}

// Read side: View model
#[derive(Clone)]
struct UserView {
    id: String,
    email: String,
    name: String,
}

// Read side: Query handler
struct FindUserByEmailHandler<'a> {
    repository: &'a InMemoryUserQueryRepository,
}

impl<'a> hexer::application::QueryHandler<FindUserByEmailQuery, Option<UserView>>
    for FindUserByEmailHandler<'a> {
    fn handle(&self, query: FindUserByEmailQuery) -> hexer::HexResult<Option<UserView>> {
        self.repository.find_by_email(&query.email)
    }
}

// Write repository (for commands)
struct InMemoryUserRepository {
    users: Vec<User>,
}

impl InMemoryUserRepository {
    fn new() -> Self {
        Self { users: Vec::new() }
    }
}

impl hexer::adapters::Adapter for InMemoryUserRepository {}

impl hexer::ports::Repository<User> for InMemoryUserRepository {
    fn find_by_id(&self, id: &String) -> hexer::HexResult<Option<User>> {
        Ok(self.users.iter().find(|u| &u.id == id).cloned())
    }

    fn save(&mut self, user: User) -> hexer::HexResult<()> {
        self.users.push(user);
        Ok(())
    }

    fn delete(&mut self, id: &String) -> hexer::HexResult<()> {
        self.users.retain(|u| &u.id != id);
        Ok(())
    }

    fn find_all(&self) -> hexer::HexResult<Vec<User>> {
        Ok(self.users.clone())
    }
}

// Read repository (for queries)
struct InMemoryUserQueryRepository {
    users: Vec<UserView>,
}

impl InMemoryUserQueryRepository {
    fn new() -> Self {
        // Preload with sample data
        Self {
            users: vec![UserView {
                id: String::from("1"),
                email: String::from("alice@example.com"),
                name: String::from("Alice"),
            }],
        }
    }

    fn find_by_email(&self, email: &str) -> hexer::HexResult<Option<UserView>> {
        Ok(self.users.iter().find(|u| u.email == email).cloned())
    }
}
