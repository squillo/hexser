//! Composite Profile Repository Example
//!
//! Demonstrates building a system that ingests data from both a primary SQL database
//! and a secondary, eventually-consistent NoSQL document store. Shows how to implement
//! a custom 'Adapter' for a ProfileRepository Port that consolidates user profile data
//! from both sources. The adapter's find_by_id method queries the SQL database first,
//! then enriches the Profile with additional data from the NoSQL store.
//!
//! This addresses Context7 Question 9: Implement a composite adapter that fetches from
//! multiple sources, handles failures, data inconsistencies, and caching strategies.
//!
//! Run with: cargo run --example composite_profile
//!
//! Revision History
//! - 2025-10-08T22:43:00Z @AI: Extract composite profile example from README to standalone file per user request.

/// User preferences from NoSQL store
#[derive(Clone, std::fmt::Debug)]
struct Preferences {
  theme: std::string::String,
  language: std::string::String,
  notifications_enabled: bool,
}

impl Preferences {
  fn default() -> Self {
    Self {
      theme: std::string::String::from("light"),
      language: std::string::String::from("en"),
      notifications_enabled: true,
    }
  }
}

/// Complete profile combining SQL and NoSQL data
#[derive(Clone, std::fmt::Debug)]
struct Profile {
  id: std::string::String,
  username: std::string::String,
  email: std::string::String,
  created_at: u64,
  preferences: Preferences,
}

impl hexser::domain::entity::HexEntity for Profile {
  type Id = std::string::String;
}

/// SQL row representing core profile data
#[derive(Clone, std::fmt::Debug)]
struct SqlProfileRow {
  id: std::string::String,
  username: std::string::String,
  email: std::string::String,
  created_at: u64,
}

/// Port for profile repository
trait ProfileRepository {
  fn find_by_id(&self, user_id: &str) -> hexser::result::hex_result::HexResult<Profile>;
}

/// Mock SQL database
struct MockSqlDatabase {
  profiles: std::collections::HashMap<std::string::String, SqlProfileRow>,
}

impl MockSqlDatabase {
  fn new() -> Self {
    let mut profiles = std::collections::HashMap::new();
    profiles.insert(
      std::string::String::from("user-001"),
      SqlProfileRow {
        id: std::string::String::from("user-001"),
        username: std::string::String::from("alice"),
        email: std::string::String::from("alice@example.com"),
        created_at: 1609459200,
      },
    );
    Self { profiles }
  }

  fn query(&self, user_id: &str) -> hexser::result::hex_result::HexResult<SqlProfileRow> {
    self.profiles.get(user_id).cloned().ok_or_else(|| {
      hexser::error::hex_error::Hexserror::not_found("Profile", user_id)
        .with_next_step("Verify the user ID exists")
    })
  }
}

/// Mock NoSQL database
struct MockNoSqlDatabase {
  preferences: std::collections::HashMap<std::string::String, Preferences>,
  simulate_failure: std::cell::RefCell<bool>,
}

impl MockNoSqlDatabase {
  fn new() -> Self {
    let mut preferences = std::collections::HashMap::new();
    preferences.insert(
      std::string::String::from("user-001"),
      Preferences {
        theme: std::string::String::from("dark"),
        language: std::string::String::from("es"),
        notifications_enabled: false,
      },
    );
    Self {
      preferences,
      simulate_failure: std::cell::RefCell::new(false),
    }
  }

  fn set_simulate_failure(&self, value: bool) {
    *self.simulate_failure.borrow_mut() = value;
  }

  fn find_preferences(
    &self,
    user_id: &str,
  ) -> hexser::result::hex_result::HexResult<std::option::Option<Preferences>> {
    if *self.simulate_failure.borrow() {
      return std::result::Result::Err(
        hexser::error::hex_error::Hexserror::adapter(
          hexser::error::codes::adapter::DB_CONNECTION_FAILURE,
          "NoSQL database unavailable",
        )
        .with_next_step("Check NoSQL database connectivity")
        .with_suggestion("Retry with exponential backoff"),
      );
    }
    std::result::Result::Ok(self.preferences.get(user_id).cloned())
  }
}

/// LRU cache implementation
struct LruCache {
  cache: std::sync::Mutex<std::collections::HashMap<std::string::String, (Profile, u64)>>,
  ttl_secs: u64,
  max_size: usize,
}

