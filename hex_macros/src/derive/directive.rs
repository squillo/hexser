//! Implementation of #[derive(HexDirective)] macro.
//!
//! Automatically implements the Directive trait for command/intent types.
//!
//! Revision History
//! - 2025-10-02T12:00:00Z @AI: Fix to implement validate method and add inventory submission.
//! - 2025-10-02T00:00:00Z @AI: Initial Directive derive implementation.

/// Derive Directive for a type
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote::quote! {
        impl #impl_generics hex::application::Directive for #name #ty_generics #where_clause {
            fn validate(&self) -> hex::HexResult<()> {
                Ok(())
            }
        }

        impl #impl_generics hex::registry::Registrable for #name #ty_generics #where_clause {
            fn node_info() -> hex::registry::NodeInfo {
                hex::registry::NodeInfo {
                    layer: hex::graph::Layer::Application,
                    role: hex::graph::Role::Directive,
                    type_name: std::any::type_name::<Self>(),
                    module_path: std::module_path!(),
                }
            }

            fn dependencies() -> std::vec::Vec<hex::graph::NodeId> {
                std::vec::Vec::new()
            }
        }

        inventory::submit! {
            hex::registry::ComponentEntry::new::<#name #ty_generics>()
        }
    };

    proc_macro::TokenStream::from(expanded)
}
