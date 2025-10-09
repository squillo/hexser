//! User Authentication Integration Example
//!
//! Demonstrates how to integrate a User Authentication Potion from hexser_potions
//! into a new Hexser application. Shows how to connect the Potion's defined Ports
//! to concrete database and session management Adapters.
//!
//! This addresses Context7 Question 4: Describe the steps to integrate the 'User
//! Authentication' Potion and connect Ports to concrete Adapters for database and
//! session management.
//!
//! Run with: cargo run --example auth_integration
//!
//! Revision History
//! - 2025-10-08T22:43:00Z @AI: Extract auth integration example from README to standalone file per user request.

/// User entity representing authenticated users
#[derive(Clone, std::fmt::Debug)]
struct User {
    id: std::string::String,
    username: std::string::String,
    email: std::string::String,
    password_hash: std::string::String,
}

impl hexser::domain::entity::Entity for User {
    type Id = std::string::String;
}

/// Port for user persistence operations
trait UserRepository: hexser::ports::repository::Repository<User> {
    fn find_by_username(&self, username: &str) -> hexser::result::hex_result::HexResult<std::option::Option<User>>;
    fn find_by_email(&self, email: &str) -> hexser::result::hex_result::HexResult<std::option::Option<User>>;
}

/// Port for session management operations
trait SessionPort {
    fn create_session(&self, user_id: &str, ttl_secs: u64) -> hexser::result::hex_result::HexResult<std::string::String>;
    fn validate_session(&self, token: &str) -> hexser::result::hex_result::HexResult<std::option::Option<std::string::String>>;
    fn revoke_session(&self, token: &str) -> hexser::result::hex_result::HexResult<()>;
}

/// In-memory user repository adapter (for demonstration)
struct InMemoryUserRepository {
    users: std::sync::Arc<std::sync::Mutex<std::vec::Vec<User>>>,
}

impl InMemoryUserRepository {
    fn new() -> Self {
        Self {
            users: std::sync::Arc::new(std::sync::Mutex::new(std::vec::Vec::new())),
        }
    }
}

impl hexser::adapters::adapter::Adapter for InMemoryUserRepository {}

impl hexser::ports::repository::Repository<User> for InMemoryUserRepository {
    fn save(&mut self, user: User) -> hexser::result::hex_result::HexResult<()> {
        let mut users = self.users.lock().unwrap();
        if let std::option::Option::Some(pos) = users.iter().position(|u| u.id == user.id) {
            users[pos] = user;
        } else {
            users.push(user);
        }
        std::result::Result::Ok(())
    }
}

impl UserRepository for InMemoryUserRepository {
    fn find_by_username(&self, username: &str) -> hexser::result::hex_result::HexResult<std::option::Option<User>> {
        let users = self.users.lock().unwrap();
        std::result::Result::Ok(users.iter().find(|u| u.username == username).cloned())
    }

    fn find_by_email(&self, email: &str) -> hexser::result::hex_result::HexResult<std::option::Option<User>> {
        let users = self.users.lock().unwrap();
        std::result::Result::Ok(users.iter().find(|u| u.email == email).cloned())
    }
}

/// In-memory session adapter (for demonstration)
struct InMemorySessionAdapter {
    sessions: std::sync::Arc<std::sync::Mutex<std::collections::HashMap<std::string::String, (std::string::String, u64)>>>,
}

impl InMemorySessionAdapter {
    fn new() -> Self {
        Self {
            sessions: std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::new())),
        }
    }

    fn current_timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

impl hexser::adapters::adapter::Adapter for InMemorySessionAdapter {}

impl SessionPort for InMemorySessionAdapter {
    fn create_session(&self, user_id: &str, ttl_secs: u64) -> hexser::result::hex_result::HexResult<std::string::String> {
        let token = std::format!("token_{}", Self::current_timestamp());
        let expires_at = Self::current_timestamp() + ttl_secs;
        let mut sessions = self.sessions.lock().unwrap();
        sessions.insert(token.clone(), (user_id.to_string(), expires_at));
        std::result::Result::Ok(token)
    }

    fn validate_session(&self, token: &str) -> hexser::result::hex_result::HexResult<std::option::Option<std::string::String>> {
        let sessions = self.sessions.lock().unwrap();
        if let std::option::Option::Some((user_id, expires_at)) = sessions.get(token) {
            if *expires_at > Self::current_timestamp() {
                return std::result::Result::Ok(std::option::Option::Some(user_id.clone()));
            }
        }
        std::result::Result::Ok(std::option::Option::None)
    }

    fn revoke_session(&self, token: &str) -> hexser::result::hex_result::HexResult<()> {
        let mut sessions = self.sessions.lock().unwrap();
        sessions.remove(token);
        std::result::Result::Ok(())
    }
}

