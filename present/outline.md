# rust-simple-db: A Lightweight Database in Rust

## Introduction

* **Motivation:** Why build a simple database in Rust?
    * Learning purposes: Demonstrate basic database concepts.
    * Educational tool:  Illustrate Rust features (structs, `impl` blocks, error handling, concurrency).
    * Lightweight solution:  Suitable for small projects or testing.

* **Features:**
    * In-memory key-value store.
    * Thread-safe with `Arc` and `Mutex`.
    * JSON persistence (`to_json`, `from_json`).

## Code Overview

* **`Database` struct:**
    * `data`:  `HashMap` to store key-value pairs.
    * `mutex`: `Mutex` for thread safety.

* **Methods:**
    * `new()`: Creates a new `Database` instance.
    * `insert()`: Inserts a key-value pair.
    * `get()`: Retrieves a value by key.
    * `insert_many()`: Inserts multiple key-value pairs.
    * `to_json()`: Saves the database to a JSON file.
    * `from_json()`: Loads the database from a JSON file.

## Example Usage

* **Creating a database:**

```rust
let db = Database::new();