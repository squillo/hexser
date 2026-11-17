//! `hexser::application::Actor`, the single writer's door, and `#[derive(HexActor)]`
//! (PRD HEXSER-0.7 D1, rows R09 and R11).
//!
//! Three fixtures carry the shapes the family's writers actually have (PRD §4): a Deck-shaped
//! actor that returns `Result<Event, Refusal>` and holds its ports, a Tables-shaped actor that
//! returns `()` and appends 0..N facts, and a Flo-shaped actor that borrows a per-call context.
//! One trait must fit all three without a lock, a forced `Result`, or `Hexserror`.
//!
//! Revision History
//! - 2026-11-17T16:31:10Z @AI: HEXSER-0.7 R09, R11 — initial cells, written red.

#[derive(Debug, Clone, PartialEq, Eq)]
enum DeckChange {
  Rename(std::string::String),
  Clear,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DeckEvent {
  seq: u64,
  change: DeckChange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ChangeRefusal {
  EmptyName,
  AlreadyClear,
}

/// Deck-shaped: holds its state, returns the event it decided or a closed refusal.
struct DeckLike {
  seq: u64,
  name: std::string::String,
}

impl hexser::application::Actor for DeckLike {
  type Command = DeckChange;
  type Context<'a> = ();
  type Outcome = std::result::Result<DeckEvent, ChangeRefusal>;

  fn handle(&mut self, command: DeckChange, _context: ()) -> Self::Outcome {
    match &command {
      DeckChange::Rename(name) if name.is_empty() => {
        return std::result::Result::Err(ChangeRefusal::EmptyName);
      }
      DeckChange::Rename(name) => self.name = name.clone(),
      DeckChange::Clear if self.name.is_empty() => {
        return std::result::Result::Err(ChangeRefusal::AlreadyClear);
      }
      DeckChange::Clear => self.name.clear(),
    }
    self.seq += 1;
    std::result::Result::Ok(DeckEvent {
      seq: self.seq,
      change: command,
    })
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TablesCommand {
  AddTwo,
  Refuse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Fact {
  Added(u32),
  Refused,
}

/// Tables-shaped: returns `()`; every outcome, refusals included, is a fact it appends.
struct TablesLike {
  facts: std::vec::Vec<Fact>,
}

impl hexser::application::Actor for TablesLike {
  type Command = TablesCommand;
  type Context<'a> = ();
  type Outcome = ();

  fn handle(&mut self, command: TablesCommand, _context: ()) {
    match command {
      TablesCommand::AddTwo => {
        self.facts.push(Fact::Added(1));
        self.facts.push(Fact::Added(2));
      }
      TablesCommand::Refuse => self.facts.push(Fact::Refused),
    }
  }
}

/// What a Flo-shaped writer borrows for one call and does not own.
struct FloContext<'a> {
  names: &'a [&'a str],
}

/// Flo-shaped: the state is the actor; the context is borrowed per call.
struct FloLike {
  revision: u64,
  picked: std::option::Option<std::string::String>,
}

impl hexser::application::Actor for FloLike {
  type Command = usize;
  type Context<'a> = FloContext<'a>;
  type Outcome = std::option::Option<u64>;

  fn handle(&mut self, index: usize, context: FloContext<'_>) -> std::option::Option<u64> {
    let name = context.names.get(index)?;
    self.picked = std::option::Option::Some(std::string::String::from(*name));
    self.revision += 1;
    std::option::Option::Some(self.revision)
  }
}

/// Generic over the door: drives any actor whose context is `()`, through the trait alone.
fn handle_all<A>(actor: &mut A, commands: std::vec::Vec<A::Command>) -> std::vec::Vec<A::Outcome>
where
  A: for<'a> hexser::application::Actor<Context<'a> = ()>,
{
  commands
    .into_iter()
    .map(|command| hexser::application::Actor::handle(actor, command, ()))
    .collect()
}

/// why: R09. One trait must carry each measured writer's own outcome. A trait that fixed the
/// outcome to a `Result` (or to `Hexserror`) would not compile for Tables' `()` or Flo's
/// borrowed context, so these three fixtures are the mutant's falsifier.
#[test]
fn an_actor_of_each_measured_shape_implements_one_trait() {
  let mut deck = DeckLike {
    seq: 0,
    name: std::string::String::new(),
  };
  let outcomes = handle_all(
    &mut deck,
    std::vec![
      DeckChange::Clear,
      DeckChange::Rename(std::string::String::from("Q3")),
      DeckChange::Rename(std::string::String::new()),
    ],
  );
  std::assert_eq!(
    outcomes,
    std::vec![
      std::result::Result::Err(ChangeRefusal::AlreadyClear),
      std::result::Result::Ok(DeckEvent {
        seq: 1,
        change: DeckChange::Rename(std::string::String::from("Q3")),
      }),
      std::result::Result::Err(ChangeRefusal::EmptyName),
    ]
  );

  let mut tables = TablesLike {
    facts: std::vec::Vec::new(),
  };
  let units = handle_all(
    &mut tables,
    std::vec![TablesCommand::AddTwo, TablesCommand::Refuse],
  );
  std::assert_eq!(units.len(), 2);
  std::assert_eq!(
    tables.facts,
    std::vec![Fact::Added(1), Fact::Added(2), Fact::Refused]
  );

  let names = ["a", "b"];
  let mut flo = FloLike {
    revision: 0,
    picked: std::option::Option::None,
  };
  let first = hexser::application::Actor::handle(&mut flo, 1, FloContext { names: &names });
  let missing = hexser::application::Actor::handle(&mut flo, 9, FloContext { names: &names });
  std::assert_eq!(first, std::option::Option::Some(1));
  std::assert_eq!(missing, std::option::Option::None);
  std::assert_eq!(flo.picked.as_deref(), std::option::Option::Some("b"));
}

/// The node that stands for a non-generic actor in the architecture graph.
#[derive(hexser::HexActor)]
struct DeckActorNode;

/// Never registered: the must-fail arm's subject.
#[allow(dead_code)]
struct NeverRegistered;

/// why: R11. An actor's node must be Application-layer with the existing `DirectiveHandler`
/// role (no new Role variant), found by name. The must-fail arm names a type no derive
/// registered, so a graph that answered every lookup would turn this red.
#[test]
fn a_hex_actor_node_is_application_layer_directive_handler_by_name() {
  let graph = hexser::registry::ComponentRegistry::build_graph();
  let id = hexser::templates::dependency_id::<DeckActorNode>();
  let node = graph
    .get_node(&id)
    .expect("the HexActor node is in the graph");
  std::assert_eq!(node.layer(), hexser::graph::Layer::Application);
  std::assert_eq!(node.role(), hexser::graph::Role::DirectiveHandler);
  std::assert!(node.type_name().ends_with("DeckActorNode"));

  let absent = hexser::graph::NodeId::from_type_name(std::any::type_name::<NeverRegistered>());
  std::assert!(graph.get_node(&absent).is_none());
}