impl LruCache {
  fn new(max_size: usize, ttl_secs: u64) -> Self {
    Self {
      cache: std::sync::Mutex::new(std::collections::HashMap::new()),
      ttl_secs,
      max_size,
    }
  }

  fn get(&self, key: &str) -> std::option::Option<Profile> {
    let mut cache = self.cache.lock().unwrap();
    if let std::option::Option::Some((profile, inserted_at)) = cache.get(key) {
      let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
      if now - inserted_at < self.ttl_secs {
        return std::option::Option::Some(profile.clone());
      } else {
        cache.remove(key);
      }
    }
    std::option::Option::None
  }

  fn put(&self, key: std::string::String, profile: Profile) {
    let mut cache = self.cache.lock().unwrap();
    let now = std::time::SystemTime::now()
      .duration_since(std::time::UNIX_EPOCH)
      .unwrap()
      .as_secs();

    // Simple eviction: remove oldest if at capacity
    if cache.len() >= self.max_size {
      if let std::option::Option::Some(oldest_key) = cache
        .iter()
        .min_by_key(|(_, (_, inserted_at))| inserted_at)
        .map(|(k, _)| k.clone())
      {
        cache.remove(&oldest_key);
      }
    }

    cache.insert(key, (profile, now));
  }

  fn invalidate(&self, key: &str) {
    let mut cache = self.cache.lock().unwrap();
    cache.remove(key);
  }
}

/// Composite adapter that fetches from SQL and enriches with NoSQL
struct CompositeProfileRepository {
  sql_db: MockSqlDatabase,
  nosql_db: MockNoSqlDatabase,
  cache: LruCache,
}

impl CompositeProfileRepository {
  fn new(sql_db: MockSqlDatabase, nosql_db: MockNoSqlDatabase) -> Self {
    Self {
      sql_db,
      nosql_db,
      cache: LruCache::new(100, 300), // 100 entries, 5 min TTL
    }
  }
}

impl hexser::adapters::adapter::Adapter for CompositeProfileRepository {}

impl ProfileRepository for CompositeProfileRepository {
  fn find_by_id(&self, user_id: &str) -> hexser::result::hex_result::HexResult<Profile> {
    // 1) Check cache first
    if let std::option::Option::Some(cached) = self.cache.get(user_id) {
      return std::result::Result::Ok(cached);
    }

    // 2) Fetch core profile from SQL (primary source, must succeed)
    let core_profile = self.sql_db.query(user_id).map_err(|e| {
      hexser::error::hex_error::Hexserror::adapter(
        hexser::error::codes::adapter::DB_CONNECTION_FAILURE,
        "Failed to fetch core profile from SQL",
      )
      .with_next_step("Check SQL database connectivity")
    })?;

    // 3) Enrich with preferences from NoSQL (optional, degrade gracefully)
    let preferences = match self.nosql_db.find_preferences(user_id) {
      std::result::Result::Ok(std::option::Option::Some(prefs)) => {
        // Successfully fetched preferences
        prefs
      }
      std::result::Result::Ok(std::option::Option::None) => {
        // User has no preferences document yet; use defaults
        Preferences::default()
      }
      std::result::Result::Err(e) => {
        // NoSQL source failed; log warning and use defaults (degrade gracefully)
        std::eprintln!(
          "Warning: Failed to fetch preferences for {}: {}",
          user_id,
          e
        );
        Preferences::default()
      }
    };

    // 4) Combine into domain model
    let profile = Profile {
      id: core_profile.id,
      username: core_profile.username,
      email: core_profile.email,
      created_at: core_profile.created_at,
      preferences,
    };

    // 5) Cache result
    self.cache.put(user_id.to_string(), profile.clone());

    std::result::Result::Ok(profile)
  }
}

