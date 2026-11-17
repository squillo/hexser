// R22 (PRD HEXSER-0.7 D5): an adapter that names a port it does not implement does not
// compile; the Implements edge is proved, never declared.
pub trait SinkPort {
  fn append(&mut self, fact: u32);
}

#[derive(hexser::HexPort)]
#[hex(port_trait = SinkPort)]
struct SinkPortNode;

#[derive(hexser::HexAdapter)]
#[hex(implements(SinkPortNode))]
struct NotASink;

fn main() {}
