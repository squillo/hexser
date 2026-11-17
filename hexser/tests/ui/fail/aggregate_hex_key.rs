// R31 (PRD HEXSER-0.7 §12 A14): `HexAggregate` declares `#[hex]` but reads no key of its own.
// A key no derive on this type reads is refused, never ignored: `code` belongs on an enum
// variant under HexVocabulary or HexRefusal, so here it would have been dropped in silence.

#[derive(hexser::HexAggregate)]
#[hex(code = "ORDER")]
struct Order {
  id: String,
}

impl hexser::HexEntity for Order {
  type Id = String;
}

fn main() {
  let _ = Order { id: String::new() }.id;
}
