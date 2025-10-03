//! Implementation of #[derive(HexPort)] macro.
//!
//! For traits, generates a companion meta struct that implements Registrable.
//! For structs, marks them as port layer types.
//!
//! Revision History
//! - 2025-10-02T00:00:00Z @AI: Initial HexPort derive implementation.

/// Derive HexPort for a type
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote::quote! {
        impl #impl_generics hexer::registry::Registrable for #name #ty_generics #where_clause {
            fn node_info() -> hexer::registry::NodeInfo {
                hexer::registry::NodeInfo {
                    layer: hexer::graph::Layer::Port,
                    role: hexer::graph::Role::Repository,
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
