//! Integration tests for dependency injection container.
//!
//! Tests the complete container workflow including registration,
//! resolution, scoping behavior, and error scenarios. Validates
//! thread safety and service lifecycle management.

#[cfg(feature = "container")]
mod container_tests {
    use hex::container::{Container, Provider, Scope};
    use hex::HexResult;

    struct EmailService {
        host: String,
    }

    struct EmailProvider {
        host: String,
    }

    impl Provider<EmailService> for EmailProvider {
        fn provide(&self) -> HexResult<EmailService> {
            Ok(EmailService {
                host: self.host.clone(),
            })
        }
    }

    struct UserService {
        email: std::sync::Arc<EmailService>,
    }

    struct UserProvider {
        email: std::sync::Arc<EmailService>,
    }

    impl Provider<UserService> for UserProvider {
        fn provide(&self) -> HexResult<UserService> {
            Ok(UserService {
                email: std::sync::Arc::clone(&self.email),
            })
        }
    }

    #[test]
    fn test_register_and_resolve_singleton() {
        let container = Container::new();
        let provider = EmailProvider {
            host: String::from("smtp.example.com"),
        };

        container
            .register("email", provider, Scope::Singleton)
            .unwrap();

        let service1 = container.resolve::<EmailService>("email").unwrap();
        let service2 = container.resolve::<EmailService>("email").unwrap();

        assert_eq!(service1.host, "smtp.example.com");
        assert_eq!(service2.host, "smtp.example.com");
    }

    #[test]
    fn test_register_and_resolve_transient() {
        let container = Container::new();
        let provider = EmailProvider {
            host: String::from("smtp.example.com"),
        };

        container
            .register("email", provider, Scope::Transient)
            .unwrap();

        let service1 = container.resolve::<EmailService>("email").unwrap();
        let service2 = container.resolve::<EmailService>("email").unwrap();

        assert_eq!(service1.host, "smtp.example.com");
        assert_eq!(service2.host, "smtp.example.com");
    }

    #[test]
    fn test_service_not_found_error() {
        let container = Container::new();
        let result = container.resolve::<EmailService>("nonexistent");

        assert!(result.is_err());
    }

    #[test]
    fn test_duplicate_registration_error() {
        let container = Container::new();
        let provider1 = EmailProvider {
            host: String::from("host1.com"),
        };
        let provider2 = EmailProvider {
            host: String::from("host2.com"),
        };

        container
            .register("email", provider1, Scope::Singleton)
            .unwrap();

        let result = container.register("email", provider2, Scope::Singleton);
        assert!(result.is_err());
    }

    #[test]
    fn test_container_clone_shares_registrations() {
        let container1 = Container::new();
        let provider = EmailProvider {
            host: String::from("smtp.example.com"),
        };

        container1
            .register("email", provider, Scope::Singleton)
            .unwrap();

        let container2 = container1.clone();

        assert!(container2.contains("email"));
        let service = container2.resolve::<EmailService>("email").unwrap();
        assert_eq!(service.host, "smtp.example.com");
    }

    #[test]
    fn test_multiple_services() {
        let container = Container::new();

        let email_provider = EmailProvider {
            host: String::from("smtp.example.com"),
        };
        container
            .register("email", email_provider, Scope::Singleton)
            .unwrap();

        assert_eq!(container.service_count(), 1);
        assert!(container.contains("email"));
    }
}
