---
description: Add a write-side Actor or a read-side Query+handler
argument-hint: "[UseCaseName] [actor|query]"
allowed-tools: Read, Write, Edit, Bash(cargo *), Grep, Glob
---

Add a hexser (0.7) use case to this crate.

- **`$1`** = the use-case name (e.g. `CreateOrder`, `ListOrdersByCustomer`).
- **`$2`** = `actor` (the write side) or `query` (the read side). `directive` is accepted
  and means `actor`. **If `$2` is omitted, default to `actor`.** If `$1` is missing, ask the
  user for a name before doing anything else.

Ground the code in the fact-checked references. Read them first and match their API
exactly. Do not invent methods, types, or features:
`skills/hexser/references/api.md`, `skills/hexser/references/derives.md`,
`skills/hexser/references/errors.md` and `skills/hexser/references/patterns.md`.
`patterns.md` still shows the 0.6 write side through `DirectiveHandler`. For the write side,
follow this command instead.

## Steps

1. **Read the references above.** Then locate where application/use-case code lives.
   Grep for `Actor`, `HexActor`, `QueryHandler`, and the 0.6 `DirectiveHandler`. Read the
   domain entity and port this use case will touch so names line up. Confirm `Cargo.toml`
   has `hexser = { version = "0.7", features = ["macros"] }`. `Actor` does not exist before
   0.7. If the crate pins `0.6`, suggest `/hexser:migrate` first.
2. **Read the target file before writing to it.** Add the code in the matching module
   (create a small `application` module if none exists) and export it.
3. Build the pieces for the chosen kind (below).
4. **Run `cargo check`** and fix any errors before reporting back.

Import the prelude in the file: `use hexser::prelude::*;`. It brings the derives `HexActor`,
`HexRefusal` and `HexVocabulary`. The traits `Actor`, `Vocabulary` and `Refusal` are not in
the prelude. Name them by path, e.g. `hexser::application::Actor`.

## Actor (the write side) — `$2 == actor`

An actor owns its state and is the one place that state changes. `handle` takes
`&mut self`, so there is one writer and one command at a time, and no lock. The command comes
in by value. The context and the outcome are associated types. The trait imposes no `Result`,
no `Hexserror`, and no `Send` or `Sync` bound.

`DirectiveHandler` is deprecated in 0.7, routed to `Actor`. Do not write new code against it.

```rust
// 1. The command: the data the write needs. It is consumed by value.
pub struct CreateOrder {
    pub customer_id: String,
    pub total_cents: u64,
}

// 2. The event the actor decides on success.
#[derive(Debug, PartialEq)]
pub enum OrderEvent {
    Created { customer_id: String, total_cents: u64 },
}

// 3. The refusal: a closed enum. Each variant may carry static guidance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, hexser::HexRefusal)]
pub enum CreateOrderRefusal {
    #[hex(next_step = "Pass a non-empty customer id")]
    MissingCustomer,
    #[hex(next_step = "Pass a total above zero", suggestion = "Totals are in cents")]
    ZeroTotal,
}

// 4. The actor owns the state it changes.
pub struct OrderBook {
    orders: Vec<Order>,
}

impl hexser::application::Actor for OrderBook {
    type Command = CreateOrder;
    type Context<'a> = ();
    type Outcome = Result<OrderEvent, CreateOrderRefusal>;

    fn handle(&mut self, command: CreateOrder, _context: ()) -> Self::Outcome {
        // Check first. Change state only after every check passes.
        if command.customer_id.is_empty() {
            return Err(CreateOrderRefusal::MissingCustomer);
        }
        if command.total_cents == 0 {
            return Err(CreateOrderRefusal::ZeroTotal);
        }
        self.orders.push(Order::new(command.customer_id.clone(), command.total_cents));
        Ok(OrderEvent::Created {
            customer_id: command.customer_id,
            total_cents: command.total_cents,
        })
    }
}

// 5. The actor's node in the architecture graph: Application layer, role `DirectiveHandler`.
#[derive(hexser::HexActor)]
struct OrderBookActorNode;

// 6. At the edge, the refusal crosses into `Hexserror` with its code and guidance.
pub fn create_order(book: &mut OrderBook, command: CreateOrder) -> hexser::HexResult<OrderEvent> {
    hexser::application::Actor::handle(book, command, ())
        .map_err(|refusal| hexser::Hexserror::refused(&refusal, "The order was not created"))
}
```

