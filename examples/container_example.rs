//! Dependency injection container example.
//!
//! Demonstrates zero-boilerplate service management with the hex
//! container. Shows registration of services with different scopes,
//! resolution with dependency injection, and proper error handling.
//!
//! Run with: cargo run --example container_example --features container

#[cfg(feature = "container")]
fn main() -> hex::HexResult<()> {
    println!("=== Dependency Injection Container Example ===\n");

    let container = hex::container::Container::new();

    println!("1. Registering services...");

    let config_provider = ConfigProvider {
        api_key: String::from("secret-key-123"),
    };
    container.register(
        "config",
        config_provider,
        hex::container::Scope::Singleton,
    )?;
    println!("   ✓ Config service registered (Singleton)");

    let email_provider = EmailProvider {
        host: String::from("smtp.example.com"),
    };
    container.register(
        "email",
        email_provider,
        hex::container::Scope::Transient,
    )?;
    println!("   ✓ Email service registered (Transient)");

    println!("\n2. Resolving services...");

    let config = container.resolve::<ConfigService>("config")?;
    println!("   ✓ Config resolved: API Key = {}", config.api_key);

    let email1 = container.resolve::<EmailService>("email")?;
    let email2 = container.resolve::<EmailService>("email")?;
    println!("   ✓ Email service resolved twice (Transient scope)");
    println!("     - Instance 1: {}", email1.host);
    println!("     - Instance 2: {}", email2.host);

    println!("\n3. Container info:");
    println!("   Total services registered: {}", container.service_count());
    println!("   Contains 'config': {}", container.contains("config"));
    println!("   Contains 'email': {}", container.contains("email"));

    println!("\n4. Cloning container (shares registrations)...");
    let container2 = container.clone();
    println!("   ✓ Cloned container service count: {}", container2.service_count());

    let config2 = container2.resolve::<ConfigService>("config")?;
    println!("   ✓ Resolved from cloned container: {}", config2.api_key);

    println!("\n✅ Container example completed!");

    Ok(())
}

#[cfg(not(feature = "container"))]
fn main() {
    println!("This example requires the 'container' feature.");
    println!("Run with: cargo run --example container_example --features container");
}

#[cfg(feature = "container")]
struct ConfigService {
    api_key: String,
}

#[cfg(feature = "container")]
struct ConfigProvider {
    api_key: String,
}

#[cfg(feature = "container")]
impl hex::container::Provider<ConfigService> for ConfigProvider {
    fn provide(&self) -> hex::HexResult<ConfigService> {
        Ok(ConfigService {
            api_key: self.api_key.clone(),
        })
    }
}

#[cfg(feature = "container")]
struct EmailService {
    host: String,
}

#[cfg(feature = "container")]
struct EmailProvider {
    host: String,
}

#[cfg(feature = "container")]
impl hex::container::Provider<EmailService> for EmailProvider {
    fn provide(&self) -> hex::HexResult<EmailService> {
        Ok(EmailService {
            host: self.host.clone(),
        })
    }
}
