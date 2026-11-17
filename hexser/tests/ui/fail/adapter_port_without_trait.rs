// R23 (PRD HEXSER-0.7 D5): a port node that never named its trait cannot be the target of
// `implements(..)`, because nothing can check the adapter against it.
#[derive(hexser::HexPort)]
struct SinkPortNode;

#[derive(hexser::HexAdapter)]
#[hex(implements(SinkPortNode))]
struct MemorySink;

fn main() {}
