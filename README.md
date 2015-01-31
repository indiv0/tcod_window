# tcod_window
A TCOD back-end for the Piston game engine

Maintainers: @indiv0

# Installation

To use this as a dependency, add the following code to your Cargo.toml
file:

```toml
[dependencies.tcod_window]
git = "https://github.com/indiv0/tcod_window"
```

## Creating a Window

```rust
let mut window = TcodWindow::new(
    WindowSettings {
        title: "My Application".to_string(),
        size: [80, 50],
        fullscreen: false,
        exit_on_esc: true,
        samples: 0,
    }
);
```
