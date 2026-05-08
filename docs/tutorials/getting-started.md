# Getting Started with Rust

## What is Rust?

Rust is a modern systems programming language that provides memory safety without sacrificing performance. It's designed to prevent entire classes of bugs at compile-time while offering performance comparable to C and C++. Rust emphasizes three core principles: safety, speed, and concurrency.

## Before You Begin

Before writing Rust code, you'll need to set up Rust on your computer. This section covers:

- Installing Rust and rustup
- Setting up your development environment
- Verifying your installation

## Installing Rust

To install Rust, you'll use **rustup**, which is the official and recommended tool for managing Rust versions. This ensures you always have the latest stable compiler.

### Step 1: Install rustup

1. Visit the official [Rust installation page](https://www.rust-lang.org/tools/install)
2. Follow the platform-specific instructions:

   **Windows:**
   - Download and run [Rustup-init.exe](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe)
   - Follow the on-screen prompts
   - The installer will add Rust to your system PATH automatically

   **Linux/macOS:**
   - Open a terminal and run:
     ```bash
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```
   - Follow the prompts and add Rust to your PATH when prompted

### Step 2: Verify Installation

After installing, open a new terminal (or restart your current one) and verify the installation:

```bash
rustc --version
```

Expected output:

```
rustc 1.95.0 (59807616e 2026-04-14)
```

Also verify Cargo (Rust's package manager, installed with rustup):

```bash
cargo --version
```

Expected output:

```
cargo 1.95.0 (f2d3ce0bd 2026-03-21)
```

## Setting Up Your Development Environment

### Recommended: Visual Studio Code (VSCode)

1. **Download VSCode:**
   - Go to [code.visualstudio.com](https://code.visualstudio.com/) and install it for your platform

2. **Install Essential Rust Extensions:**
   - Open VSCode and press `Ctrl+Shift+X` (or `Cmd+Shift+X` on macOS) to open the Extensions marketplace
   - Search for and install:
     - **rust-analyzer** (by The Rust Programming Language) — Provides intelligent code analysis, autocompletion, and error detection
     - **CodeLLDB** (Optional) — Enables debugging Rust code

### Alternative Editors

Rust is well-supported in other editors:

- JetBrains IntelliJ IDEA with the Rust plugin
- Vim/Neovim with rust.vim
- Sublime Text with rust enhancements

## Verifying Your Complete Setup

To confirm everything is working correctly:

1. **Create a test project:**

   ```bash
   cargo new test_project
   cd test_project
   ```

2. **Run the default program:**

   ```bash
   cargo run
   ```

   Expected output:

   ```
      Compiling test_project v0.1.0
       Finished dev [unoptimized + debuginfo] target(s) in 0.50s
       Running `target/debug/test_project`
   Hello, world!
   ```

3. **Open in VSCode:**
   ```bash
   code .
   ```

If you see "Hello, world!" printed, your Rust environment is properly configured! You're now ready to move on to [Cargo and rustup](cargo_and_rustup.md) to learn more about project management.

## Troubleshooting

**"rustc: command not found"**

- Restart your terminal or IDE after installing rustup
- On Windows, ensure the Rust installer added it to PATH

**Extension not working in VSCode**

- Ensure rust-analyzer is installed (not rust extension)
- Restart VSCode after installing the extension

**Cargo build is slow**

- This is normal on first build; subsequent builds are much faster due to caching
- Debug builds are faster; use `cargo build --release` for optimized builds (slower compile, faster execution)
