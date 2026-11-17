// R17 (PRD HEXSER-0.7 D2): a vocabulary is a closed enum; a struct has no variants to name.
#[derive(hexser::HexVocabulary)]
struct NotAnEnum {
  code: &'static str,
}

fn main() {}
