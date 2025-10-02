//! Container-specific error types.
//!
//! Defines errors that can occur during dependency injection container
//! operations such as service registration, resolution, and lifecycle
//! management. All errors follow ERRORS_PROMPT.md guidelines with
//! actionable remediation steps.
//!
//! Revision History
//! - 2025-10-02T20:00:00Z @AI: Initial container error types for Phase 6.

/// Errors specific to dependency injection container operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContainerError {
    /// Service not registered in container
    ServiceNotFound {
        /// Name of missing service type
        service_name: String,
    },

    /// Circular dependency detected
    CircularDependency {
        /// Chain of dependencies forming cycle
        cycle: Vec<String>,
    },

    /// Service provider failed to create instance
    ProviderFailed {
        /// Name of service that failed
        service_name: String,
        /// Reason for failure
        reason: String,
    },

    /// Service already registered
    DuplicateRegistration {
        /// Name of already registered service
        service_name: String,
    },
}

impl ContainerError {
    /// Create service not found error
    pub fn service_not_found(service_name: impl Into<String>) -> Self {
        Self::ServiceNotFound {
            service_name: service_name.into(),
        }
    }

    /// Create circular dependency error
    pub fn circular_dependency(cycle: Vec<String>) -> Self {
        Self::CircularDependency { cycle }
    }

    /// Create provider failed error
    pub fn provider_failed(service_name: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::ProviderFailed {
            service_name: service_name.into(),
            reason: reason.into(),
        }
    }

    /// Create duplicate registration error
    pub fn duplicate_registration(service_name: impl Into<String>) -> Self {
        Self::DuplicateRegistration {
            service_name: service_name.into(),
        }
    }
}

impl std::fmt::Display for ContainerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ServiceNotFound { service_name } => {
                write!(f, "Service not found: {}", service_name)?;
                write!(f, "\nNext Steps: Register the service before resolving")?;
                write!(f, "\nSuggestion: container.register::<{}>(provider, scope)?", service_name)
            }
            Self::CircularDependency { cycle } => {
                write!(f, "Circular dependency detected: {}", cycle.join(" -> "))?;
                write!(f, "\nNext Steps: Break the cycle by introducing an interface or removing dependency")?;
                write!(f, "\nSuggestion: Use dependency inversion to break circular references")
            }
            Self::ProviderFailed { service_name, reason } => {
                write!(f, "Provider failed for {}: {}", service_name, reason)?;
                write!(f, "\nNext Steps: Check provider implementation and dependencies")?;
                write!(f, "\nSuggestion: Ensure all dependencies are registered and provider logic is correct")
            }
            Self::DuplicateRegistration { service_name } => {
                write!(f, "Service already registered: {}", service_name)?;
                write!(f, "\nNext Steps: Remove duplicate registration or use different service name")?;
                write!(f, "\nSuggestion: Check if service is registered elsewhere in the application")
            }
        }
    }
}

impl std::error::Error for ContainerError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_not_found_error() {
        let err = ContainerError::service_not_found("UserService");
        assert_eq!(
            err,
            ContainerError::ServiceNotFound {
                service_name: String::from("UserService")
            }
        );
        assert!(err.to_string().contains("Service not found"));
        assert!(err.to_string().contains("Register the service"));
    }

    #[test]
    fn test_circular_dependency_error() {
        let cycle = vec![
            String::from("A"),
            String::from("B"),
            String::from("A"),
        ];
        let err = ContainerError::circular_dependency(cycle.clone());
        assert_eq!(err, ContainerError::CircularDependency { cycle });
        assert!(err.to_string().contains("Circular dependency"));
    }

    #[test]
    fn test_provider_failed_error() {
        let err = ContainerError::provider_failed("TestService", "Invalid state");
        assert!(err.to_string().contains("Provider failed"));
        assert!(err.to_string().contains("Invalid state"));
    }

    #[test]
    fn test_duplicate_registration_error() {
        let err = ContainerError::duplicate_registration("MyService");
        assert!(err.to_string().contains("already registered"));
    }
}
