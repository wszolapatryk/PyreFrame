# PyreFrame

PyreFrame is an experimental game engine framework written in Rust. It's a small, friendly codebase for exploring modular engine design, explicit systems, and clean separation between engine and game logic.

Status: early / experimental — APIs will change.

---

**What's here**

- A lightweight engine crate under `engine/` (core systems, ECS, renderer, input, math, time, ...).
- A game crate under `game/` which shows how a game ties into the engine.
- Tests and generated docs under `tests/` and `doc/`.

This repo is meant for learning and experimentation rather than production use.

**Goals**

- Keep the architecture modular and explicit.
- Separate engine code from game logic.
- Prefer clarity and correctness over clever hacks.

---

**Quick start**

Prerequisites: Rust toolchain (rustup + stable toolchain).

---

**Project layout**

- `engine/` — the engine crate (src/lib.rs).
- `game/` — example game crate (src/main.rs) showing how to use the engine.
- `tests/` — integration/unit tests.

Entry points:

- Game binary entry: `game/src/main.rs`.
- Engine is a library crate under `engine/`.

---

**Development notes**

- Keep engine APIs small and explicit — prefer clearly named systems and modules.
- When adding runtime code, put modules in `engine/src/` and re-export as needed from `lib.rs`.
- Add dependencies in `Cargo.toml` at the crate that needs them.

---

**License**

See the `LICENSE` file in the repo.
