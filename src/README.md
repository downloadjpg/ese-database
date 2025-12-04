# In Memory Key-Value Database with Transactions
This is an in-memory key-value database with transaction support, implemented in Rust. It allows users to perform basic operations such as setting, getting, and deleting key-value pairs, as well as managing transactions with commit and rollback functionality.

## Instructions
To run this code, you need to have Rust installed on your machine. You can download and install Rust from [here](https://www.rust-lang.org/tools/install).

To use this library in your own Rust project, simply add it as a dependency in your `Cargo.toml` file. You can then create an instance of the `Database` struct and use its methods to interact with the key-value store.

```rust
// cargo.toml
[dependencies]
ese-database = { git = "https://github.com/downloadjpg/ese-database.git" }
```

```rust
// main.rs
use ese_database::Database;
fn main() {
    let mut db = Database::new();
    db.set("key1".to_string(), 42).unwrap();
    let value = db.get("key1").unwrap();
    println!("The value for 'key1' is: {}", value);
}
```

Alternatively, you can clone the repository and run the tests to see how the database works:

```bash
cargo test
```


## Feedback for Assignment
I enjoy this assignment, particularly the freedom in selecting a language. I would lean in to this when expanding the assignment in the future, perhaps by adding an automated testing suite that uses command line inputs and outputs to allow for any language to be used. (This would mean changing the assignment from writing a library to an actual binary that accepts command line args as well.) To further expand, I would add more transaction features, such as removing elements or nested transactions.