# Module 1: Beginner

<br>


### Rust is an ahead-of-time compiled language

* The `main` function in Rust is always the first code that runs in every executable Rust program.

* In any directory, compile and run a Rust program with:


```sh
rustc main.rs
./main
```


<br>

---

### Cargo

* Cargo is a dependency manager and build tool, which makes adding, compiling, and managing dependencies painless and consistent across the Rust ecosystem.

* A crate is a collection of Rust source code files. Cargo coordinates external crates in the `Cargo.toml` file. You can add a new crate with `cargo add <crate name>` and update with `cargo update <crate name>`.


#### Creating a new project

```sh
cargo new gm_world
```

* This command creates a cargo TOML file and a placeholder for `main.rs`:

```sh
.
├── Cargo.toml
└── src
    └── main.rs
```


#### Building and running with Cargo

* Alternatively, in any directory, compile and run a Rust program with:


```sh
cargo build
```

* This command creates a file named `Cargo.lock` (which keeps track of the versions for all dependencies), and an executable inside `target/debug/` that can be ran. 

* If you are building for release or benchmarking, add the flag `--release` to compile it with optimizations (and the target will be `release`).

* Additionally, you can build and run with:

```sh
cargo run
```

* Finally, to make sure your code compile in a fast manner without executing it, you can run:

```sh
cargo check
```



<br>

---

### Rustfmt

* A formatting tool ensuring a consistent coding style across developers.

<br>

---

### Macros

* A `!` (as in `println!`) calls a Rust macro.

<br>

---

### Variables

* Variables and references are immutable by default.
* Constants are declared with `const` and always immutable.

<br>

----

### Results

* Rust handles potential failure with `Result`, which is an `enumeration` (`enum`, a type that can be in one of multiple possible states or variant).

