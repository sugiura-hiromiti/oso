# OSO Architecture & Dependency Guidelines

This document outlines a refactor-oriented, practical guide to keep the project modular and avoid cyclic dependencies while scaling the codebase.

## Summary

- Goal: maintain a clear, acyclic dependency graph between crates and between kernel modules.
- Approach: define explicit layers, document allowed directions, and add simple CI checks.

## Current Crates (observed)

- Runtime: `oso_kernel`, `oso_loader`
- Core (leaf): `oso_error`, `oso_no_std_shared`
- Codegen: `oso_proc_macro` (proc-macro facade), `oso_proc_macro_logic` (implementation)
- Dev/Tools: `xtask`, `oso_dev_util`, `oso_dev_util_helper`

These already form a DAG. The main risk is future coupling across these layers.

## Target Layering Model

```text
Core      :  oso_error, oso_no_std_shared
Codegen   :  oso_proc_macro  ->  oso_proc_macro_logic
Runtime   :  oso_loader, oso_kernel
Dev/Tools :  oso_dev_util_helper, oso_dev_util, xtask
```

Allowed dependency directions (must be acyclic):
- Runtime → Core, Codegen
- Codegen → Dev/Tools (read-only helpers OK)
- Dev/Tools → Core (for types) is OK, but NOT Runtime
- Core → (none)

Disallowed examples:
- Dev/Tools → Runtime (leaks tooling into shipped artifacts)
- Core → Codegen/Runtime/Dev (pollutes foundational crates)
- Codegen → Runtime (macros shouldn’t depend on runtime behavior)

## Cycle Hotspots to Watch

- Proc-macro chain: `dev_util` → `proc_macro` → `proc_macro_logic` → `dev_util_helper`. Keep `dev_util_helper` free of proc-macro or runtime deps to prevent a loop.
- Workspace-introspecting macros: the `#[features]` macro that scans the workspace couples macro expansion to repo layout.
- Shared crate growth: if `oso_no_std_shared` starts importing macros or higher layers, it can create indirect cycles.

## Concrete Refactors

1) Workspace hygiene
- Add all local crates to `[workspace].members` for single lockfile and unified lints:
  - `oso_kernel`, `oso_loader`, `xtask`, `oso_error`, `oso_no_std_shared`, `oso_proc_macro`, `oso_proc_macro_logic`, `oso_dev_util`, `oso_dev_util_helper`
- Optional: move crates under `crates/` and tools under `tools/`, then set `workspace.default-members = ["oso_loader", "oso_kernel"]` to keep `cargo build` focused.

2) Proc-macro boundaries
- Keep `oso_proc_macro` thin; it must not depend on runtime crates.
- Replace the workspace-scanning `#[features]` proc-macro with a `build.rs` code generator that:
  - Reads features from workspace `Cargo.toml`s
  - Emits `OUT_DIR/generated_features.rs`
  - Use `include!(concat!(env!("OUT_DIR"), "/generated_features.rs"));`
- Benefit: removes compile-time coupling between macro expansion and the workspace structure.

3) Kernel layering (intra-crate hygiene)
- Define traits in `base` (e.g., display/framebuffer), implement in `driver`.
- Let `app` depend on `base` interfaces, not on `driver` directly.
- Prefer `pub(crate)` over `pub` inside the kernel to limit accidental cross-module reach.

4) Dev/Tools containment
- Keep `oso_dev_util_helper` lean and dependency-free (no macros, no runtime).
- Ensure `xtask`/`oso_dev_util` don’t leak into runtime crates via feature flags or re-exports.

## Enforcement in CI

Lightweight checks that prevent regressions:

- Allowed edges policy (documented in this file) and a metadata check:
  - Generate graph: `cargo metadata --format-version=1`
  - Parse `packages[].name` and `packages[].dependencies[]` and assert allowed edges.
- Optionally add `cargo-deny` rules to fail on cycles or forbidden edges.

Example check sketch (pseudo-shell):

```sh
# Generate metadata JSON
cargo metadata --format-version=1 > target/metadata.json

# Example: jq to print edges (adjust as needed)
jq -r '.packages[] | .name as $n | .dependencies[].name as $d | "\($n) -> \($d)"' \
  target/metadata.json

# Then validate edges against an allowlist in a small script
```

## Quick Wins

- Add the missing crates to the workspace members to unify lockfiles.
- Move features enumeration from `#[features]` macro to `build.rs` codegen.
- Document the allowed dependency directions and link this file from `README.md`.
- Adopt `pub(crate)` more broadly in `oso_kernel` to keep module boundaries tight.

## Action Checklist

- [ ] Update `[workspace].members` to include all local crates
- [ ] Create a `build.rs`-based features generator and remove workspace-scanning macro
- [ ] Add a simple `cargo metadata`-based CI check for allowed edges
- [ ] Mark internal APIs `pub(crate)` where possible in kernel
- [ ] Keep `oso_dev_util_helper` as a leaf helper with no proc-macro/runtime deps

---

This document is intentionally short and pragmatic; it’s meant to serve as a guardrail for daily development and reviews.

