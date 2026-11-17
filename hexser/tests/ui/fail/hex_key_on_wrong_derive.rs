// R31 (PRD HEXSER-0.7 §12 A14): a key another derive reads is refused where it means nothing,
// instead of being ignored: only HexAdapter proves `implements`.
pub trait SinkPort {
  fn append(&mut self, fact: u32);
}

#[derive(hexser::HexPort)]
#[hex(port_trait = SinkPort)]
struct SinkPortNode;

#[derive(hexser::HexDomain)]
#[hex(implements(SinkPortNode))]
struct Ledger;

fn main() {}
