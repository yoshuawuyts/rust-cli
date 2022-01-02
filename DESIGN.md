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
├── debug      Run the interactive debugger
├── dep        Manage the local dependencies
│   ├── audit       Create a report on your local dependencies
│   ├── add         Add a dependency to matched packages
│   ├── clean       Remove all local dependencies
│   ├── fetch       Fetch dependencies from the network
│   ├── remove      Remove a local dependency
│   ├── tree        List all installed dependencies
│   ├── update      Update dependencies as recorded in the local lock file
│   ├── upgrade     Upgrade dependencies as specified in the local manifest file
│   ├── vendor      Vendor all dependencies for a project locally
│   ├── prune       Remove all unused dependencies from the project
│   └── why         Show why a package was required
├── doc          Compile the package documentation     
├── fmt          Format the source code
├── lint         Check for common issues in Rust source code
├── profile      Run the interactive profiler
├── registry     Manage the registry
│   ├── cache       Manage the local registry cache
│   ├── login       Login to a registry
│   ├── logout      Log out from a registry
│   ├── owner       Manage the owner of a crate
│   ├── search      Search for a crate
│   ├── publish     Publish a crate to the registry
│   └── yank        Remove a pushed crate from the index
├── run          Run a binary or example of the local package
├── self         Manage the rust command itself
│   ├── set         Modify a local setting
│   └── component   Manage locally installed components
├── server       Run the LSP language server
├── task         Run a custom subcommand
│   └── list        List all tasks
├── test         Run the tests
│   ├── audit       Validate all local audit rules
│   ├── compile     Run compile tests
│   ├── doc         Run documentation tests
│   ├── fuzz        Run fuzz tests
│   ├── lint        Test for common issues in Rust source code
│   ├── fmt         Validate the formatting
│   ├── shrink      Shrink a rust source program, preserving interesting properties
│   ├── snapshot    Run snapshot tests
│   └── unit        Manage the owner of a crate
├── toolchain    Manage the local Rust toolchain       
│   ├── check       Check for updates to locally installed toolchains
│   ├── update      Update locally installed toolchains
│   ├── override    Override the currently used toolchain
│   ├── create      Create a custom toolchain
│   ├── list        List installed toolchains
│   ├── add         Install or update a toolchain
│   └── remove      Remove an installed toolchain
└── workspace    Manage the current workspace
│   ├── add         Add a new package to the local workspace
│   ├── init        Create a new workspace
│   ├── publish     Publish packages in the current project
│   ├── version     Bump version of packages changed since the last release
│   ├── list        List local packages
│   ├── changed     List local packages that have changed since the last release
│   ├── diff        Diff local packages that have changed since the last release
│   ├── exec        Execute an arbitrary command in each package
│   └── link        Link local packages as dependencies of each other
```

Universal flags
```txt
--debug        Spin up a debugger on a breakpoint
--profile      Profile the operation
--backtrace    Generate a backtrace on crash
--fix          `rust test` commands should all be auto-fixable
--toolchain    To select a toolchain
-q, --quiet    To suppress all output
-v, --verbose  To enable verbose output
-h, --help     Print help information
```

Contentious topics
- Should `rust(1)` act as a system-wide package manager?
    - It competes with the system package manager
    - Makes it easy to publish rust tools
    - But does make `docs.rs` often a bit, uh, lackluster
    - But usually far easier to publish to all the platforms to than using
      system package managers
    -Solution: spin this out into its own subcommand
