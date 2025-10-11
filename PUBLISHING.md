# Publishing Guide for the hex workspace

This repository is a Cargo workspace containing three publishable crates:

1. hex/hexser_macros (proc-macro crate)
2. hex (crate name: hexser)
3. hex/hexser_potions (examples/patterns crate)

Publishing must occur in this exact order because `hexser` depends on `hexser_macros`, and `hexser_potions` depends on `hexser`.

---

## 1) Prerequisites

- A crates.io account with 2FA enabled (recommended).
- A crates.io API token stored locally (one-time):
  - Run: `cargo login <YOUR_CRATES_IO_TOKEN>`
- Clean working tree: no uncommitted changes and CI is green.
- Confirm Rust builds locally:
  - `cargo test --workspace --all-features`
  - `cargo check --workspace` (optional)
- Versions are correct and consistent across crates (see section 2).

Notes
- Publishing is done from the repository root using the `-p <package>` flag.
- Use `--dry-run` first to validate the package before publishing.

---

## 2) Versioning and consistency

Current versions (as committed):
- hexser_macros: 0.4.0
- hexser: 0.4.0 (depends on hexser_macros = "0.4.0")
- hexser_potions: 0.4.0 (depends on hexser = "0.4.0")

When bumping versions in the future:
- Update each crate's `Cargo.toml` `[package] version`.
- Update dependent crates to reference the new version (keep the `path` dependency for local dev, plus the semver version for publishing).
- Commit all changes together and tag the release (e.g., `v0.3.1`).

---

## 3) Publish order and exact commands

Run these commands from the repository root. Always perform a dry run before publishing.

1. Publish macros first
```
# Validate
cargo publish -p hexser_macros --dry-run

# Publish
cargo publish -p hexser_macros
```

2. Wait for indexing
- crates.io indexing can take a few minutes. Verify that the specific version is visible either on crates.io or via:
```
cargo search hexser_macros | grep 0.4.6
```

3. Publish the main crate
```
# Validate
cargo publish -p hexser --dry-run

# Publish
cargo publish -p hexser
```

4. Publish potions
```
# Validate
cargo publish -p hexser_potions --dry-run

# Publish
cargo publish -p hexser_potions
```

---

## 4) Pre-publish checks (recommended)

- Inspect what would be packaged (ignore target/, VCS files, etc.):
```
cargo package -p hexser_macros --list
cargo package -p hexser --list
cargo package -p hexser_potions --list
```
- Verify documentation build features:
  - `hexser` sets `package.metadata.docs.rs.all-features = true`.
- Optional environment for faster index:
```
export CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
```

---

## 5) Post-publish: verification and tagging

- After each publish, confirm the crate page on crates.io and docs.rs build status.
- Create and push a signed tag (example):
```
git tag -s v0.4.5 -m "hexser 0.4.6 release"
git push origin v0.4.5
```

---

## 6) Troubleshooting

- 403 or permission errors: ensure you are an owner/maintainer on crates.io.
- `dependency not found` when publishing `hexser`: wait for `hexser_macros` to be indexed.
- `yanked` or accidental publish:
  - You can yank a version on crates.io to prevent new downloads.
  - Publish a patched version (e.g., bump to 0.3.1) if contents were wrong.
- 2FA code prompts: you'll be asked for a TOTP during `cargo publish` if 2FA is enforced.

---

## 7) Optional automation

If desired later, introduce `cargo-release` to automate version bumps, tagging, and sequential publishing across the workspace. For now, the manual flow above is intentionally simple and reliable.

---

## Quick reference

```
# One-time
cargo login <TOKEN>

# Every release (from repo root)
cargo test --workspace --all-features

cargo publish -p hexser_macros --dry-run
cargo publish -p hexser_macros

# wait for indexing

cargo publish -p hexser --dry-run
cargo publish -p hexser

cargo publish -p hexser_potions --dry-run
cargo publish -p hexser_potions
```
