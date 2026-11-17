// R31 (PRD HEXSER-0.7 §12 A14): an unknown `#[hex(...)]` key is a compile error naming the keys
// hexser reads. In 0.6 a malformed attribute was ignored, so a key after it was dropped.
#[derive(hexser::HexDomain)]
#[hex(rol = "ValueObject")]
struct Money;

fn main() {}
