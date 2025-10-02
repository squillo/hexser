//! Tutorial 01: Your First Entity
//!
//! Learn to create domain entities - the foundation of hexagonal architecture.
//! Time: 5 minutes
//!
//! Run with: `cargo run --example tutorial_01_entity`

fn main() -> hex::HexResult<()> {
    println!("=== Tutorial 01: Your First Entity ===\n");

    // Step 1: Define a domain entity
    // Entities have identity and represent core business concepts
    #[derive(Clone, Debug)]
    struct User {
        id: String,
        email: String,
        name: String,
    }

    // Step 2: Implement the Entity trait
    // This gives your type hexagonal architecture superpowers!
    impl hex::domain::Entity for User {
        type Id = String;  // Define what uniquely identifies this entity
    }

    // Step 3: Create and use your entity
    let user = User {
        id: String::from("user_001"),
        email: String::from("alice@example.com"),
        name: String::from("Alice"),
    };

    println!("✅ Created entity:");
    println!("   ID: {}", user.id);
    println!("   Name: {}", user.name);
    println!("   Email: {}", user.email);

    // Step 4: Understand what you've built
    println!("\n📚 What you learned:");
    println!("   - Domain entities represent business concepts");
    println!("   - Entities have unique identity (ID)");
    println!("   - Entities live in the Domain layer (no dependencies!)");

    println!("\n🎯 Key Concepts:");
    println!("   ┌─────────────┐");
    println!("   │   Domain    │  ← Your User entity lives here");
    println!("   │   [User]    │  ← Pure business logic, no tech dependencies");
    println!("   └─────────────┘");

    println!("\n✅ Tutorial 01 Complete!");
    println!("Next: `cargo run --example tutorial_02_repository`");

    Ok(())
}
