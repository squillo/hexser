//! UI test: #[derive(HexEntity)] on a struct without an `id` field must fail to compile.

fn main() {}

#[derive(hexser::HexEntity)]
struct NoIdField {
  name: String,
}
