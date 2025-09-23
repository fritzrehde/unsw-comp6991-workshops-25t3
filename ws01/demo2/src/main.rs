// to cover:

// all builds stored in one dir (easy to delete): `export CARGO_TARGET_DIR="$HOME/cargo-targets"`

// ownership rules:
// - Each value in Rust has **one** owner.
// - There can only be one owner at a time.
// - When the owner goes out of scope, the value will be dropped.
// use after free, double free, dangling pointers (next week), thread safety

// immutable vs mutable

// how can we simulate referencing objects temporarily (both immut and mut) without proper rust references?

// structs, code blocks, enums, pattern matching, errors as values, unwrapping

// () unit type (and value), demo match returning () or {}

// if, match are expressions (return value)

// ranges

// &str vs String

fn main() {
    println!("hello world");
}

