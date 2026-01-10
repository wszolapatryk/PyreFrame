<!-- Auto-generated guidance for AI coding agents working on PyreFrame -->
# PyreFrame — Copilot Instructions

This project is a learning-focused Rust game engine framework built as a Cargo workspace. The goal is to understand game engine architecture in Rust while emphasizing explicit data flow, clear ownership, simple ECS design, and strong testability. The project avoids over-engineering early, favoring minimal, idiomatic Rust solutions that evolve only when real needs appear. Execution is deterministic with no hidden globals or implicit data flow—all state changes are explicit. Uses stable Rust, avoids macros, heavy generic abstractions, parallelism, and async code for now, prioritizing clarity and correctness over performance.

Quick facts
- **Workspace members:** `engine` (library crate: pyreframe_engine), `game` (binary crate: pyreframe_game) (see [Cargo.toml](../Cargo.toml)).
- **Engine crate entry:** [engine/src/lib.rs](../engine/src/lib.rs) — re-exports stable engine API.
- **Game binary entry:** [game/src/main.rs](../game/src/main.rs) — demonstrates engine usage.
- **Status:** early / experimental; APIs and layout may change frequently as design evolves based on real needs.

What to do when editing
- Add runtime modules under `engine/src/` and re-export stable types from `engine/src/lib.rs`.
- Add dependencies in the appropriate `Cargo.toml` (engine or game) and run `cargo build` to fetch and compile them.
- Prefer small, well-named modules and explicit wiring over large, implicit constructors.
- Keep public API boundaries small — engine code should be decoupled from game logic.

Build / run / debug commands
- Build entire workspace: `cargo build` (or `cargo build --workspace`).
- Run the example game: `cargo run -p game`.
- Run docs: `cargo doc --workspace --no-deps`.
- Run tests: `cargo test --workspace`.
- Format: `cargo fmt --all`; Lint: `cargo clippy --all-targets --all-features`.

Project-specific patterns to follow
- **Explicit data flow and ownership:** All state changes are explicit; avoid implicit behavior or hidden globals.
- **Simple ECS design:** Centralize ECS under `engine/src/ecs`; systems operate via `engine::World`.
- **Deterministic execution:** Execution order is predictable; no parallelism or async.
- **Strong testability:** Write tests for new functionality; run `cargo test --workspace` after changes.
- **Minimal and idiomatic:** Use stable Rust features; avoid macros, complex generics, or premature optimizations.
- **Evolutionary design:** Only add complexity when real needs arise; start simple and expand based on usage.

Files to inspect for context
- [README.md](../README.md) — high-level goals and intent for architecture.
- [Cargo.toml](../Cargo.toml) — workspace configuration.
- [engine/src/lib.rs](../engine/src/lib.rs) — engine public API.
- [engine/src/](../engine/src/) — core engine modules (ecs, render, etc.).
- [game/src/main.rs](../game/src/main.rs) — example game wiring.
- [tests/](../engine/tests/) — integration tests.

# PyreFrame — AI coding agent instructions

This repository is a learning-focused Rust workspace for building a game engine. It consists of two crates: the `engine` library exposing core subsystems, and the `game` binary demonstrating usage. The design emphasizes explicit data flow, clear ownership, simple ECS, and testability, evolving minimally as needs appear.

Essential architecture notes
- The `engine/` crate provides core subsystems: `ecs`, `render`, `input`, `math`, `time`, etc. (see [engine/src/lib.rs](../engine/src/lib.rs)).
- Re-export a narrow public API from `lib.rs`; keep internals private.
- The `game/` crate shows how to use the engine; treat it as a usage example, not production code.
- ECS is central: systems and world state in `engine/src/ecs`; games operate via `engine::World` and registered systems.

Developer workflows
- Build: `cargo build --workspace`.
- Run game: `cargo run -p pyreframe_game`.
- Test: `cargo test --workspace`.
- Format: `cargo fmt --all`; Lint: `cargo clippy --all-targets --all-features`.

Project-specific conventions
- Engine-first: Add modules in `engine/src/` and re-export from `lib.rs`.
- Small, explicit APIs: Prefer stable, well-named functions/systems.
- Subsystems: e.g., `engine/src/ecs`, `engine/src/render`. Integrate by calling into modules, not internals.
- Deterministic: No async, parallelism, or non-deterministic behavior.
- Testable: Add tests for new code; ensure `cargo test --workspace` passes.

Integration points & patterns
- ECS central: Use `engine::World` and systems for game logic.
- Renderer/input: Engine subsystems; register in `game/src/main.rs`.
- Cross-crate: `game` depends on `engine`; add deps in respective `Cargo.toml`.

Editing guidance for AI agents
- Preserve API shape: Consider `game/` and tests when changing `lib.rs`.
- Be explicit: Add small modules and re-export only intended surface.
- Update docs: Modify `README.md` or add notes for API changes.
- Constraints: Respect stable Rust, no macros/generics/async/parallelism; prioritize clarity over performance.
- Feedback focus: Provide architectural feedback, design critique, evolutionary suggestions explaining why approaches are good/bad, respecting learning goals and constraints.

Where to look first
- [README.md](../README.md) — project goals.
- [engine/src/lib.rs](../engine/src/lib.rs) — API.
- [engine/src/](../engine/src/) — engine code.
- [game/src/main.rs](../game/src/main.rs) — usage.
- [tests/](../engine/tests/) — tests.

After changes, run `cargo build` and `cargo test --workspace`. If API changes affect `game/` or tests, seek clarification.

For expansions: Ask about module layouts, test strategies, or CI if unclear.
