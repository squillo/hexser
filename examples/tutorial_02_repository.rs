//! Tutorial 02: Repository Port
//!
//! Learn to define ports (interfaces) for persistence.
//! Time: 10 minutes
//!
//! Run with: `cargo run --example tutorial_02_repository`

fn main() -> hex::HexResult<()> {
    println!("=== Tutorial 02: Repository Port ===\n");

    // From Tutorial 01: Our domain entity
    #[derive(Clone, Debug)]
    struct User {
        id: String,
        email: String,
        name: String,
    }

    impl hex::domain::Entity for User {
        type Id = String;
    }

    // NEW: Define a Port (interface) for persistence
    // Ports define WHAT we need, not HOW to do it
    trait UserRepository: hex::ports::Repository<User> {
        fn find_by_email(&self, email: &str) -> hex::HexResult<Option<User>>;
        fn count(&self) -> hex::HexResult<usize>;
    }

    println!("✅ Defined UserRepository port");
    println!("   - Extends base Repository<User> trait");
    println!("   - Adds custom queries: find_by_email, count");
    println!("   - Lives in the Port layer (interfaces only!)");

    println!("\n🎯 Architecture so far:");
    println!("   ┌─────────────────┐");
    println!("   │     Domain      │  ← User entity");
    println!("   │     [User]      │");
    println!("   └────────┬────────┘");
    println!("            │ depends on");
    println!("            ↓");
    println!("   ┌─────────────────┐");
    println!("   │      Port       │  ← UserRepository interface");
    println!("   │ [UserRepository]│");
    println!("   └─────────────────┘");

    println!("\n📚 What you learned:");
    println!("   - Ports define interfaces (the WHAT)");
    println!("   - Domain depends on ports, not implementations");
    println!("   - Custom methods extend base functionality");

    println!("\n💡 Why this matters:");
    println!("   - You can now implement this with ANY database");
    println!("   - Domain code never changes when switching databases");
    println!("   - Easy to mock for testing");

    println!("\n✅ Tutorial 02 Complete!");
    println!("Next: `cargo run --example tutorial_03_adapter`");

    Ok(())
}
