# Space Invaders (Terminal Game)

Welcome to Space Invaders, a simple space-themed game playable in the terminal.

## Getting Started

To start the game, run the following command inside the project directory:

```bash
cargo run
```

## Dependencies

This game is built using Rust and relies on the following external libraries:

- [crossterm](https://github.com/crossterm-rs/crossterm) - Cross-platform terminal manipulation.
- [rusty_time](https://crates.io/crates/rusty_time) - A simple time library for Rust.
- [crossbeam](https://crates.io/crates/crossbeam) - Concurrency primitives for Rust.
- [kira](https://crates.io/crates/kira) - A multiplatform audio library for Rust.

Make sure to include these dependencies in your `Cargo.toml` file:

```toml
[dependencies]
crossterm = "0.27.0"
rusty_time = "0.12"
crossbeam = "0.8.3"
kira = "0.8.5"
```
## Controls

- **A/D:** Move the player spaceship left/right.
- **Space:** Shoot projectiles to destroy incoming invaders.
- **Q:** Quit the game.

## License

This project is licensed under the following licenses:
 1. Apache License 2.0
 2. MIT License
