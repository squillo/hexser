//! The 0.6 to 0.7 migration guide names every public item 0.7 adds, deprecates or changes, and
//! each item it names exists in the source (PRD HEXSER-0.7 D11, row R30).
//!
//! The guide's code blocks are compiled and run as doctests (`hexser::MigratingFrom06To07`,
//! `#[cfg(doctest)]`). This cell is the other half: a doc walk that turns red when a section
//! is deleted, or when the guide and the code stop naming the same thing.
//!
//! Revision History
//! - 2026-11-17T18:04:19Z @AI: HEXSER-0.7 Q4 — section 8 names the three deprecated aliases and the 0.8.0 boundary, each checked against its source.
//! - 2026-11-17T16:31:10Z @AI: HEXSER-0.7 R30 — initial cell.

/// The guide, as shipped in the `hexser` package.
const GUIDE: &str = include_str!("../MIGRATING_0.6_TO_0.7.md");

/// One item 0.7 adds, deprecates or changes: the words its guide section must contain, the
/// source file that defines it, and the words that file must contain.
type Item = (&'static str, &'static str, &'static str);

/// The guide's sections, in order, each with the items it must name.
const SECTIONS: &[(&str, &[Item])] = &[
  (
    "## 1. `Actor` in place of `DirectiveHandler`",
    &[
      (
        "impl hexser::application::Actor for Deck",
        include_str!("../src/application/actor.rs"),
        "pub trait Actor",
      ),
      (
        "#[derive(hexser::HexActor)]",
        include_str!("../src/lib.rs"),
        "HexActor",
      ),
    ],
  ),
  (
    "## 2. `HexVocabulary` in place of a hand-written name list",
    &[
      (
        "hexser::domain::Vocabulary",
        include_str!("../src/domain/vocabulary.rs"),
        "pub trait Vocabulary",
      ),
      (
        "hexser::HexVocabulary",
        include_str!("../src/lib.rs"),
        "HexVocabulary",
      ),
    ],
  ),
  (
    "## 3. `HexRefusal` and `Hexserror::refused`",
    &[
      (
        "hexser::error::Refusal::next_steps",
        include_str!("../src/error/refusal.rs"),
        "pub trait Refusal",
      ),
      (
        "hexser::HexRefusal",
        include_str!("../src/lib.rs"),
        "HexRefusal",
      ),
      (
        "hexser::Hexserror::refused(",
        include_str!("../src/error/hex_error.rs"),
        "pub fn refused",
      ),
      (
        "err.code()",
        include_str!("../src/error/hex_error.rs"),
        "pub fn code",
      ),
    ],
  ),
  (
    "## 4. `port_trait` on a port marker",
    &[
      (
        "#[hex(role = \"OutputPort\", port_trait = SinkPort)]",
        include_str!("../src/lib.rs"),
        "HexPort",
      ),
      (
        "port_trait: RowsPort<u32>",
        include_str!("../src/templates/mod.rs"),
        "port_trait: $",
      ),
    ],
  ),
  (
    "## 5. `implements(..)` in place of a `depends_on` onto a port marker",
    &[
      (
        "#[hex(implements(SinkPortNode))]",
        include_str!("../src/lib.rs"),
        "HexAdapter",
      ),
      (
        "implements: [SinkPortNode]",
        include_str!("../src/templates/mod.rs"),
        "implements: [",
      ),
      (
        "hexser::registry::ImplementsEntry",
        include_str!("../src/registry/implements_entry.rs"),
        "pub struct ImplementsEntry",
      ),
      (
        "hexser::templates::implements_edge_exists::<",
        include_str!("../src/templates/mod.rs"),
        "pub fn implements_edge_exists",
      ),
    ],
  ),
  (
    "## 6. `depends_on(..)` on every registration derive",
    &[(
      "#[hex(depends_on(RepoPortNode))]",
      include_str!("../src/templates/mod.rs"),
      "pub fn dependency_edge_exists",
    )],
  ),
  (
    "## 7. `findings()` in a graph cell",
    &[
      (
        "graph.findings()",
        include_str!("../src/graph/hex_graph.rs"),
        "pub fn findings",
      ),
      (
        "ShortNameCollision { id, kept_type_name, kept_module_path, dropped_type_name, dropped_module_path }",
        include_str!("../src/graph/finding.rs"),
        "ShortNameCollision {",
      ),
      (
        "AdapterWithoutProvedPort { adapter }",
        include_str!("../src/graph/finding.rs"),
        "AdapterWithoutProvedPort {",
      ),
      (
        "EdgeToUnregisteredNode { source, target, relationship }",
        include_str!("../src/graph/finding.rs"),
        "EdgeToUnregisteredNode {",
      ),
    ],
  ),
  (
    "## 8. Four doors are deprecated",
    &[
      (
        "| `application::DirectiveHandler` | `application::Actor` |",
        include_str!("../src/application/directive_handler.rs"),
        "since = \"0.7.0\"",
      ),
      (
        "| `ports::UseCase` | `application::Actor` |",
        include_str!("../src/ports/use_case.rs"),
        "since = \"0.7.0\"",
      ),
      (
        "| `ports::InputPort` | `application::Actor` |",
        include_str!("../src/ports/input_port.rs"),
        "since = \"0.7.0\"",
      ),
      (
        "| `ports::Query` | `application::QueryHandler`",
        include_str!("../src/ports/query.rs"),
        "since = \"0.7.0\"",
      ),
      (
        "| `GraphBuilder::add_node` | `GraphBuilder::with_node`",
        include_str!("../src/graph/builder.rs"),
        "use `GraphBuilder::with_node`, the same call. Scheduled for removal in 0.8.0.",
      ),
      (
        "| `NodeId::from_type_name` | `NodeId::from_name`",
        include_str!("../src/graph/node_id.rs"),
        "use `NodeId::from_name`, the same hash. Scheduled for removal in 0.8.0.",
      ),
      (
        "scheduled for removal in 0.8.0",
        include_str!("../src/application/directive_handler.rs"),
        "Scheduled for removal in 0.8.0.",
      ),
    ],
  ),
  (
    "## 9. `#[hex(...)]` is read strictly",
    &[(
      "```rust,compile_fail",
      include_str!("ui/fail/hex_unknown_key.stderr"),
      "unknown `#[hex(...)]` key",
    )],
  ),
  (
    "## 10. The location switch is read once",
    &[(
      "`HEXSER_INCLUDE_SOURCE_LOCATION`",
      include_str!("../src/error/env_control.rs"),
      "std::sync::OnceLock",
    )],
  ),
  (
    "## 11. Nothing else moves",
    &[(
      "`#![forbid(unsafe_code)]`",
      include_str!("../src/lib.rs"),
      "#![forbid(unsafe_code)]",
    )],
  ),
];

/// Walk `guide`: every section present, in order, and each naming its items; every item still
/// defined in the source. The first failure, in words.
fn walk(guide: &str) -> std::result::Result<(), std::string::String> {
  let mut from = 0;
  for (heading, items) in SECTIONS {
    let at = guide[from..]
      .find(heading)
      .ok_or_else(|| std::format!("section {heading:?} is missing or out of order"))?;
    let start = from + at;
    let end = guide[start + heading.len()..]
      .find("\n## ")
      .map_or(guide.len(), |next| start + heading.len() + next);
    let section = &guide[start..end];
    for (in_guide, source, in_source) in *items {
      if !section.contains(in_guide) {
        return std::result::Result::Err(std::format!("{heading:?} no longer names {in_guide:?}"));
      }
      if !source.contains(in_source) {
        return std::result::Result::Err(std::format!(
          "{heading:?} names {in_guide:?}, but its source no longer contains {in_source:?}"
        ));
      }
    }
    from = end;
  }
  std::result::Result::Ok(())
}

/// why: R30. Every item 0.7 adds, deprecates or changes is named in its own section of the
/// guide, and the source still defines what the guide names. Deleting a section or an example,
/// or renaming an item without moving the guide, turns this red.
#[test]
fn the_migration_guide_names_every_new_public_item() {
  if let std::result::Result::Err(why) = walk(GUIDE) {
    std::panic!("{why}");
  }
}

/// why: the must-fail arm. A guide whose section 5 keeps its heading but loses its body fails
/// the walk, as does one with section 7 cut out entirely.
#[test]
fn a_guide_missing_a_section_or_its_body_fails_the_walk() {
  let five = GUIDE.find("## 5. ").expect("section 5 exists");
  let six = GUIDE.find("## 6. ").expect("section 6 exists");
  let heading_end = five + GUIDE[five..].find('\n').expect("a heading line");
  let hollow = std::format!("{}{}", &GUIDE[..heading_end + 1], &GUIDE[six..]);
  std::assert!(walk(&hollow).is_err());

  let seven = GUIDE.find("## 7. ").expect("section 7 exists");
  let eight = GUIDE.find("## 8. ").expect("section 8 exists");
  let cut = std::format!("{}{}", &GUIDE[..seven], &GUIDE[eight..]);
  std::assert!(walk(&cut).is_err());
}
