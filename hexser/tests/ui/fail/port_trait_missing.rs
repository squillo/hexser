// R21 (PRD HEXSER-0.7 D4, PRD-886 T67): a port node names its trait, so a port node whose trait
// was deleted (or never existed) does not compile.
#[derive(hexser::HexPort)]
#[hex(port_trait = DeletedSinkPort)]
struct SinkPortNode;

fn main() {}
