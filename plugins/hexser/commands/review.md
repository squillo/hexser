---
description: Review Rust code against hexser best practices
argument-hint: "[path (optional)]"
allowed-tools: Read, Grep, Glob, Bash(cargo *)
---

# /hexser:review — read-only hexser best-practices review

Review hexser (0.5.x) Rust code for correctness and adherence to the framework's
grain. This command is **READ-ONLY**: you inspect, cite, and recommend — you do
**not** edit, write, or fix any file. Suggest fixes in prose only.

## Scope

- If `$ARGUMENTS` is given, review that path (a file or a directory — recurse into
  `.rs` files under a directory).
- If `$ARGUMENTS` is empty, review the changed / related hexser files. Prefer the
  working set: run `cargo metadata` sparingly if needed, and use Grep/Glob to find
  Rust files that mention hexser (e.g. `use hexser`, `HexEntity`, `HexDomain`,
  `HexPort`, `HexAdapter`, `Repository`, `Hexserror`). Do not run git.

Use only Read, Grep, Glob, and `cargo` commands. Read a file before citing it.

## Ground truth

The rules below are the fact-checked hexser 0.5 API. Trust the skill's reference
docs over memory (and over the stale workspace-root README). Cite the doc that
backs each finding:

- `skills/hexser/references/derives.md` — derives, `#[hex(role = "…")]`, `id`-field rule.
- `skills/hexser/references/api.md` — trait signatures, `Repository`/`QueryRepository`, features.
- `skills/hexser/references/patterns.md` — Domain → Port → Adapter layering, full example.
- `skills/hexser/references/errors.md` — `Hexserror`, boxing, constructors, rich-error builders.
- `skills/hexser/references/graph-and-ai.md` — `HexGraph`, `to_json`/export, `AIContext`/`AgentPack`.
- `skills/hexser/references/migration-0.4-to-0.5.md` — 0.4 → 0.5 breaking changes.

## Checklist (check each; cite `file:line` + the rule + the backing doc)

1. **`HexEntity` types have an `id` field.** Any type deriving `HexEntity` (or
   `HexDomain`, which also targets entities) MUST have a struct field literally named
   `id`; the macro takes `type Id` from it. No `id` field (or a non-struct) is a
   **compile error** in 0.5 — there is no silent `Id = String` default. A hand-written
   `impl hexser::HexEntity for T { type Id = …; }` is the escape hatch when the
   identity differs. Rule: derives.md, patterns.md.

2. **`Repository<T>` is used for writes, `QueryRepository<T>` for reads.**
   `Repository<T>` is save-only: `fn save(&mut self, entity: T) -> HexResult<()>`.
   Reads/counts/deletes live on `QueryRepository<T>` (associated `Filter` + `SortKey`;
   `find_one`, `find`, `exists`, `count`, `delete_where`). Flag reads attempted through
   `Repository` alone, and repository adapters that implement one but need both. Rule:
   api.md, patterns.md.

3. **`delete_where` is overridden wherever deletion is used.** Its 0.5 default returns
   `Err(port::NOT_IMPLEMENTED = E_HEX_103)`, not `Ok(0)`. Flag any `QueryRepository`
   impl whose callers delete but which does not override `delete_where` — the delete
   silently returns an error. Rule: api.md, patterns.md.

4. **`Hexserror` is constructed via constructor methods or properly boxed.** In 0.5
   every variant payload is `Box<…>`. Prefer the constructors that box for you —
   `Hexserror::domain(code, msg)`, `::port(code, msg)`, `::adapter(code, msg)`,
   `::validation(msg)`, `::validation_field(msg, field)`, `::not_found(resource, id)`,
   `::conflict(msg)`. Direct variant construction MUST wrap the layer error in a box,
   e.g. `Hexserror::Adapter(std::boxed::Box::new(err))`. Flag direct construction that
   passes an unboxed payload. (Matching `Hexserror::Adapter(inner)` binds the `Box` and
   needs no change.) Rule: errors.md.

5. **Errors carry `with_next_step` / `with_suggestion`.** Rich guidance is hexser's
   whole point. `.with_next_step(…)` / `.with_next_steps(…)` and `.with_suggestion(…)` /
   `.with_suggestions(…)` apply to all six variants; `.with_source(err)` chains a cause
   on Domain/Port/Adapter. Flag error values returned to callers with no next-step or
   suggestion as an improvement. Rule: errors.md.

6. **Correct derives and `#[hex(role)]`.** Check registration derives match the layer:
   `HexDomain` (Domain/Entity), `HexPort` (Port; default `Role::Repository`, override
   `#[hex(role = "InputPort")]` etc.), `HexAdapter` (Adapter; default `Role::Adapter`,
   override `#[hex(role = "Mapper")]`, and it also emits `impl Adapter for T {}`),
   `HexDirective`/`HexQuery` (Application). `HexRepository` is a pure marker (emits no
   code) — pair it with `HexPort`. Derives target structs/enums only (unions error).
   Flag a `#[hex(role = "…")]` whose value is not one of the 17 `Role` variants, or a
   role that contradicts the derive's layer. Rule: derives.md.

7. **Layering is respected (domain does not depend on adapters/infra).** Domain modules
   must not import or reference adapter/infrastructure types; dependencies point inward
   (Adapter → Port → Domain). Flag `use` statements or types in a domain module that
   reach into adapter/infra code. Rule: patterns.md.

8. **Prelude is used instead of deep paths.** Prefer `use hexser::prelude::*;`. Deep
   paths are only justified for items NOT in the prelude (e.g.
   `hexser::ports::repository::QueryRepository` for disambiguation, or
   `hexser::showcase::visualizable::Visualizable`, which is not re-exported). Flag deep
   paths for items the prelude already exports as a readability improvement. Rule: api.md.

9. **`to_json` call sites handle `HexResult`.** In 0.5 `AIContext::to_json`,
   `AgentPack::to_json`, and `HexGraph::to_json` (feature `visualization`) return
   `HexResult<String>`, not `Result<String, String>` or a bare `String`. Flag call
   sites that ignore the `Result` or assume a plain `String`. Rule: graph-and-ai.md,
   migration-0.4-to-0.5.md.

## Optionally build/lint

You MAY run, from the crate root, read-only checks to catch compile-level issues:

```bash
cargo check
cargo clippy
```

Use their output to confirm findings (e.g. a missing `id` field surfaces as a
compile error). Do not add features or modify the build. Never edit code to make
these pass — this command only reports.

## Report format

Present findings grouped by severity, most severe first:

- **High** — will not compile or is a correctness bug (missing `id` field on a
  `HexEntity`; read via `Repository` that lacks `QueryRepository`; unboxed direct
  `Hexserror` variant; `delete_where` relied on but not overridden; ignored
  `to_json` `HexResult`; domain depending on adapter/infra).
- **Medium** — likely wrong at runtime or a mismatched derive/role.
- **Low** — style / best-practice (missing `with_next_step`/`with_suggestion`,
  deep paths where the prelude suffices).

For each finding give:

```
[Severity] file:line — <what> — rule: <one-line rule> (see <reference doc>)
Suggested fix: <concise prose; do NOT apply it>
```

End with a short summary (counts per severity) and, if you ran them, the
`cargo check` / `cargo clippy` result. Do not edit any file.
