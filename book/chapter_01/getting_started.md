# Getting Started

## Install Rust
https://doc.rust-lang.org/book/ch01-01-installation.html

I choose `rustup` to install Rust and associated tools.

Versions when I wrote this:
|tool|version|
|-|-|
|rustup|1.27.0|
|rustc|1.77.0|
|cargo|1.77.0|

## Hello, World!

#### **01_hello_world.rs**
```rust
fn main() {
    println!("Hello, world!");
}
```

This looks mostly similar in every language. The only peculiarity here is the exclamation mark `!` behind the `println` which indicates this is not a `function` but a `macro`. The specialities of macros will be explain in a later chapter.

Running `rustc 01_hello_world.rs` creates file `01_hello_world` which can be executed using `./01_hello_world`

## Hello, Cargo!

Cargo is Rusts build system and package manager.

I'll create a *Hello, World!* like program with `cargo`

```sh
cargo new hello_cargo
cd hello_cargo
```

Open `Cargo.toml`, which looks like this:
```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

Obviously, Cargo config is written in TOML.
The `[package]` section contains information about my cargo package. `name` and
`version` are obvious. What `edition` will be explained later in the book.
`[dependencies]` is a list of packages from the *crates* repository. Here
nothing is needed.

Cargo expects the code in `src/main.rs`. I can build it with `cargo build` and
run with (surprise) `cargo run`.
```sh
cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/hello_cargo`
Hello, world!
```

`cargo check` checks the code to make sure it compiles without execution.
This is useful because it is much faster than `cargo build`. If the project
gets bigger the compile time increases, so it can quickly be checked if it
would still compiles.

When the project is ready for release it can be build with `cargo build
--release`.

