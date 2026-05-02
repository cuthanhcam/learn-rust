# Cargo and rustup

Rust comes with two essential tools to manage projects and toolchains: **Cargo** and **rustup**. Understanding these tools is fundamental to effective Rust development.

## What is Cargo?

**Cargo** is Rust's official build system and package manager. It simplifies project creation, dependency management, compilation, testing, and documentation generation. Every Rust project uses a `Cargo.toml` file (manifest) to define metadata, dependencies, and build settings.

### Key Features

- Automatic dependency management
- Standardized project structure
- One-command builds and runs
- Integrated testing and benchmarking
- Documentation generation

### Common Cargo Commands

#### `cargo new <project_name>`

**Purpose:** Create a new Rust project with the standard directory structure.

```bash
cargo new hello_world
```

**Output:**

```
     Created binary (application) `hello_world` package
```

**Creates:**

- `hello_world/` directory
- `Cargo.toml` (project manifest)
- `src/main.rs` (entry point with starter code)
- `.gitignore` file

---

#### `cargo build`

**Purpose:** Compile your project in debug mode (unoptimized, faster compile time).

```bash
cargo build
```

**Output:**

```
   Compiling hello_world v0.1.0
    Finished debug [unoptimized + debuginfo] target(s) in 0.42s
```

**When to use:**

- During development when you want fast compilation
- To check for compile errors
- To generate an executable in `target/debug/`

**For optimized builds:**

```bash
cargo build --release
```

This produces a slower-to-compile but faster-to-run binary in `target/release/`

---

#### `cargo run`

**Purpose:** Compile and immediately execute your project.

```bash
cargo run
```

**Output:**

```
   Compiling hello_world v0.1.0
    Finished debug [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/hello_world`
Hello, world!
```

**When to use:**

- Testing your code during development
- Running with specific arguments: `cargo run -- --arg value`
- Default choice for rapid iteration

---

#### `cargo test`

**Purpose:** Compile and run all tests in your project.

```bash
cargo test
```

**Output:**

```
   Compiling hello_world v0.1.0
    Finished test [unoptimized + debuginfo] target(s) in 0.52s
     Running unittests src/lib.rs

running 2 tests
test tests::it_works ... ok
test tests::it_fails ... ok

test result: ok. 2 passed; 0 failed; 0 ignored
```

**Common options:**

- `cargo test -- --nocapture` — Show print statements in tests
- `cargo test --release` — Run tests in optimized mode

---

#### `cargo add <crate_name>`

**Purpose:** Add a dependency to your project.

```bash
cargo add serde
```

**Effect:** Adds `serde = "1.0"` to your `Cargo.toml` and updates `Cargo.lock`

**Alternatives:**

- `cargo add serde@0.9` — Specific version
- `cargo add --dev criterion` — Add as dev dependency

---

#### `cargo update`

**Purpose:** Update all dependencies to their latest compatible versions.

```bash
cargo update
```

**When to use:**

- Regularly during development to get bug fixes and features
- Before shipping to production

**Be careful:** May introduce breaking changes if dependencies update major versions

---

#### `cargo fmt`

**Purpose:** Automatically format your code according to Rust style guidelines.

```bash
cargo fmt
```

**Effect:** Reformats all `.rs` files in place to match Rust's standard style

**Before committing code, always run this command to ensure consistency!**

---

#### `cargo clippy`

**Purpose:** Run Rust's linter to catch common mistakes and suggest improvements.

```bash
cargo clippy
```

**Output example:**

```
warning: this loop could be written as a `while let` loop
  --> src/main.rs:5:5
   |
5  |     loop {
   |     ^^^^ help: try: `while let Some(item) = iterator.next()`
```

**Strongly recommended:** Run before code reviews and before pushing to production

---

#### `cargo doc`

**Purpose:** Generate HTML documentation for your project and dependencies.

```bash
cargo doc --open
```

**Effect:** Generates docs in `target/doc/` and opens them in your browser

---

## What is rustup?

**rustup** is the official Rust toolchain installer and manager. It handles:

- Installing the Rust compiler (`rustc`)
- Managing multiple Rust versions (stable, beta, nightly)
- Installing additional targets (e.g., WebAssembly)
- Updating your Rust installation

### Common rustup Commands

#### `rustup update`

**Purpose:** Update Rust to the latest stable version.

```bash
rustup update
```

**Output:**

```
info: syncing channel updates for 'stable-x86_64-pc-windows-msvc'
info: latest toolchain for channel 'stable' is 1.75.0
info: downloading toolchain for 'stable-x86_64-pc-windows-msvc'
info: installing toolchain for 'stable-x86_64-pc-windows-msvc'
info: default toolchain set to 'stable-x86_64-pc-windows-msvc'
```

**Recommendation:** Run monthly to stay current with bug fixes and new features

---

#### `rustup show`

**Purpose:** Display your installed toolchains and active Rust version.

```bash
rustup show
```

**Output:**

```
active toolchain
----------------
stable-x86_64-pc-windows-msvc (default)
rustc 1.75.0 (1d8b05fc5 2023-12-21)

installed toolchains
--------------------
stable-x86_64-pc-windows-msvc (default)
beta-x86_64-pc-windows-msvc
nightly-x86_64-pc-windows-msvc
```

---

#### `rustup install <toolchain>`

**Purpose:** Install a specific Rust toolchain.

```bash
rustup install nightly
rustup install 1.70.0
```

**Common toolchains:**

- `stable` — Latest stable release (recommended for most projects)
- `beta` — Pre-release version (upcoming stable)
- `nightly` — Latest development version (unstable, for experimental features)

---

#### `rustup default <toolchain>`

**Purpose:** Switch your default Rust toolchain.

```bash
rustup default nightly
```

**When to use:**

- Project requires unstable features
- Need to test compatibility with upcoming versions
- For most projects, keep `stable` as default

**Project-specific override:**

```bash
rustup override set nightly
```

This applies only to the current directory and its subdirectories

---

#### `rustup self update`

**Purpose:** Update rustup itself (not Rust).

```bash
rustup self update
```

---

## Typical Workflow

1. **First-time setup:** Follow [Getting Started](getting_started.md)
2. **Monthly maintenance:** `rustup update` to get latest stable
3. **Create project:** `cargo new my_project`
4. **Add dependencies:** `cargo add serde` (as needed)
5. **Develop:** `cargo run` repeatedly
6. **Check quality:** `cargo clippy` and `cargo fmt`
7. **Test:** `cargo test`
8. **Release:** `cargo build --release`
