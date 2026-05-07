# Rust Naming Conventions

Rust has specific, community-established guidelines for naming variables, functions, types, and other code elements. Following these conventions improves code readability, makes your code more idiomatic, and helps other Rust developers quickly understand your intent.

## Why Naming Conventions Matter

- **Readability:** Consistent naming makes code easier to scan and understand
- **Maintainability:** Team members can quickly identify what something is (variable vs constant vs type)
- **Idiomaticity:** Following Rust conventions signals you understand the language
- **Tool Support:** `rustfmt` and `clippy` enforce these conventions automatically

## Naming Styles Overview

Rust uses different naming conventions for different program elements:

| Element      | Style                  | Example                                |
| ------------ | ---------------------- | -------------------------------------- |
| Variables    | `snake_case`           | `let user_name = "Alice";`             |
| Functions    | `snake_case`           | `fn calculate_total()`                 |
| Constants    | `UPPERCASE_SNAKE_CASE` | `const MAX_BUFFER_SIZE: usize = 1024;` |
| Structs      | `UpperCamelCase`       | `struct UserProfile`                   |
| Enums        | `UpperCamelCase`       | `enum Color { Red, Green }`            |
| Traits       | `UpperCamelCase`       | `trait Iterator`                       |
| Modules      | `snake_case`           | `mod user_management`                  |
| Type aliases | `UpperCamelCase`       | `type UserId = u64;`                   |
| Methods      | `snake_case`           | `fn get_name()`                        |

---

## Detailed Conventions

### 1. snake_case for Variables

**Usage:** Variable names, bindings, and local identifiers

**Rules:**

- All lowercase
- Words separated by underscores
- Descriptive and meaningful

**Good Examples:**

```rust
let user_age = 25;
let is_admin = true;
let total_balance = 1000.50;
let first_name = "Alice";
```

**Avoid:**

```rust
let userAge = 25;       // Camel case
let user_a = 25;        // Not descriptive
let a = 25;             // Too short
let UAB = 25;           // All caps
```

---

### 2. snake_case for Functions

**Usage:** Function and method definitions

**Rules:**

- All lowercase
- Words separated by underscores
- Describe what the function does
- Use verbs or verb phrases

**Good Examples:**

```rust
fn calculate_tax(amount: f64) -> f64 { ... }
fn is_valid_email(email: &str) -> bool { ... }
fn fetch_user_data() { ... }
fn set_timeout(ms: u64) { ... }
```

**Avoid:**

```rust
fn Calculate_Tax() { }        // Camel case
fn calc() { }                 // Too short, unclear
fn CTax() { }                 // Mixed case
fn computeTaxForUser() { }    // Camel case
```

**Naming Patterns:**

- Prefix with `is_`, `has_`, `can_` for boolean returns:
  ```rust
  fn is_valid() -> bool { ... }
  fn has_permission() -> bool { ... }
  fn can_edit() -> bool { ... }
  ```

---

### 3. UpperCamelCase for Types (Structs, Enums, Traits)

**Usage:** Type definitions like structs, enums, and trait names

**Rules:**

- First letter capitalized
- No underscores between words
- Each word capitalized
- Nouns or noun phrases

**Good Examples:**

```rust
struct UserProfile { ... }
struct HttpRequest { ... }
struct DatabaseConnection { ... }

enum Color { Red, Green, Blue }
enum RequestStatus { Pending, Approved, Rejected }

trait Iterator { ... }
trait DatabaseAdapter { ... }
```

**Avoid:**

```rust
struct user_profile { }      // snake_case
struct User_Profile { }      // Mixed case
enum color { }              // Lowercase
trait database_adapter { }   // snake_case
```

---

### 4. UPPERCASE_SNAKE_CASE for Constants

**Usage:** Constant values that don't change at runtime

**Rules:**

- All uppercase
- Words separated by underscores
- Must explicitly specify the type

**Good Examples:**

```rust
const MAX_CONNECTIONS: u32 = 100;
const DEFAULT_TIMEOUT_MS: u64 = 5000;
const DATABASE_URL: &str = "postgresql://localhost";
const VERSION: &str = "1.0.0";
```

**Avoid:**

```rust
const maxConnections: u32 = 100;  // Camel case
const max_connections = 100;       // Lowercase, no type
const MaxConnections: u32 = 100;   // Camel case
```

**Distinction:**

- Constants are compile-time values: `const MAX_SIZE: usize = 100;`
- Immutable variables: `let max_size = 100;` (use snake_case)

---

### 5. snake_case for Modules and Files

**Usage:** Module names and source file names

**Rules:**

- All lowercase
- Words separated by underscores
- Single file: `src/user_management.rs`
- Module directory: `src/user_management/mod.rs`

**Good Examples:**

```
src/
├── main.rs
├── user_management.rs      // Filename
├── database_connection.rs  // Filename
├── api/
│   └── mod.rs
└── models/
    └── mod.rs
```

