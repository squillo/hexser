# Tutorial 01: Hello Hex (5 minutes)

## Goal
Create your first hex component and see the magic happen!

## The "Aha Moment"
With just **3 lines of code**, you'll have a working hexagonal architecture component that:
- ✅ Automatically registers itself
- ✅ Appears in the architecture graph
- ✅ Can be visualized instantly

## Prerequisites
- Rust installed (rustup is recommended)
- A new Rust project

## Step 1: Create a New Project
```bash
cargo new hello-hex
cd hello-hex
```


## v0.4 Note: Reading data with QueryRepository

As of v0.4, repositories focus on saving aggregates, while reads and deletes-by-criteria go through the generic QueryRepository trait using domain-owned filters (e.g., enums). Examples are included in:
- hexser/examples/tutorial_03_adapters.rs
- hexser/examples/simple_todo.rs
- hexser_potions/src/crud/mod.rs

These demonstrate find_one, find with options (sort/pagination), exists, count, and delete_where.
