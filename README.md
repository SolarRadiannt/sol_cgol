# Conway's Game of Life — Rust / Bevy

**Status:** Completed

**Stack:** Rust, Bevy (ECS game engine)

## Overview
A fully interactive implementation of Conway's Game of Life built in Rust using the Bevy game engine. The simulation starts paused; you can spawn cells by holding `LMB` and toggle pause by clicking `RMB`.

## Controls
| Input | Output |
|-----------|---------|
| `LMB Hold` | Spawn cells |
| `RMB Click` | Pause and Unpause the game |

## Demo
<img width="662" height="550" alt="Gameplay demo: holding LMB spawns cells; clicking RMB toggles pause" src="https://github.com/user-attachments/assets/1478427e-701e-4eee-8d30-d8d7f2fedcf6" />

## How to Run
1. Install the Rust toolchain: https://rustup.rs
2. Clone the repo.
3. Run: `cargo run --release`

Alternatively, a pre-built `main.exe` is included in the release assets.

## Architecture & Design
- Built with **Bevy's ECS** architecture, separating game data (components) from behavior (systems).
- Core systems handle cell state updates, neighbor counting, and input for spawning/pausing.
- [bevy_pancam](https://github.com/johanhelsing/bevy_pancam) is utilized for the camera panning
