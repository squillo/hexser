//! The one reader of `#[hex(...)]`, for every hexser derive (PRD HEXSER-0.7 §12 A14).
//!
//! Until 0.7 each derive read only `role`, and a malformed or unknown key was ignored: the
//! parse error was discarded, so a key written after it was dropped too and the derive applied
//! its default in silence. 0.7 adds keys (`depends_on`, `implements`, `port_trait`, and on enum
//! variants `code`, `next_step`, `suggestion`), so this reader parses EVERY key hexser knows,
//! refuses an unknown key or a malformed value with a compile error, and lets each derive say
//! which keys it reads and which it leaves to a sibling derive on the same type.
//!
//! Revision History
//! - 2026-11-17T16:31:10Z @AI: HEXSER-0.7 — initial strict reader; replaces `role_override`.

/// A key hexser reads inside `#[hex(...)]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
  /// `role = "Variant"` on a registered type.
  Role,
  /// `depends_on(Type, ..)` on a registered type.
  DependsOn,
  /// `implements(PortNode, ..)` on a `HexAdapter`.
  Implements,
  /// `port_trait = path::Trait` on a `HexPort`.
  PortTrait,
  /// `code = "..."` on an enum variant.
  Code,
  /// `next_step = "..."` on a refusal variant (repeatable).
  NextStep,
  /// `suggestion = "..."` on a refusal variant (repeatable).
  Suggestion,
}

impl Key {
  fn name(self) -> &'static str {
    match self {
      Key::Role => "role",
      Key::DependsOn => "depends_on",
      Key::Implements => "implements",
      Key::PortTrait => "port_trait",
      Key::Code => "code",
      Key::NextStep => "next_step",
      Key::Suggestion => "suggestion",
    }
  }

  /// Which derive reads this key, for the refusal's words.
  fn reader(self) -> &'static str {
    match self {
      Key::Role | Key::DependsOn => {
        "the registration derives (HexDomain, HexPort, HexAdapter, HexDirective, HexQuery, HexActor)"
      }
      Key::Implements => "HexAdapter",
      Key::PortTrait => "HexPort",
      Key::Code => "HexVocabulary and HexRefusal, on an enum variant",
      Key::NextStep | Key::Suggestion => "HexRefusal, on an enum variant",
    }
  }
}

/// Every `#[hex(...)]` key on one item, parsed.
#[derive(Default)]
pub struct HexAttrs {
  /// `role = "..."`.
  pub role: Option<syn::LitStr>,
  /// `depends_on(..)`, in order.
  pub depends_on: Vec<syn::Path>,
  /// `implements(..)`, in order.
  pub implements: Vec<syn::Path>,
  /// `port_trait = ...`.
  pub port_trait: Option<syn::TraitBound>,
  /// `code = "..."`.
  pub code: Option<syn::LitStr>,
  /// `next_step = "..."`, in order.
  pub next_steps: Vec<syn::LitStr>,
  /// `suggestion = "..."`, in order.
  pub suggestions: Vec<syn::LitStr>,
  /// Each key as written, with its span, for `admit`.
  seen: Vec<(Key, proc_macro2::Span)>,
}

/// Parse every `#[hex(...)]` attribute on one item. An unknown key, a malformed value, or a
/// single-valued key written twice is a compile error at the offending tokens.
pub fn parse(attrs: &[syn::Attribute]) -> syn::Result<HexAttrs> {
  let mut out = HexAttrs::default();
  for attr in attrs {
    if !attr.path().is_ident("hex") {
      continue;
    }
    attr.parse_nested_meta(|meta| {
      let span = syn::spanned::Spanned::span(&meta.path);
      let key = if meta.path.is_ident("role") {
        let lit: syn::LitStr = meta.value()?.parse()?;
        set_once(&mut out.role, lit, span, "role")?;
        Key::Role
      } else if meta.path.is_ident("depends_on") {
        out.depends_on.extend(path_list(&meta)?);
        Key::DependsOn
      } else if meta.path.is_ident("implements") {
        out.implements.extend(path_list(&meta)?);
        Key::Implements
      } else if meta.path.is_ident("port_trait") {
        let bound: syn::TraitBound = meta.value()?.parse()?;
        set_once(&mut out.port_trait, bound, span, "port_trait")?;
        Key::PortTrait
      } else if meta.path.is_ident("code") {
        let lit: syn::LitStr = meta.value()?.parse()?;
        set_once(&mut out.code, lit, span, "code")?;
        Key::Code
      } else if meta.path.is_ident("next_step") {
        out.next_steps.push(meta.value()?.parse()?);
        Key::NextStep
      } else if meta.path.is_ident("suggestion") {
        out.suggestions.push(meta.value()?.parse()?);
        Key::Suggestion
      } else {
        return Err(meta.error(
          "unknown `#[hex(...)]` key. hexser reads `role`, `depends_on(..)`, `implements(..)` \
           and `port_trait` on a type, and `code`, `next_step` and `suggestion` on an enum \
           variant",
        ));
      };
      out.seen.push((key, span));
      Ok(())
    })?;
  }
  Ok(out)
}

impl HexAttrs {
  /// Check the keys written against what `derive_name` reads (`owns`) and what it leaves to a
  /// sibling derive on the same item (`tolerates`). Any other key is a compile error naming
  /// the derive that does read it, instead of a key ignored in silence.
  pub fn admit(&self, derive_name: &str, owns: &[Key], tolerates: &[Key]) -> syn::Result<()> {
    for (key, span) in &self.seen {
      if owns.contains(key) || tolerates.contains(key) {
        continue;
      }
      let reads: Vec<String> = owns.iter().map(|k| format!("`{}`", k.name())).collect();
      let reads = if reads.is_empty() {
        String::from("no `#[hex(...)]` key here")
      } else {
        reads.join(", ")
      };
      return Err(syn::Error::new(
        *span,
        format!(
          "`#[derive({derive_name})]` does not read `#[hex({})]` (it reads {reads}); `{}` is \
           read by {}",
          key.name(),
          key.name(),
          key.reader()
        ),
      ));
    }
    Ok(())
  }

  /// The `::hexser::graph::Role::Variant` tokens for `role`, when written. An unknown variant
  /// is a compile error at the `Role::` path, pointing at the user's string.
  pub fn role_tokens(&self) -> Option<proc_macro2::TokenStream> {
    self.role.as_ref().map(|lit| {
      let ident = syn::Ident::new(&lit.value(), lit.span());
      quote::quote!(::hexser::graph::Role::#ident)
    })
  }
}

fn set_once<T>(
  slot: &mut Option<T>,
  value: T,
  span: proc_macro2::Span,
  key: &str,
) -> syn::Result<()> {
  if slot.is_some() {
    return Err(syn::Error::new(
      span,
      format!("`#[hex({key})]` is written twice; write it once"),
    ));
  }
  *slot = Some(value);
  Ok(())
}

fn path_list(meta: &syn::meta::ParseNestedMeta<'_>) -> syn::Result<Vec<syn::Path>> {
  let content;
  syn::parenthesized!(content in meta.input);
  let paths = syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated(&content)?;
  if paths.is_empty() {
    return Err(meta.error("name at least one type inside the parentheses"));
  }
  Ok(paths.into_iter().collect())
}
