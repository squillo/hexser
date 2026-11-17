//! UI test: a registration derive on a generic type must fail to compile, not silently skip it.

fn main() {}

#[derive(hexser::HexDomain)]
struct GenericDomain<T> {
  inner: T,
}