- `#[derive(HexActor)]` emits no `Actor` impl. It only registers the node. It reads `role`
  and `depends_on(..)`. It refuses a generic type, so a generic actor registers a non-generic
  marker struct like `OrderBookActorNode`. A non-generic actor may derive it directly.
- An actor that holds a port keeps it as a plain field. `&mut self` reaches it, so it needs
  no `RefCell` or lock. Name the port's node on the actor's node with
  `#[hex(depends_on(OrderRepositoryPortNode))]`. That node must itself be registered.
- The outcome is yours: an event, a view, a `Result` of the actor's own refusal, or `()`. If
  a port call can fail, put that failure in the outcome type.
- A context that borrows per call is a generic associated type, for example
  `type Context<'a> = &'a Clock;`.
- A call to `Actor::handle` through a shared reference does not compile. The single writer
  holds `&mut`.
- `Order` is the domain entity. Reuse the existing one.

## Query (read side) — `$2 == query`

`#[derive(HexQuery)]` registers the struct (Layer::Application, Role::Query) and emits
no trait impl — you write the handler. Read repositories implement `QueryRepository`,
so query through `find` / `find_one` with a `FindOptions`. Implement
`hexser::application::QueryHandler`. `hexser::ports::Query` is deprecated in 0.7, routed to
`QueryHandler`.

```rust
use hexser::prelude::*;

// 1. The query: the parameters that select the data.
#[derive(HexQuery)]
struct ListOrdersByCustomerQuery {
    customer_id: String,
}

// 2. QueryHandler<Q, R>::handle(&self, Q) -> HexResult<R> returns the result.
struct ListOrdersByCustomerHandler<Repo> {
    orders: Repo,
}

impl<Repo> QueryHandler<ListOrdersByCustomerQuery, Vec<Order>>
    for ListOrdersByCustomerHandler<Repo>
where
    Repo: QueryRepository<Order, Filter = OrderFilter>,
{
    fn handle(&self, query: ListOrdersByCustomerQuery) -> HexResult<Vec<Order>> {
        if query.customer_id.is_empty() {
            return Err(
                Hexserror::validation_field("customer_id is required", "customer_id")
                    .with_suggestion("Provide the customer id to filter by"),
            );
        }
        self.orders.find(
            &OrderFilter::ByCustomer(query.customer_id),
            FindOptions::default(),
        )
    }
}
```

`Order`, `OrderFilter`, and the repository are the domain entity, the query
repository's `Filter` type, and its adapter — reuse the existing ones (see
`skills/hexser/references/patterns.md`; `/hexser:add-adapter` scaffolds a repo).

## Rich errors

A refusal stays a typed value inside the crate. It crosses into `Hexserror` only at the edge:

- `hexser::Hexserror::refused(&refusal, message)` builds the existing `Domain` variant. It
  copies the refusal's code and its static `next_step` and `suggestion` guidance.
- `err.code()` returns the code for any of the six variants.
  `CreateOrderRefusal::from_code(err.code())` reads the refusal back.

```rust
let err = hexser::Hexserror::refused(&CreateOrderRefusal::MissingCustomer, "The order was not created");
assert!(matches!(err, hexser::Hexserror::Domain(_)));
assert_eq!(err.code(), "MissingCustomer");
assert_eq!(CreateOrderRefusal::from_code(err.code()), Some(CreateOrderRefusal::MissingCustomer));
```

For a failure that is not a refusal, return `HexResult` and attach guidance so it explains
itself:

- `Hexserror::validation_field(message, field)` (E_HEX_300) — a bad/missing field.
- `Hexserror::validation(message)` (E_HEX_301) — a general invalid input.
- `Hexserror::conflict(message)` (E_HEX_402) — e.g. the entity already exists.
- `Hexserror::not_found(resource, id)` (E_HEX_400) — the target does not exist.
- Chain `.with_next_step("…")` and/or `.with_suggestion("…")` on any of them.

After writing, run `cargo check`. If registration-derived types don't appear in the
graph, remember `HexGraph`/`inventory` is fixed at link time — a rebuild is required.
