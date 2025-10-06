//! Template framework for quickly scaffolding hexer components.
//!
//! This module provides lightweight helpers and macros to implement
//! Registrable for your components without relying on derive macros.
//! It complements proc-macro derives by offering simple, explicit
//! building blocks you can use in any context (including no-macros builds).
//!
//! # Quick examples
//!
//! ```rust
//! use hexer::prelude::*;
//! use hexer::hex_register_domain;
//!
//! struct MyEntity { id: u64 }
//!
//! // Implement Registrable for a domain Entity using a template macro
//! hexer::hex_register_domain!(MyEntity, Role::Entity);
//!
//! // Now you can obtain node info for graph registration
//! let info = <MyEntity as Registrable>::node_info();
//! assert_eq!(info.layer, Layer::Domain);
//! assert_eq!(info.role, Role::Entity);
//! ```
//!
//! ```rust
//! use hexer::prelude::*;
//! use hexer::hex_register_adapter;
//!
//! struct PgUserRepo;
//!
//! // Register as an Adapter implementing a Repository
//! hexer::hex_register_adapter!(PgUserRepo, Role::Adapter);
//! ```
//!
//! These helpers are intended as templates: copy, adapt, and extend as needed.

/// Split a fully-qualified Rust type path into (module_path, type_name).
///
/// For example: "my_crate::module::Type" -> ("my_crate::module", "Type").
/// If the input has no module qualifiers, module_path is an empty string.
pub fn split_type_name(full: &'static str) -> (&'static str, &'static str) {
    match full.rfind("::") {
        Some(idx) => (&full[..idx], &full[idx + 2..]),
        None => ("", full),
    }
}

/// Core macro to implement Registrable for a type with a specific layer and role.
#[macro_export]
macro_rules! hex_register_component {
    ($t:ty, $layer:expr, $role:expr) => {
        impl $crate::registry::Registrable for $t {
            fn node_info() -> $crate::registry::NodeInfo {
                let full = ::std::any::type_name::<Self>();
                let (module_path, type_name) = $crate::templates::split_type_name(full);
                $crate::registry::NodeInfo::new($layer, $role, type_name, module_path)
            }
            fn dependencies() -> ::std::vec::Vec<$crate::graph::NodeId> {
                ::std::vec![]
            }
        }
    };
}

/// Convenience macro for Domain-layer components.
#[macro_export]
macro_rules! hex_register_domain {
    ($t:ty, $role:expr) => {
        $crate::hex_register_component!($t, $crate::graph::Layer::Domain, $role);
    };
}

/// Convenience macro for Port-layer components.
#[macro_export]
macro_rules! hex_register_port {
    ($t:ty, $role:expr) => {
        $crate::hex_register_component!($t, $crate::graph::Layer::Port, $role);
    };
}

/// Convenience macro for Adapter-layer components.
#[macro_export]
macro_rules! hex_register_adapter {
    ($t:ty, $role:expr) => {
        $crate::hex_register_component!($t, $crate::graph::Layer::Adapter, $role);
    };
}

/// Convenience macro for Application-layer components.
#[macro_export]
macro_rules! hex_register_application {
    ($t:ty, $role:expr) => {
        $crate::hex_register_component!($t, $crate::graph::Layer::Application, $role);
    };
}

/// Convenience macro for Infrastructure-layer components.
#[macro_export]
macro_rules! hex_register_infrastructure {
    ($t:ty, $role:expr) => {
        $crate::hex_register_component!($t, $crate::graph::Layer::Infrastructure, $role);
    };
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    struct TDomain;
    struct TAdapter;

    // Use the macros to implement Registrable
    hex_register_domain!(TDomain, Role::Entity);
    hex_register_adapter!(TAdapter, Role::Adapter);

    #[test]
    fn test_domain_template_macro() {
        let info = <TDomain as Registrable>::node_info();
        assert_eq!(info.layer, Layer::Domain);
        assert_eq!(info.role, Role::Entity);
        assert_eq!(info.type_name, "TDomain");
    }

    #[test]
    fn test_adapter_template_macro() {
        let info = <TAdapter as Registrable>::node_info();
        assert_eq!(info.layer, Layer::Adapter);
        assert_eq!(info.role, Role::Adapter);
        assert_eq!(info.type_name, "TAdapter");
    }

    #[test]
    fn test_split_type_name() {
        let (m, n) = super::split_type_name("foo::bar::Baz");
        assert_eq!(m, "foo::bar");
        assert_eq!(n, "Baz");
        let (m2, n2) = super::split_type_name("Baz");
        assert_eq!(m2, "");
        assert_eq!(n2, "Baz");
    }
}
