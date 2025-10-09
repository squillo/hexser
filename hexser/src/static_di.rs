//! Static (non-dyn) dependency wiring helpers.
//!
//! This module provides zero-cost, compile-time friendly wiring utilities
//! that avoid dynamic dispatch and are WASM-friendly (no threads, no tokio).
//! Use it when you want to construct your application graph explicitly at
//! compile time without any `dyn` indirection or runtime containers.
//!
//! Quick example:
//!
//! ```rust
//! use hexser::prelude::*;
//!
//! #[derive(Clone, Debug)]
//! struct Repo;
//! #[derive(Clone, Debug)]
//! struct Service { repo: Repo }
//!
//! let app = hexser::hex_static!({
//!     let repo = Repo;
//!     let service = Service { repo: repo.clone() };
//!     (repo, service)
//! });
//!
//! let (repo, service) = app.into_inner();
//! assert_eq!(format!("{:?}", service.repo), format!("{:?}", repo));
//! ```
//!
//! The `hex_static!` macro builds your object graph once and stores it inside
//! a `StaticContainer<T>`. It uses zero dynamic dispatch and no async runtime.

/// A minimal, zero-cost container that owns a fully built object graph.
#[derive(Debug, Clone)]
pub struct StaticContainer<T> {
  value: T,
}

impl<T> StaticContainer<T> {
  /// Wrap an already-built value.
  pub fn new(value: T) -> Self {
    Self { value }
  }

  /// Borrow the inner value.
  pub fn get(&self) -> &T {
    &self.value
  }

  /// Borrow the inner value mutably.
  pub fn get_mut(&mut self) -> &mut T {
    &mut self.value
  }

  /// Consume and return the inner value.
  pub fn into_inner(self) -> T {
    self.value
  }

  /// Transform the inner value into another, returning a new container.
  pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> StaticContainer<U> {
    StaticContainer {
      value: f(self.value),
    }
  }
}

/// A generic, inlineable builder that constructs a value using a closure.
///
/// This avoids any trait objects (`dyn`) and compiles down to direct calls.
#[derive(Debug, Clone, Copy)]
pub struct StaticBuilder<T, F>
where
  F: FnOnce() -> T,
{
  factory: F,
}

impl<T, F> StaticBuilder<T, F>
where
  F: FnOnce() -> T,
{
  /// Create a new static builder from a zero-arg closure.
  pub fn new(factory: F) -> Self {
    Self { factory }
  }

  /// Build the value and wrap it in a StaticContainer.
  pub fn build(self) -> StaticContainer<T> {
    StaticContainer {
      value: (self.factory)(),
    }
  }
}

/// Build a StaticContainer from an explicit construction block.
///
/// The block executes once and returns the fully built root value. The macro
/// infers the resulting type automatically. It never uses `dyn`, is `no_std`
/// friendly (when your types are), and works on wasm32-unknown-unknown.
#[macro_export]
macro_rules! hex_static {
  ($block:block) => {{
    let value = (|| $block)();
    $crate::static_di::StaticContainer::new(value)
  }};
}

#[cfg(test)]
mod tests {
  use super::*;

  #[derive(Clone, Debug, PartialEq, Eq)]
  struct A(u8);
  #[derive(Clone, Debug, PartialEq, Eq)]
  struct B {
    a: A,
  }

  #[test]
  fn builds_static_graph() {
    let app = crate::hex_static!({
      let a = A(7);
      let b = B { a: a.clone() };
      (a, b)
    });
    let (a, b) = app.into_inner();
    assert_eq!(b.a, a);
  }

  #[test]
  fn builder_works() {
    let c = StaticBuilder::new(|| 42).build();
    assert_eq!(*c.get(), 42);
    let d = c.map(|x| x + 1);
    assert_eq!(d.into_inner(), 43);
  }
}
