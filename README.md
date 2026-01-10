# PyreFrame

PyreFrame is a learning-focused Rust game engine project designed to explore game engine architecture while emphasizing explicit data flow, clear ownership, simple ECS design, and strong testability. The goal is to understand how game engines work in Rust by building minimal, idiomatic solutions that evolve only when real needs arise, avoiding over-engineering early on.

Status: early / experimental — APIs will change.

---

**What's here**

- A lightweight engine crate under `engine/` (core systems, ECS, renderer, input, math, time, ...).
- A game crate under `game/` which shows how a game ties into the engine.
- Tests and generated docs under `tests/` and `doc/`.

This repo is meant for learning and experimentation rather than production use.

**Goals**

- Understand game engine architecture in Rust through hands-on implementation.
- Emphasize explicit data flow, clear ownership, and deterministic execution.
- Implement a simple ECS design with strong testability.
- Favor minimal, idiomatic Rust solutions that prioritize clarity and correctness over performance.
- Evolve the design incrementally based on real needs, avoiding premature optimization or complexity.

**Design Principles**

- **Explicit Data Flow**: All state changes are explicit; no hidden globals or implicit behavior.
- **Clear Ownership**: Leverage Rust's ownership system for safe, predictable resource management.
- **Simple ECS**: Centralize ECS under `engine/src/ecs`; systems operate via `engine::World`.
- **Strong Testability**: Write tests for new functionality; ensure `cargo test --workspace` passes after changes.
- **Deterministic Execution**: Execution order is predictable; no parallelism or async code.
- **Minimal and Idiomatic**: Use stable Rust features; avoid macros, heavy generic abstractions, or premature optimizations.
- **Evolutionary Design**: Only add complexity when real needs arise; start simple and expand based on usage.

---

**Quick start**

Prerequisites: Rust toolchain (rustup + stable toolchain).

---

**Project layout**

- `engine/` — the engine crate (src/lib.rs), a library crate called `pyreframe_engine`.
- `game/` — example game crate (src/main.rs), a binary crate called `pyreframe_game` that demonstrates engine usage.
- `tests/` — integration/unit tests.

Entry points:

- Game binary entry: `game/src/main.rs`.
- Engine is a library crate under `engine/`.

---

**Development notes**

- Keep engine APIs small and explicit — prefer clearly named systems and modules.
- When adding runtime code, put modules in `engine/src/` and re-export as needed from `lib.rs`.
- Add dependencies in `Cargo.toml` at the crate that needs them.
- This is a learning project: welcome architectural feedback, design critique, and evolutionary suggestions that respect the constraints (stable Rust, no macros/generics/parallelism/async) and focus on why certain approaches are good or bad for learning game engine architecture.

---

**License**

See the `LICENSE` file in the repo.
