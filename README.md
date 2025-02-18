# Multiple Mutable Borrows in Rust
This example demonstrates a common error in Rust: attempting to create multiple mutable borrows of the same variable.  Rust's borrow checker prevents this to avoid data races and ensure memory safety.  The solution involves refactoring to use a single mutable borrow or to pass ownership.