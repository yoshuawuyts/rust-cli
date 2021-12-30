//! rust(1) CLI prototype
//!
//! # Goals
//!
//! - single interface to manage all of Rust
//! - uniform interface
//! - aware of workspaces out of the box
//!
//! # Design Decisions
//!
//! - `cargo fix` becomes a flag. `cargo fmt --fix`, `cargo lint --fix`, `cargo build --fix`
//! - `cargo lint` becomes a flag. `cargo check --lint`, `cargo bulid --lint`

#![forbid(unsafe_code, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, future_incompatible, unreachable_pub)]

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "rust", about = "The Rust Programming Language")]
struct Opt {
    /// Use a specific toolchain version.
    toolchain: Option<String>,
    #[structopt(subcommand)] // Note that we mark a field as a subcommand
    cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    /// Run the benchmarks
    Bench,
    /// Compile the current package
    Build,
    /// Check a local package for errors
    Check,
    /// Manage the local crates
    Crate(Crate),
    /// Compile the package documentation
    Doc,
    /// Run a binary or example of the local package
    Run,
    /// Manage the command itself
    _Self(Celf),
    /// Run a custom subcommand
    Task,
    /// Run the tests
    Test,
    /// Manage the local Rust toolchain
    Toolchain(Toolchain),
    /// Manage the current workspace
    Workspace(Workspace),
}

#[derive(Debug, StructOpt)]
enum Celf {
    Login,
    Logout,
    Config,
}

#[derive(Debug, StructOpt)]
enum Toolchain {
    Show,
    Update,
    Check,
    Default,
    Target,
    Component,
    Override,
    Run,
}

#[derive(Debug, StructOpt)]
enum Workspace {
    Clean,
    Diff,
    Exec,
    Generate,
    List,
    Publish,
    Version,
}

#[derive(Debug, StructOpt)]
enum Crate {
    Add,
    Install,
    Login,
    Remove,
    Search,
    Tree,
    Update,
    Upgrade,
    Vendor,
    Yank,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
