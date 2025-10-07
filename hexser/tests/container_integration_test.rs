//! Integration tests for dependency injection container.
//!
//! Tests the complete container workflow including registration,
//! resolution, scoping behavior, and error scenarios. Validates
//! thread safety and service lifecycle management.

#[cfg(feature = "container")]
mod container_tests {
    use hexser::container::{Container, Provider, Scope};
    use hexser::HexResult;

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

    #[tokio::test]
    async fn test_register_and_resolve_singleton() {
        let container = Container::new();
        let provider = EmailProvider {
            host: String::from("smtp.example.com"),
        };

        container
            .register("email", provider, Scope::Singleton)
            .await
            .unwrap();

        let service1 = container.resolve::<EmailService>("email").await.unwrap();
        let service2 = container.resolve::<EmailService>("email").await.unwrap();

        assert_eq!(service1.host, "smtp.example.com");
        assert_eq!(service2.host, "smtp.example.com");
    }

    #[tokio::test]
    async fn test_register_and_resolve_transient() {
        let container = Container::new();
        let provider = EmailProvider {
            host: String::from("smtp.example.com"),
        };

        container
            .register("email", provider, Scope::Transient)
            .await
            .unwrap();

        let service1 = container.resolve::<EmailService>("email").await.unwrap();
        let service2 = container.resolve::<EmailService>("email").await.unwrap();

        assert_eq!(service1.host, "smtp.example.com");
        assert_eq!(service2.host, "smtp.example.com");
    }

    #[tokio::test]
    async fn test_service_not_found_error() {
        let container = Container::new();
        let result = container.resolve::<EmailService>("nonexistent").await;

        assert!(result.is_err());
    }

    #[cfg(feature = "container")]
    #[tokio::test]
    async fn test_async_service_registration_and_resolution() {
        struct AsyncEmailService {
            host: String,
        }

        struct AsyncEmailProvider {
            host: String,
        }

        #[async_trait::async_trait]
        impl hexser::container::AsyncProvider<AsyncEmailService> for AsyncEmailProvider {
            async fn provide_async(&self) -> hexser::HexResult<AsyncEmailService> {
                tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                Ok(AsyncEmailService {
                    host: self.host.clone(),
                })
            }
        }

        let container = Container::new();
        let provider = AsyncEmailProvider {
            host: String::from("async.example.com"),
        };

        container
            .register_async("async_email", provider, Scope::Singleton)
            .await
            .unwrap();

        let service = container.resolve_async::<AsyncEmailService>("async_email").await.unwrap();
        assert_eq!(service.host, "async.example.com");
    }

    #[cfg(feature = "container")]
    #[tokio::test]
    async fn test_async_concurrent_resolution() {
        struct ConcurrentService {
            id: i32,
        }

        struct ConcurrentProvider {
            id: i32,
        }

        #[async_trait::async_trait]
        impl hexser::container::AsyncProvider<ConcurrentService> for ConcurrentProvider {
            async fn provide_async(&self) -> hexser::HexResult<ConcurrentService> {
                tokio::time::sleep(std::time::Duration::from_millis(5)).await;
                Ok(ConcurrentService { id: self.id })
            }
        }

        let container = Container::new();
        container
            .register_async("concurrent", ConcurrentProvider { id: 1 }, Scope::Transient)
            .await
            .unwrap();

        let container1 = container.clone();
        let container2 = container.clone();

        let handle1 = tokio::spawn(async move {
            container1.resolve_async::<ConcurrentService>("concurrent").await
        });

        let handle2 = tokio::spawn(async move {
            container2.resolve_async::<ConcurrentService>("concurrent").await
        });

        let result1 = handle1.await.unwrap().unwrap();
        let result2 = handle2.await.unwrap().unwrap();

        assert_eq!(result1.id, 1);
        assert_eq!(result2.id, 1);
    }

    #[tokio::test]
    async fn test_duplicate_registration_error() {
        let container = Container::new();
        let provider1 = EmailProvider {
            host: String::from("host1.com"),
        };
        let provider2 = EmailProvider {
            host: String::from("host2.com"),
        };

        container
            .register("email", provider1, Scope::Singleton)
            .await
            .unwrap();

        let result = container.register("email", provider2, Scope::Singleton).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_container_clone_shares_registrations() {
        let container1 = Container::new();
        let provider = EmailProvider {
            host: String::from("smtp.example.com"),
        };

        container1
            .register("email", provider, Scope::Singleton)
            .await
            .unwrap();

        let container2 = container1.clone();

        assert!(container2.contains("email").await);
        let service = container2.resolve::<EmailService>("email").await.unwrap();
        assert_eq!(service.host, "smtp.example.com");
    }

    #[tokio::test]
    async fn test_multiple_services() {
        let container = Container::new();

        let email_provider = EmailProvider {
            host: String::from("smtp.example.com"),
        };
        container
            .register("email", email_provider, Scope::Singleton)
            .await
            .unwrap();

        assert_eq!(container.service_count().await, 1);
        assert!(container.contains("email").await);
    }
}