/// Application context wiring adapters together
struct AppContext {
    user_repo: std::boxed::Box<dyn UserRepository>,
    session_port: std::boxed::Box<dyn SessionPort>,
}

impl AppContext {
    fn new_in_memory() -> Self {
        Self {
            user_repo: std::boxed::Box::new(InMemoryUserRepository::new()),
            session_port: std::boxed::Box::new(InMemorySessionAdapter::new()),
        }
    }
}

fn main() -> hexser::result::hex_result::HexResult<()> {
    std::println!("User Authentication Integration Example\n");
    std::println!("{}", "=".repeat(60));

    std::println!("\n1. Port Definitions:");
    std::println!("   UserRepository: Persistence abstraction");
    std::println!("     - find_by_username(&self, username: &str)");
    std::println!("     - find_by_email(&self, email: &str)");
    std::println!("   SessionPort: Session management abstraction");
    std::println!("     - create_session(&self, user_id: &str, ttl_secs: u64)");
    std::println!("     - validate_session(&self, token: &str)");
    std::println!("     - revoke_session(&self, token: &str)");

    std::println!("\n2. Concrete Adapters:");
    std::println!("   InMemoryUserRepository: Vec-based storage");
    std::println!("   InMemorySessionAdapter: HashMap-based sessions");
    std::println!("   (In production: PostgresUserRepository, RedisSessionAdapter)");

    std::println!("\n3. Demonstration:");
    let mut user_repo = InMemoryUserRepository::new();
    let session_adapter = InMemorySessionAdapter::new();

    // Create a user
    let user = User {
        id: std::string::String::from("user-001"),
        username: std::string::String::from("alice"),
        email: std::string::String::from("alice@example.com"),
        password_hash: std::string::String::from("$2b$12$hashedpassword"),
    };

    <InMemoryUserRepository as hexser::ports::repository::Repository<User>>::save(&mut user_repo, user.clone())?;
    std::println!("   ✓ User created: {} ({})", user.username, user.email);

    // Find user by username
    let found = user_repo.find_by_username("alice")?;
    std::println!("   ✓ User found by username: {:?}", found.is_some());

    // Create session
    let token = session_adapter.create_session(&user.id, 3600)?;
    std::println!("   ✓ Session created: {} (TTL: 3600s)", token);

    // Validate session
    let validated = session_adapter.validate_session(&token)?;
    std::println!("   ✓ Session validated: user_id = {:?}", validated);

    // Revoke session
    session_adapter.revoke_session(&token)?;
    std::println!("   ✓ Session revoked");

    let revalidated = session_adapter.validate_session(&token)?;
    std::println!("   ✓ Session no longer valid: {:?}", revalidated.is_none());

    std::println!("\n4. Integration Steps:");
    std::println!("   Step 1: Define UserRepository and SessionPort traits");
    std::println!("   Step 2: Implement InMemoryUserRepository (dev/test)");
    std::println!("   Step 3: Implement InMemorySessionAdapter (dev/test)");
    std::println!("   Step 4: Wire adapters in AppContext");
    std::println!("   Step 5: Swap adapters for production (Postgres + Redis)");

    std::println!("\n5. Production Adapters (pseudocode):");
    std::println!("   PostgresUserRepository:");
    std::println!("     - Uses sqlx::PgPool for database connection");
    std::println!("     - Implements find_by_username with SQL query");
    std::println!("     - Error mapping: DB_READ_FAILURE, DB_WRITE_FAILURE");
    std::println!("   RedisSessionAdapter:");
    std::println!("     - Uses redis::Client for session storage");
    std::println!("     - SETEX for create_session (atomic TTL)");
    std::println!("     - GET for validate_session");
    std::println!("     - DEL for revoke_session");

    std::println!("\n6. Error Handling:");
    std::println!("   - Hexserror::adapter() for infrastructure failures");
    std::println!("   - with_source() preserves underlying error chain");
    std::println!("   - with_next_steps() provides actionable guidance");
    std::println!("   - Example: CONNECTION_FAILURE, DB_READ_FAILURE, DB_WRITE_FAILURE");

    std::println!("\n7. Architecture Benefits:");
    std::println!("   ✓ Domain logic (User entity) has no infrastructure dependencies");
    std::println!("   ✓ Ports define clear contracts for persistence and sessions");
    std::println!("   ✓ Adapters can be swapped without changing business logic");
    std::println!("   ✓ Easy to test with in-memory implementations");
    std::println!("   ✓ Production adapters use real databases/caches");

    std::println!("\n{}", "=".repeat(60));
    std::println!("Example complete. See hexser/README.md for production adapter code.");

    std::result::Result::Ok(())
}
