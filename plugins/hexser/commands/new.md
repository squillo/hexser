---
description: Scaffold a hexser project or a Domain/Port/Adapter slice
argument-hint: "[entity-or-slice name]"
allowed-tools: Read, Write, Edit, Bash(cargo *), Glob, Grep
---

# /hexser:new — scaffold a hexser slice

You are scaffolding a hexser 0.5 Domain → Port → Adapter slice for **`$ARGUMENTS`**.

Read `skills/hexser/references/patterns.md` first and follow it **exactly** — it is
the fact-checked, compiling canonical build. Trust it (and the skill's other
`references/*.md`) over memory and over the stale workspace-root README. Every hexser
API name, trait signature, and feature you emit must match those references; if unsure,
omit rather than guess.

## 0. Resolve what to scaffold

- If `$ARGUMENTS` is empty: don't guess blindly. Inspect the repo (`Glob` for
  `**/Cargo.toml`, `src/**`, existing `domain/`/`ports/`/`adapters/` modules) and, if the
  intent is still unclear, **ask the user** what entity or slice to scaffold before writing
  anything.
- Otherwise treat `$ARGUMENTS` as the entity/slice name. Derive:
  - **Type name** in PascalCase (e.g. `product` → `Product`).
  - **Module/file names** in snake_case (e.g. `Product` → `product`).
  - Repository port `<Type>Repository`, in-memory adapter `InMemory<Type>Repository`,
    filter enum `<Type>Filter`, sort-key enum `<Type>SortKey`.

## 1. Ensure the hexser dependency

`Read` the nearest `Cargo.toml`. Confirm the `[dependencies]` table contains exactly:

```toml
hexser = { version = "0.6", features = ["macros"] }
```

- If `hexser` is **missing**, add that line (`Edit`). The `macros` feature is default-on
  but list it explicitly. Add `visualization` and/or `ai` to the features array only if the
  user also wants graph JSON export or AI context.
- If `hexser` is present at an **older** version (0.4.x), do not silently bump it — tell the
  user and suggest `/hexser:migrate` for the 0.4 → 0.5 breaking changes.
- If it already matches, leave it untouched.

## 2. Create or extend the module layout

Use a `domain/` → `ports/` → `adapters/` layout under `src/` (create the directories/`mod`
declarations if absent, extend them if present — never clobber existing code). Wire each new
module into its parent `mod.rs`/`lib.rs`/`main.rs` with `pub mod …;`.

```
src/
  domain/      <type>.rs        entity
  ports/       <type>_repository.rs   custom Repository port
  adapters/    in_memory_<type>_repository.rs   in-memory adapter
```

### 2a. Entity (`domain/<type>.rs`)

A struct with a **mandatory field literally named `id`** (the `HexEntity` derive takes
`type Id` from it — no `id` field is a compile error in 0.5), deriving `HexDomain` +
`HexEntity`:

```rust
use hexser::prelude::*;

#[derive(HexDomain, HexEntity, Clone, Debug)]
pub struct Product {
    pub id: String,
    pub name: String,
}
```

### 2b. Custom Repository port (`ports/<type>_repository.rs`)

A trait that extends the base `Repository<T>` (which is **save-only**:
`fn save(&mut self, T) -> HexResult<()>`), adding any domain-specific reads:

```rust
use hexser::prelude::*;
use crate::domain::product::Product;

pub trait ProductRepository: Repository<Product> {
    fn find_by_name(&self, name: &str) -> HexResult<Option<Product>>;
}
```

### 2c. In-memory adapter (`adapters/in_memory_<type>_repository.rs`)

Derive `HexAdapter` (which also emits `impl Adapter`). Implement **all three**:
`Repository<T>` (save), `QueryRepository<T>`, and the custom port trait. `QueryRepository`
carries the reads/counts/deletes via associated `Filter` + `SortKey`. Reach it with the
qualified path `hexser::ports::repository::QueryRepository` (not in the prelude), and
**override `delete_where`** — its 0.5 default returns `Err(E_HEX_103 / port::NOT_IMPLEMENTED)`,
not `Ok(0)`. `exists` and `count` have working defaults; keep them unless you need to
override. `FindOptions`, `Sort`, and `Direction` come from the prelude.

```rust
use hexser::prelude::*;
use crate::domain::product::Product;
use crate::ports::product_repository::ProductRepository;

#[derive(Clone)]
pub enum ProductFilter {
    All,
    ById(String),
    ByName(String),
}

#[derive(Clone, Copy)]
pub enum ProductSortKey {
    Id,
    Name,
}

#[derive(HexAdapter, Default)]
pub struct InMemoryProductRepository {
    products: Vec<Product>,
}

impl Repository<Product> for InMemoryProductRepository {
    fn save(&mut self, entity: Product) -> HexResult<()> {
        if let Some(existing) = self.products.iter_mut().find(|p| p.id == entity.id) {
            *existing = entity;
        } else {
            self.products.push(entity);
        }
        Ok(())
    }
}

impl hexser::ports::repository::QueryRepository<Product> for InMemoryProductRepository {
    type Filter = ProductFilter;
    type SortKey = ProductSortKey;

    fn find_one(&self, filter: &ProductFilter) -> HexResult<Option<Product>> {
        Ok(match filter {
            ProductFilter::All => self.products.first().cloned(),
            ProductFilter::ById(id) => self.products.iter().find(|p| &p.id == id).cloned(),
            ProductFilter::ByName(n) => self.products.iter().find(|p| &p.name == n).cloned(),
        })
    }

    fn find(&self, filter: &ProductFilter, _options: FindOptions<ProductSortKey>) -> HexResult<Vec<Product>> {
        Ok(match filter {
            ProductFilter::All => self.products.clone(),
            ProductFilter::ById(id) => self.products.iter().filter(|p| &p.id == id).cloned().collect(),
            ProductFilter::ByName(n) => self.products.iter().filter(|p| &p.name == n).cloned().collect(),
        })
    }

    fn delete_where(&mut self, filter: &ProductFilter) -> HexResult<u64> {
        let before = self.products.len();
        match filter {
            ProductFilter::All => self.products.clear(),
            ProductFilter::ById(id) => self.products.retain(|p| &p.id != id),
            ProductFilter::ByName(n) => self.products.retain(|p| &p.name != n),
        }
        Ok((before - self.products.len()) as u64)
    }
}

impl ProductRepository for InMemoryProductRepository {
    fn find_by_name(&self, name: &str) -> HexResult<Option<Product>> {
        Ok(self.products.iter().find(|p| p.name == name).cloned())
    }
}
```

Replace `Product`/`product` throughout with the resolved names from step 0. Prefer
`use hexser::prelude::*;`; use qualified paths only for items not in the prelude (e.g.
`hexser::ports::repository::QueryRepository`). Match `patterns.md` exactly for anything not
shown here.

## 3. Verify and hand off

- Run `cargo check` and fix any errors. If an entity fails to compile, check the `id`-field
  rule first (it is the most common cause).
- Report what you created/edited (files, the new dependency line if added).
- Suggest **`/hexser:graph`** so the user can see the architecture the code just described
  (`HexGraph::current().pretty_print()`), and mention `/hexser:add-usecase` to add a CQRS
  Directive/Query on top of this slice.