fn main() -> hexser::result::hex_result::HexResult<()> {
  std::println!("Composite Profile Repository Example\n");
  std::println!("{}", "=".repeat(60));

  std::println!("\n1. Architecture:");
  std::println!("   Primary Source: SQL database (core profile data)");
  std::println!("   Secondary Source: NoSQL database (user preferences)");
  std::println!("   Caching Layer: LRU cache with TTL");

  std::println!("\n2. Port Definition:");
  std::println!("   trait ProfileRepository {{");
  std::println!("       fn find_by_id(&self, user_id: &str) -> HexResult<Profile>;");
  std::println!("   }}");

  std::println!("\n3. Data Flow:");
  std::println!("   Step 1: Check cache (TTL-based expiry)");
  std::println!("   Step 2: Fetch core profile from SQL (must succeed)");
  std::println!("   Step 3: Enrich with preferences from NoSQL (optional)");
  std::println!("   Step 4: Combine into domain Profile model");
  std::println!("   Step 5: Cache the result");

  std::println!("\n4. Demonstration - Success Case:");
  let sql_db = MockSqlDatabase::new();
  let nosql_db = MockNoSqlDatabase::new();
  let repo = CompositeProfileRepository::new(sql_db, nosql_db);

  let profile = repo.find_by_id("user-001")?;
  std::println!(
    "   ✓ Profile fetched: {} ({})",
    profile.username,
    profile.email
  );
  std::println!(
    "   ✓ Preferences: theme={}, language={}, notifications={}",
    profile.preferences.theme,
    profile.preferences.language,
    profile.preferences.notifications_enabled
  );

  std::println!("\n5. Demonstration - Cache Hit:");
  let profile2 = repo.find_by_id("user-001")?;
  std::println!("   ✓ Profile fetched from cache (no database queries)");

  std::println!("\n6. Demonstration - NoSQL Failure (Graceful Degradation):");
  let sql_db2 = MockSqlDatabase::new();
  let nosql_db2 = MockNoSqlDatabase::new();
  nosql_db2.set_simulate_failure(true);
  let repo2 = CompositeProfileRepository::new(sql_db2, nosql_db2);

  let profile3 = repo2.find_by_id("user-001")?;
  std::println!("   ✓ Profile fetched despite NoSQL failure");
  std::println!(
    "   ✓ Default preferences used: theme={}",
    profile3.preferences.theme
  );
  std::println!("   ⚠️  Warning logged to stderr (see above)");

  std::println!("\n7. Failure Handling Strategies:");
  std::println!("   Primary Source (SQL) Failure:");
  std::println!("     - Return Hexserror::Adapter with DB_READ_FAILURE");
  std::println!("     - Include actionable guidance: 'Check SQL connectivity'");
  std::println!("   Secondary Source (NoSQL) Failure:");
  std::println!("     - Degrade gracefully with Preferences::default()");
  std::println!("     - Log warning to stderr for monitoring");
  std::println!("     - Service remains available with partial data");

  std::println!("\n8. Data Inconsistency Handling:");
  std::println!("   Stale Preferences:");
  std::println!("     - NoSQL is eventually consistent");
  std::println!("     - Cache TTL limits staleness window (5 minutes)");
  std::println!("     - Invalidate cache on preference updates");
  std::println!("   Missing Preferences:");
  std::println!("     - Use sensible defaults (Preferences::default())");
  std::println!("     - System remains functional");

  std::println!("\n9. Caching Strategies:");
  std::println!("   Read-Through Cache:");
  std::println!("     - Check cache before querying databases");
  std::println!("     - Populate cache on miss");
  std::println!("   TTL-Based Expiry:");
  std::println!("     - Each entry has timestamp");
  std::println!("     - Expired entries automatically removed");
  std::println!("   LRU Eviction:");
  std::println!("     - Max 100 entries");
  std::println!("     - Oldest entry removed when full");
  std::println!("   Write-Through (on updates):");
  std::println!("     - Invalidate cache entry on preference update");

  std::println!("\n10. Architecture Benefits:");
  std::println!("   ✓ Single port (ProfileRepository) hides complexity");
  std::println!("   ✓ Domain model (Profile) is clean and unified");
  std::println!("   ✓ Graceful degradation ensures high availability");
  std::println!("   ✓ Caching reduces load on both databases");
  std::println!("   ✓ Easy to test with mock databases");
  std::println!("   ✓ Production: swap mocks for real SQL/NoSQL clients");

  std::println!("\n11. Production Implementation:");
  std::println!("   SQL Source:");
  std::println!("     - Use sqlx::PgPool or diesel::PgConnection");
  std::println!("     - SELECT id, username, email, created_at FROM users WHERE id = $1");
  std::println!("   NoSQL Source:");
  std::println!("     - Use mongodb::Client or similar");
  std::println!("     - collection.find_one(doc! {{ \"user_id\": user_id }})");
  std::println!("   Caching:");
  std::println!("     - Use redis for distributed cache");
  std::println!("     - Or moka/lru crate for in-process cache");

  std::println!("\n{}", "=".repeat(60));
  std::println!("Example complete. See hexser/README.md for production SQL/NoSQL code.");

  std::result::Result::Ok(())
}
