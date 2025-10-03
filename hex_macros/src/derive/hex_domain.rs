//! Implementation of #[derive(HexDomain)] macro.
//!
//! Automatically implements Registrable trait and generates
//! inventory submission for domain layer types.
//!
//! Revision History
//! - 2025-10-02T00:00:00Z @AI: Initial HexDomain derive implementation.

/// Derive HexDomain for a type
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    if let Err(e) = crate::common::validation::validate_struct_or_enum(&input) {
        return e.to_compile_error().into();
    }

    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote::quote! {
        impl #impl_generics hexer::registry::Registrable for #name #ty_generics #where_clause {
            fn node_info() -> hexer::registry::NodeInfo {
                hexer::registry::NodeInfo {
                    layer: hexer::graph::Layer::Domain,
                    role: hexer::graph::Role::Entity,
                    type_name: std::any::type_name::<Self>(),
                    module_path: std::module_path!(),
                }
            }

            fn dependencies() -> std::vec::Vec<hexer::graph::NodeId> {
                std::vec::Vec::new()
            }
        }

        hexer::inventory::submit! {
            hexer::registry::ComponentEntry::new::<#name #ty_generics>()
        }
    };

    proc_macro::TokenStream::from(expanded)
}
