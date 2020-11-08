# My learning journey

Here some notes about each things I've learned about Rust.

- language:
    - helloworld
    - variables
    - ownership
    - conditions
    - pattern
    - pointers
    - collections
    - loops
    - functions
    - modules
    - structs
    - enums
    - errors
    - generics
    - traits
    - lifetime
- projects:
    - cli hangman game

#   Good Blogs:

- https://blog.yoshuawuyts.com/

#   Good videos:

- Idiomatic Rust: https://www.youtube.com/watch?v=P2mooqNMxMs

#   Good links:

- https://github.com/mre/idiomatic-rust
- https://github.com/rust-lang/rust-clippy

#   Having `rust-analyzer` working on VSCode on W10

Based on [this post from github](https://github.com/rust-lang/vscode-rust/issues/237#issuecomment-478299249)

```bash
rustup update

rustup install stable
rustup default stable
rustup component add rls rust-analysis rust-src
```


-   Uninstall Rust and rust-analyzer plugins in VSCode
-   restart VSCode
-   install the plugins
-   close VSCode

On your project run:

```bash
cargo check && cargo build && cargo doc on respository
```

Open VSCode again -> RLS is building


<!-- -   install `rust-analyzer` for VSCode
-   Settings ->
    -   Activate all available features
    -   "rust-client.channel": "nightly",
    -   "rust-client.engine": "rust-analyzer",
    -   "rust-analyzer.trace.extension": true,
    -   "rust-analyzer.trace.server": "verbose",

Here an example of the `rust-analyzer` json setting

```json
{
    "workbench.colorTheme": "Horizon",
    "rust-client.trace.server": "verbose",
    "rust-client.updateOnStartup": true,
    "rust.build_lib": false,
    "rust.rust-analyzer.releaseTag": "stable",
    "rust.rls.executable": "cargo",
    "rust.rls.args": null,
    "rust.rls.env": null,
    "editor.parameterHints.enabled": false,
    "files.eol": "\n",
    "debug.allowBreakpointsEverywhere": true,
    "editor.defaultFormatter": "rust-lang.rust",
    "editor.formatOnPaste": true,
    "editor.formatOnSave": true,
    "rust-client.channel": "stable",
    "editor.codeActionsOnSave": null,
    "window.zoomLevel": 0,
    "rust-client.engine": "rust-analyzer",
    "rust-analyzer.cargo.allFeatures": true,
    "rust-analyzer.cargo.target": null,
    "rust-analyzer.trace.extension": true,
    "rust-analyzer.trace.server": "verbose",
    "rust-analyzer.linkedProjects": [
        "absolute-path-to/Cargo.toml"
    ],
    "rust.rust-analyzer": {
        "releaseTag": "stable",
        "path": null
    }
}

``` -->