//! Implementation of `#[derive(HexVocabulary)]` and `#[derive(HexRefusal)]` (PRD HEXSER-0.7
//! D2, D3).
//!
//! Both name the variants of a closed enum in declaration order. `HexVocabulary` implements
//! `::hexser::domain::Vocabulary` (`CODES`, `code()`), and on an all-unit enum an inherent
//! `from_code(&str) -> Option<Self>` that answers `None` for an unknown code. `HexRefusal` does
//! the same and implements `::hexser::error::Refusal`, with any static guidance written per
//! variant as `#[hex(next_step = "...", suggestion = "...")]`. `#[hex(code = "...")]` on a
//! variant replaces its name as its code. Two variants with one code are refused, since the code
//! could not read back. Neither derive writes `Display` or `Default`: the words for a value are
//! said at the edge, and there is no default refusal.
//!
//! A type-level `#[hex(...)]` belongs to a registration derive beside this one (Taxa derives
//! `HexDomain` on its refusal vocabularies), so those keys are left to it here.
//!
//! Revision History
//! - 2026-11-17T16:31:10Z @AI: HEXSER-0.7 D2, D3 — initial derives, one shared expansion.

/// Derive HexVocabulary for a closed enum
pub fn derive_vocabulary(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input = syn::parse_macro_input!(input as syn::DeriveInput);
  proc_macro::TokenStream::from(expand(&input, "HexVocabulary", false))
}

/// Derive HexRefusal for a closed enum
pub fn derive_refusal(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input = syn::parse_macro_input!(input as syn::DeriveInput);
  proc_macro::TokenStream::from(expand(&input, "HexRefusal", true))
}

/// One variant as the expansion needs it.
struct Named {
  ident: syn::Ident,
  code: syn::LitStr,
  pattern: proc_macro2::TokenStream,
  unit: bool,
  next_steps: Vec<syn::LitStr>,
  suggestions: Vec<syn::LitStr>,
}

fn expand(input: &syn::DeriveInput, derive_name: &str, refusal: bool) -> proc_macro2::TokenStream {
  match try_expand(input, derive_name, refusal) {
    Ok(tokens) => tokens,
    Err(error) => error.to_compile_error(),
  }
}

fn try_expand(
  input: &syn::DeriveInput,
  derive_name: &str,
  refusal: bool,
) -> syn::Result<proc_macro2::TokenStream> {
  crate::common::attrs::parse(&input.attrs)?.admit(
    derive_name,
    &[],
    &[
      crate::common::attrs::Key::Role,
      crate::common::attrs::Key::DependsOn,
      crate::common::attrs::Key::Implements,
      crate::common::attrs::Key::PortTrait,
    ],
  )?;

  let name = &input.ident;
  let data = match &input.data {
    syn::Data::Enum(data) => data,
    syn::Data::Struct(_) | syn::Data::Union(_) => {
      return Err(syn::Error::new_spanned(
        name,
        format!(
          "`#[derive({derive_name})]` names the variants of a closed enum, and `{name}` is not \
           an enum. Write the closed set as one: `enum {name} {{ First, Second }}`"
        ),
      ));
    }
  };

  let variant_keys: &[crate::common::attrs::Key] = if refusal {
    &[
      crate::common::attrs::Key::Code,
      crate::common::attrs::Key::NextStep,
      crate::common::attrs::Key::Suggestion,
    ]
  } else {
    &[crate::common::attrs::Key::Code]
  };

  let mut named: Vec<Named> = Vec::new();
  for variant in &data.variants {
    let attrs = crate::common::attrs::parse(&variant.attrs)?;
    attrs.admit(derive_name, variant_keys, &[])?;
    let ident = variant.ident.clone();
    let code = attrs
      .code
      .clone()
      .unwrap_or_else(|| syn::LitStr::new(&ident.to_string(), ident.span()));
    if let Some(earlier) = named.iter().find(|n| n.code.value() == code.value()) {
      return Err(syn::Error::new_spanned(
        &variant.ident,
        format!(
          "two variants of `{name}` answer the code {:?} (`{}` and `{ident}`), so the code could \
           not read back to one of them; give one a distinct `#[hex(code = \"...\")]`",
          code.value(),
          earlier.ident
        ),
      ));
    }
    let (pattern, unit) = match &variant.fields {
      syn::Fields::Unit => (quote::quote!(Self::#ident), true),
      syn::Fields::Unnamed(_) => (quote::quote!(Self::#ident(..)), false),
      syn::Fields::Named(_) => (quote::quote!(Self::#ident { .. }), false),
    };
    named.push(Named {
      ident,
      code,
      pattern,
      unit,
      next_steps: attrs.next_steps,
      suggestions: attrs.suggestions,
    });
  }

  let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
  let codes: Vec<&syn::LitStr> = named.iter().map(|n| &n.code).collect();
  let patterns: Vec<&proc_macro2::TokenStream> = named.iter().map(|n| &n.pattern).collect();

  let code_body = if named.is_empty() {
    quote::quote!(match *self {})
  } else {
    quote::quote!(match self { #( #patterns => #codes, )* })
  };

  let vocabulary = quote::quote! {
    impl #impl_generics ::hexser::domain::Vocabulary for #name #ty_generics #where_clause {
      const CODES: &'static [&'static str] = &[#( #codes ),*];

      fn code(&self) -> &'static str {
        #code_body
      }
    }
  };

  let from_code = if named.iter().all(|n| n.unit) {
    let idents = named.iter().map(|n| &n.ident);
    quote::quote! {
      impl #impl_generics #name #ty_generics #where_clause {
        /// The value that answers `code`, or `None` when no variant does: an unknown code is a
        /// value, never a guessed variant.
        pub fn from_code(code: &str) -> ::core::option::Option<Self> {
          match code {
            #( #codes => ::core::option::Option::Some(Self::#idents), )*
            _ => ::core::option::Option::None,
          }
        }
      }
    }
  } else {
    proc_macro2::TokenStream::new()
  };

  let refusal_impl = if refusal {
    let guidance = |pick: fn(&Named) -> &Vec<syn::LitStr>, method: proc_macro2::TokenStream| {
      if named.iter().all(|n| pick(n).is_empty()) {
        return proc_macro2::TokenStream::new();
      }
      let lists = named.iter().map(|n| {
        let items = pick(n);
        quote::quote!(&[#( #items ),*])
      });
      quote::quote! {
        fn #method(&self) -> &'static [&'static str] {
          match self { #( #patterns => #lists, )* }
        }
      }
    };
    let next_steps = guidance(|n| &n.next_steps, quote::quote!(next_steps));
    let suggestions = guidance(|n| &n.suggestions, quote::quote!(suggestions));
    quote::quote! {
      impl #impl_generics ::hexser::error::Refusal for #name #ty_generics #where_clause {
        #next_steps
        #suggestions
      }
    }
  } else {
    proc_macro2::TokenStream::new()
  };

  Ok(quote::quote! {
    #vocabulary
    #from_code
    #refusal_impl
  })
}
