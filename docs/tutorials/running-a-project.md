# Running a Rust Project

This guide covers the practical aspects of building, testing, and executing Rust projects. For detailed command references, see [Cargo and rustup](cargo_and_rustup.md).

## Project Structure

When you create a new project with `cargo new my_project`, Cargo sets up this standard structure:

```
my_project/
├── Cargo.toml           # Project manifest with metadata and dependencies
├── Cargo.lock           # Lock file (auto-generated after first build)
├── src/
│   └── main.rs          # Entry point for executable projects
├── target/              # Build output directory (auto-generated)
├── .gitignore           # Git ignore file
└── README.md            # Project documentation
```

## Build Modes

Rust supports two primary build modes, each with different optimization levels and trade-offs:

### Debug Build (Development)

**Purpose:** Fast compilation for testing during development

```bash
cargo build
```

**Characteristics:**

- Compiled with minimal optimizations
- Includes debug symbols (larger binary)
- Compilation speed: Very fast
- Runtime performance: Slower
- Output: `target/debug/<project_name>`

**When to use:**

- During active development
- When iteration speed matters
- For testing and debugging
- Default choice for `cargo run`

**Example run:**

```bash
cargo run
```

### Release Build (Production)

**Purpose:** Optimized, production-ready executables

```bash
cargo build --release
```

**Characteristics:**

- Compiled with aggressive optimizations
- Excludes debug symbols (smaller binary)
- Compilation speed: Slower (can be 5-10x slower)
- Runtime performance: Much faster (2-10x improvement)
- Output: `target/release/<project_name>`

**When to use:**

- Before shipping to production
- Performance benchmarking
- Distribution to users
- When file size matters

**Example run:**

```bash
cargo build --release
./target/release/my_project  # On Windows: .\target\release\my_project.exe
```

## Running Projects

### Basic Execution

**Compile and run in debug mode (single command):**

```bash
cargo run
```

**With arguments:**

```bash
cargo run -- argument1 argument2
```

Note: Arguments after `--` are passed to your program, not to Cargo

**Compile and run in release mode:**

```bash
cargo run --release
```

### Direct Binary Execution

After building, you can run the binary directly:

**Debug binary:**

```bash
# Linux/macOS
./target/debug/my_project

# Windows
.\target\debug\my_project.exe
```

**Release binary:**

```bash
# Linux/macOS
./target/release/my_project

# Windows
.\target\release\my_project.exe
```

## Testing Your Project

### Running Tests

Run all tests:

```bash
cargo test
```

**Sample output:**

```
   Compiling my_project v0.1.0
    Finished test [unoptimized + debuginfo] target(s) in 0.42s
     Running unittests src/lib.rs

running 3 tests
test result::add ... ok
test result::subtract ... ok
test integration::add_then_subtract ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured
```

### Run Specific Test

```bash
# Run tests matching a name pattern
cargo test test_name

# Example: run all tests starting with "add"
cargo test add
```

### Show Test Output

By default, Rust captures and hides output from passing tests. To see it:

```bash
cargo test -- --nocapture
```

### Run Tests in Release Mode

For performance testing:

```bash
cargo test --release
```

## Code Quality Checks

### Format Your Code

Automatically format all Rust code to follow style guidelines:

```bash
cargo fmt
```

**Best practice:** Always run this before committing code

### Lint with Clippy

Run the Rust linter to catch mistakes and suggest improvements:

```bash
cargo clippy
```

**Example output:**

```
warning: this loop could be written as a `while let` loop
  --> src/main.rs:10:5
   |
10 |     loop {
   |     ^^^^ help: try: `while let Some(item) = iterator.next()`
   |
   = note: `#[warn(clippy::while_let_loop)]` implied by `-W clippy::pedantic`
```

**Best practice:** Address clippy warnings before code reviews

## Common Workflows

### Development Loop

1. Write/modify code in `src/main.rs`
2. Test with `cargo run`
3. Fix compilation errors
4. Format with `cargo fmt`
5. Check with `cargo clippy`
6. Run tests with `cargo test`
7. Repeat from step 1

### Before Committing

```bash
cargo fmt          # Auto-format code
cargo clippy       # Check for improvements
cargo test         # Ensure tests pass
cargo build        # Final check
```

### Before Shipping to Production

```bash
cargo test --release           # Test in release mode
cargo build --release          # Create optimized binary
cargo clippy --all-targets     # Full lint check
cargo doc --open               # Generate and review documentation
```

## Project Output Structure

After building, your `target/` directory contains:

```
target/
├── debug/
│   ├── my_project              # Debug executable
│   ├── deps/                   # Compiled dependencies
│   └── incremental/            # Build cache for faster rebuilds
├── release/
│   ├── my_project              # Release executable (optimized)
│   ├── deps/                   # Optimized dependencies
│   └── incremental/            # Build cache
└── doc/                         # Generated documentation
```

**Note:** You can safely delete the `target/` directory—Cargo will regenerate it on the next build.

## Troubleshooting

### "could not compile `project_name`"

- Check error messages for specific issues
- Verify you're in the correct directory
- Try `cargo clean && cargo build` to clean and rebuild

### Binary not found after build

- Ensure you used `cargo build` or `cargo run` first
- Check that output is in `target/debug/` (debug) or `target/release/` (release)
- Verify your project type (binary vs library)

### Tests fail but code runs

- Tests may have different requirements or setup
- Use `cargo test -- --nocapture` to see detailed output
- Check test code for assertions or expected behaviors
