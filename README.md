## About

This is a [Zellij][zellij] plugin in Rust. 

[zellij]: https://github.com/zellij-org/zellij
[docs]: https://zellij.dev/documentation/plugins.html

## Development

*Note*: you will need to have `wasm32-wasi` added to rust as a target to build the plugin. This can be done with `rustup target add wasm32-wasi`.

## Image

<img width="1917" height="25" alt="image" src="https://github.com/user-attachments/assets/b033320e-458f-434e-a6f9-a369a1052fab" />

## How to use

`~/.config/zellij/layouts/defaults.kdl`
```kdl
layout {
    default_tab_template {
        children
        pane size=1 borderless=true {
            plugin location="file:~/.config/zellij/plugins/little-bar.wasm" {
            }
        }
    }
}
```

### Otherwise

1. Build the project: `cargo build`
2. Load it inside a running Zellij session: `zellij action start-or-reload-plugin file:target/wasm32-wasi/debug/rust-plugin-example.wasm`
3. Repeat on changes (perhaps with a `watchexec` or similar command to run on fs changes).
