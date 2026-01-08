<!-- Auto-generated guidance for AI coding agents working on PyreFrame -->
# PyreFrame — Copilot Instructions

This project is a small Rust binary crate (see `Cargo.toml`) with its entrypoint at `src/main.rs`.
The repo is an experimental game engine framework; the `readme.md` documents high-level goals: modular engine architecture, explicit systems, and decoupling engine code from game logic.

Quick facts
- **Entrypoint:** `src/main.rs` (currently prints a placeholder `Hello, world!`).
- **Manifest:** `Cargo.toml` (edition = 2024, no dependencies listed yet).
- **Status:** early / experimental; APIs and layout may change frequently (see `readme.md`).

What to do when editing
- If adding runtime code, place modules under `src/` and import them from `src/main.rs` (this repository currently uses a single binary crate layout).
- Add dependencies in `Cargo.toml` and run `cargo build` to fetch and compile them.

Build / run / debug commands
- Build: `cargo build`
- Run (dev): `cargo run`
- Run (release): `cargo run --release` or `cargo build --release` then run the binary in `target/release/`.
- Formatting: `cargo fmt`
- Lint: `cargo clippy` (if the toolchain is installed)

Project-specific patterns to follow
- The README emphasizes explicit systems and clear separation between engine and game logic. Prefer small, well-named modules and explicit wiring in `main.rs` rather than large, implicit constructors.
- Keep public API boundaries small — engine code should be decoupled from game logic per the project's stated goals.

Files to inspect for context
- `readme.md` — high-level goals and intent for architecture
- `Cargo.toml` — crate metadata and dependencies
- `src/main.rs` — current program entrypoint and example wiring

# PyreFrame — AI coding agent instructions

This repository is a small, experimental Rust workspace with two members: the engine library (`engine/`) and an example game binary (`game/`). The project favors explicit, modular systems and clear separation between engine and game logic.

Quick facts
- Workspace members: `engine`, `game` (see [Cargo.toml](../Cargo.toml)).
- Engine crate entry: [engine/src/lib.rs](../engine/src/lib.rs) — re-exports stable engine API.
- Game binary entry: [game/src/main.rs](../game/src/main.rs).

Essential architecture notes
- The `engine/` crate is a library exposing core subsystems: `ecs`, `render`, `input`, `math`, `time` (see [engine/src/lib.rs](../engine/src/lib.rs) and `engine/src/`).
- The engine intentionally re-exports a narrow public API (e.g., `World`) from `lib.rs`; keep runtime internals private and add runtime modules under `engine/src/`.
- The `game/` crate demonstrates how to wire systems and assets on top of the engine; treat it as a usage example rather than production code.

Developer workflows (what actually works here)
- Build entire workspace: `cargo build` (or `cargo build --workspace`).
- Run the example game: `cargo run -p game`.
- Run docs: `cargo doc --workspace --no-deps` (docs are generated under `target/doc` and partial HTML is in `doc/`).
- Run tests: `cargo test --workspace` (there are integration tests under `tests/`).
- Format: `cargo fmt --all`; Lint: `cargo clippy --all-targets --all-features`.

Project-specific conventions
- Engine-first design: add new runtime modules in `engine/src/` and re-export stable types from `engine/src/lib.rs`.
- Small, explicit APIs: prefer stable, well-named functions/systems over broad convenience helpers.
- Modules map to subsystems: e.g., `engine/src/ecs`, `engine/src/render`, `engine/src/input`. When integrating, call into these modules rather than reaching into private internals.

Integration points & patterns
- ECS is central — systems and world state live under `engine/src/ecs`. Games should operate via `engine::World` and registered systems.
- Renderer and input are engine subsystems; the example game shows registration patterns in `game/src/main.rs` (simple wiring today).
- Cross-crate communication: `game` depends on `engine` via workspace member; add dependencies in the corresponding `Cargo.toml`.

Editing guidance for AI agents
- Preserve public API shape: when changing `engine/src/lib.rs`, consider compatibility with `game/` and tests.
- Be explicit in added APIs — add small modules and re-export only the intended surface.
- Update `README.md` or add a short changelog note when altering API contracts.

Where to look first
- [README.md](../README.md) — project goals and layout.
- [engine/src/lib.rs](../engine/src/lib.rs) and directory `engine/src/` — core engine code.
- [game/src/main.rs](../game/src/main.rs) — example wiring.
- [tests/](../engine/tests/) — integration tests that show expected behaviors.

If you change runtime code, run `cargo build` and `cargo test --workspace` before submitting changes. Ask for clarification if an API change affects `game/` or tests.

If anything above is unclear or you want the file expanded with more examples, tell me which area to deepen (module layout examples, test strategy, or CI suggestions).