**In code:**

```rust
mod user_management;  // Matches user_management.rs
mod database;         // Matches database.rs

use user_management::User;
```

**Avoid:**

```
UserManagement.rs        // Camel case filename
user-management.rs       // Hyphens
USER_MANAGEMENT.rs       // All caps
```

---

### 6. Type Aliases with UpperCamelCase

**Usage:** Custom type names

**Example:**

```rust
type UserId = u64;
type Callback = fn() -> ();
type Result<T> = std::result::Result<T, Error>;
```

---

### 7. Acronyms in Names

**Handle acronyms as single words, capitalize only the first letter:**

**Good:**

```rust
struct HttpRequest { }      // Not HTTPRequest
fn parse_json() { }         // Not parse_JSON
struct SqlDatabase { }      // Not SQLDatabase
```

**Avoid:**

```rust
struct HTTPRequest { }      // Wrong
fn parse_JSON() { }         // Wrong
```

---

## Code Organization Example

```rust
// Module name (file: user_service.rs)
pub mod user_service {
    // Constants
    pub const DEFAULT_USER_TIMEOUT_MS: u64 = 5000;

    // Struct (type)
    pub struct User {
        id: u64,
        full_name: String,
        is_active: bool,
    }

    // Enum (type)
    pub enum UserRole {
        Admin,
        Editor,
        Viewer,
    }

    // Trait (type)
    pub trait UserRepository {
        fn get_user(&self, user_id: u64) -> Option<User>;
    }

    // Function
    pub fn create_user(full_name: &str) -> User {
        User {
            id: generate_id(),
            full_name: full_name.to_string(),
            is_active: true,
        }
    }

    // Helper function
    fn generate_id() -> u64 {
        // Implementation
        42
    }
}
```

---

## Automatic Enforcement

### rustfmt: Automatic Code Formatting

`rustfmt` automatically fixes many naming and formatting issues:

```bash
cargo fmt
```

This command reformats all Rust files to follow community standards.

**Always run before committing code!**

### clippy: Linting and Suggestions

`clippy` catches naming anti-patterns and suggests improvements:

```bash
cargo clippy
```

**Example warning:**

```
warning: variable name starts with underscore
  --> src/main.rs:5:9
   |
5  |     let _unused_var = 5;
   |
   = note: consider using `_` or a less obvious name
```

---

## Common Mistakes to Avoid

| ❌ Wrong                | ✅ Correct                   | Why                                |
| ----------------------- | ---------------------------- | ---------------------------------- |
| `let userName = 5;`     | `let user_name = 5;`         | Variables use snake_case           |
| `fn getUserAge() { }`   | `fn get_user_age() { }`      | Functions use snake_case           |
| `struct user { }`       | `struct User { }`            | Structs use UpperCamelCase         |
| `const max_size = 100;` | `const MAX_SIZE: u32 = 100;` | Constants use UPPERCASE_SNAKE_CASE |
| `enum userRole { }`     | `enum UserRole { }`          | Enums use UpperCamelCase           |
| `mod UserModule { }`    | `mod user_module { }`        | Modules use snake_case             |

---

## Best Practices

1. **Be Descriptive:** Use clear, meaningful names that explain intent

   ```rust
   // Good
   let user_age = 25;

   // Avoid
   let age = 25;        // Too generic
   let a = 25;          // Too short
   ```

2. **Use Domain Language:** Use terminology from your domain

   ```rust
   // For a web API:
   struct HttpRequest { }
   struct JsonResponse { }

   // For a game:
   struct Player { }
   struct GameObject { }
   ```

3. **Avoid Redundancy:** The type already indicates the context

   ```rust
   // Good
   struct User { name: String }

   // Avoid
   struct User { user_name: String }  // Redundant "user_"
   ```

4. **Keep It Short:** Balance clarity with conciseness

   ```rust
   // Good
   let total_cost = 100.0;

   // Avoid (too long)
   let the_total_cost_of_all_items = 100.0;
   ```

5. **Use Verb Phrases for Functions:** Start with action words
   ```rust
   fn calculate_total() { }    // Good
   fn get_user_data() { }      // Good
   fn validate_input() { }     // Good
   fn total() { }              // Unclear
   fn user_data() { }          // Sounds like a variable
   ```

---

## Quick Reference

- **Variables & Functions:** `snake_case`
- **Types (struct, enum, trait):** `UpperCamelCase`
- **Constants:** `UPPERCASE_SNAKE_CASE`
- **Modules & Files:** `snake_case`
- **Acronyms:** Treat as single word `HttpRequest`, not `HTTPRequest`

---

## Tools to Help

- `cargo fmt` — Auto-formats all code
- `cargo clippy` — Catches naming and style issues
- VS Code with rust-analyzer — Real-time feedback on naming

**Next:** Apply these conventions to your code and always run `cargo fmt` before committing!
