---
description: Add a CQRS Directive+handler or a Query+handler
argument-hint: "[UseCaseName] [directive|query]"
allowed-tools: Read, Write, Edit, Bash(cargo *), Grep, Glob
---

Add a hexser (0.5) CQRS use case to this crate.

- **`$1`** = the use-case name (e.g. `CreateOrder`, `ListOrdersByCustomer`).
- **`$2`** = `directive` (a command / write side) or `query` (a read side).
  **If `$2` is omitted, default to `directive`.** If `$1` is missing, ask the user
  for a name before doing anything else.

Ground the code in the fact-checked references — read them first and match their
API exactly; do not invent methods, types, or features:
`skills/hexser/references/patterns.md` and `skills/hexser/references/derives.md`.

## Steps

1. **Read the references above.** Then locate where application/use-case code lives:
   use Glob/Grep to find existing `Directive`, `DirectiveHandler`, or `QueryHandler`
   code (e.g. `Grep` for `DirectiveHandler` / `QueryHandler` / `HexDirective`), and
   read the domain entity and port this use case will touch so names line up. Confirm
   `Cargo.toml` has `hexser = { version = "0.6", features = ["macros"] }`.
2. **Read the target file before writing to it.** Add the code in the matching module
   (create a small `application` module if none exists) and export it.
3. Build the pieces for the chosen kind (below).
4. **Run `cargo check`** and fix any errors before reporting back.

Import the prelude in the file: `use hexser::prelude::*;`.

## Directive (command / write side) — `$2 == directive`

`#[derive(HexDirective)]` registers the struct in the graph (Layer::Application,
Role::Directive) **and** derives `fn validate(&self) -> HexResult<()> { Ok(()) }`.
The handler is hand-written.

```rust
use hexser::prelude::*;

// 1. The command: the data needed to perform the write.
#[derive(HexDirective)]
struct CreateOrderDirective {
    customer_id: String,
    total_cents: u64,
}

// 2. The handler holds the port(s) it needs. `handle` takes `&self`, and
//    `Repository::save` takes `&mut self`, so wrap the save-side repo in interior
//    mutability (RefCell single-threaded; std::sync::Mutex if shared across threads).
struct CreateOrderHandler<R> {
    orders: std::cell::RefCell<R>,
}

impl<R> DirectiveHandler<CreateOrderDirective> for CreateOrderHandler<R>
where
    R: Repository<Order>,
{
    fn handle(&self, directive: CreateOrderDirective) -> HexResult<()> {
        // Validate inputs first, then return rich errors on failure.
        directive.validate()?; // derived no-op today; call it to honor the contract
        if directive.customer_id.is_empty() {
            return Err(
                Hexserror::validation_field("customer_id is required", "customer_id")
                    .with_next_step("Pass a non-empty customer id"),
            );
        }
        let order = Order::new(directive.customer_id, directive.total_cents);
        self.orders.borrow_mut().save(order)?;
        Ok(())
    }
}
```

**Where to validate.** The derived `Directive::validate` is a no-op, and you cannot
add a second `impl Directive` for the same struct (conflicting impls). Choose one:

- **Keep the derive** and put real checks at the top of the handler's `handle`
  (recommended — keeps the struct in the architecture graph).
- **Drop `#[derive(HexDirective)]`** and hand-write `impl Directive for CreateOrderDirective
  { fn validate(&self) -> HexResult<()> { … } }` with the real checks. Trade-off: the
  struct is no longer auto-registered in the graph.

## Query (read side) — `$2 == query`

`#[derive(HexQuery)]` registers the struct (Layer::Application, Role::Query) and emits
no trait impl — you write the handler. Read repositories implement `QueryRepository`,
so query through `find` / `find_one` with a `FindOptions`.

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

Return `HexResult` everywhere and attach guidance so failures explain themselves:

- `Hexserror::validation_field(message, field)` (E_HEX_300) — a bad/missing field.
- `Hexserror::validation(message)` (E_HEX_301) — a general invalid input.
- `Hexserror::conflict(message)` (E_HEX_402) — e.g. the entity already exists.
- `Hexserror::not_found(resource, id)` (E_HEX_400) — the target does not exist.
- Chain `.with_next_step("…")` and/or `.with_suggestion("…")` on any of them.

After writing, run `cargo check`. If registration-derived types don't appear in the
graph, remember `HexGraph`/`inventory` is fixed at link time — a rebuild is required.
