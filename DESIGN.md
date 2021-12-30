# Workflows

1. Manage all dependencies installed on the current machine
    - [cargo-wipe](https://github.com/mihai-dinculescu/cargo-wipe) to remove all installed crates.
2. Manage local dependencies for the current project
    - [cargo-tree](https://doc.rust-lang.org/cargo/commands/cargo-tree.html) to show installed deps
    - [cargo-geiger](https://docs.rs/cargo-geiger/latest/cargo_geiger/) to audit unsafe crates
    - [cargo-audit](https://docs.rs/cargo-audit/0.16.0/cargo_audit/index.html) to audit for security vulnerabilities
    - [cargo add](https://github.com/killercup/cargo-edit)
    - cargo rm
    - cargo upgrade
    - cargo set-version
3. Run a sandbox which we could use for build scripts
4. Better testing facilities
    - `cargo-fuzz`
    - `cargo-clippy`
    - [`loom`](https://github.com/tokio-rs/loom)
    - [criterion](https://github.com/bheisler/criterion.rs)
5. workspace tools
    - [cargo-workspaces](https://github.com/pksunkara/cargo-workspaces)
6. publishing tools
    - [cargo-breaking](https://twitter.com/jcsvveiga/status/1474390696572424192)
    - [cargo-release](https://crates.io/crates/cargo-release)
7. Debugging
    - [cargo-expand](https://github.com/dtolnay/cargo-expand)
    
# Commands

```txt
rust
├── bench      Run the benchmarks
├── build      Compile the current package
├── check      Check a local package for errors      
├── crate      Manage the local crates
│   ├──  add        Add a dependency
│   ├──  install    Install a crate globally
│   ├──  login      Login to crates.io
│   ├──  remove     Remove a local crate
│   ├──  search     Search for a crate
│   ├──  tree       List all installed crates
│   ├──  update     Update dependencies as recorded in the local lock file
│   ├──  upgrade    Upgrade dependencies as specified in the local manifest file
│   ├──  vendor     Vendor all dependencies for a project locally
│   └──  yank       Remove a pushed crate from the index
├── doc          Compile the package documentation     
├── run          Run a binary or example of the local package
├── self         Manage the command itself
├── task         Run a custom subcommand
├── test         Run the tests
├── toolchain    Manage the local Rust toolchain       
└── workspace    Manage the current workspace
```
