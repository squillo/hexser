//! Defines the Tag value object for the domain layer.
//!
//! The Tag represents a categorization label that can be applied to articles.
//! Tags are simple string values used for filtering and organizing content.
//!
//! Revision History
//! - 2025-10-09T22:14:00Z @AI: Initial implementation of Tag value object.

#[derive(hexser::HexDomain, hexser::HexValueItem, std::clone::Clone, std::fmt::Debug, std::cmp::PartialEq, std::cmp::Eq, std::hash::Hash)]
pub struct Tag {
    pub name: std::string::String,
}

impl Tag {
    pub fn new(name: std::string::String) -> Self {
        Self { name }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_tag_creation() {
        let tag = super::Tag::new(std::string::String::from("rust"));
        std::assert_eq!(tag.name, "rust");
    }

    #[test]
    fn test_tag_equality() {
        let tag1 = super::Tag::new(std::string::String::from("rust"));
        let tag2 = super::Tag::new(std::string::String::from("rust"));
        let tag3 = super::Tag::new(std::string::String::from("hexagonal"));
        std::assert_eq!(tag1, tag2);
        std::assert_ne!(tag1, tag3);
    }
}
