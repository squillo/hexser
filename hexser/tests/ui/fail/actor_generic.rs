// R12 (PRD HEXSER-0.7 D2): `#[derive(HexActor)]` on a generic type is refused with the
// marker-struct remedy, as every registration derive is since 0.6.0.
#[derive(hexser::HexActor)]
struct DeckActor<S> {
  sink: S,
}

fn main() {}
