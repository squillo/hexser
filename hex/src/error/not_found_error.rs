//! Not found error type for missing resources.
//!
//! Provides NotFoundError struct for resource not found failures.
//! Not found errors occur when requested resources don't exist.
//! Includes resource type and identifier context.
//!
//! Revision History
//! - 2025-10-06T02:00:00Z @AI: Fix merge conflict duplicates.
//! - 2025-10-06T00:00:00Z @AI: Initial NotFoundError struct for Phase 1.

/// Not found error for missing resources
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotFoundError {
    /// Error code from codes::resource module
    pub code: String,
    /// Type of resource that wasn't found
    pub resource: String,
    /// Identifier of missing resource
    pub id: String,
    /// Optional source code location
    pub location: Option<crate::error::source_location::SourceLocation>,
}

impl NotFoundError {
    /// Create new not found error
    pub fn new(resource: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            code: String::from(crate::error::codes::resource::NOT_FOUND),
            resource: resource.into(),
            id: id.into(),
            location: None,
        }
    }

    /// Add source location (builder pattern)
    pub fn with_location(mut self, location: crate::error::source_location::SourceLocation) -> Self {
        self.location = Some(location);
        self
    }
}

impl std::fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error [{}]: {} not found with id '{}'", self.code, self.resource, self.id)?;
        write!(f, "\nNext Steps: Verify {} ID and existence", self.resource)?;

        if let Some(ref location) = self.location {
            write!(f, "\nSource: {}", location)?;
        }

        Ok(())
    }
}

impl std::error::Error for NotFoundError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_found_error_creation() {
        let err = NotFoundError::new("User", "123");
        assert_eq!(err.resource, "User");
        assert_eq!(err.id, "123");
        assert_eq!(err.code, crate::error::codes::resource::NOT_FOUND);
    }

    #[test]
    fn test_not_found_error_display() {
        let err = NotFoundError::new("Order", "abc-123");
        let display = format!("{}", err);
        assert!(display.contains("Order"));
        assert!(display.contains("abc-123"));
        assert!(display.contains(crate::error::codes::resource::NOT_FOUND));
    }
}
