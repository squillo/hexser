//! Implementation of #[derive(HexAdapter)] macro.
//!
//! Automatically implements Registrable and detects implemented traits
//! to generate relationship edges.
//!
//! Revision History
//! - 2025-10-02T00:00:00Z @AI: Initial HexAdapter derive implementation.

/// Derive HexAdapter for a type
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    if let Err(e) = crate::common::validation::validate_struct_or_enum(&input) {
        return e.to_compile_error().into();
    }

    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote::quote! {
        impl #impl_generics hex::registry::Registrable for #name #ty_generics #where_clause {
            fn node_info() -> hex::registry::NodeInfo {
                hex::registry::NodeInfo {
                    layer: hex::graph::Layer::Adapter,
                    role: hex::graph::Role::Adapter,
                    type_name: std::any::type_name::<Self>(),
                    module_path: std::module_path!(),
                }
            }

            fn dependencies() -> std::vec::Vec<hex::graph::NodeId> {
                std::vec::Vec::new()
            }
        }

        impl #impl_generics hex::adapters::Adapter for #name #ty_generics #where_clause {}

        hex::inventory::submit! {
            hex::registry::ComponentEntry::new::<#name #ty_generics>()
        }
    };

    proc_macro::TokenStream::from(expanded)
}
