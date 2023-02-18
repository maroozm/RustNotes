```rust
fn read(y: bool) {}

fn main() {
    read(x);
    let x = true;
  
    let x = true;
    read(x);
}
```
- This will throw errors (and the pr0gram is unsafe) because read() fn expects x to be a bool, but x hasn't been defined on line 4. the right way is after line 7 ad 8.

-----

```rust
fn main() {  
    let first = String::from("Ferris");  
    let full = add_suffix(first);  
    println!("{full}");  
    }  
fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
    }
```
1. At L1, the string `"Ferris"` has been allocated on the heap. It is owned by `first`.
2. At L2, the function `add_suffix(first)` has been called. This moves ownership of the string from first to name. The string data is not copied, but the pointer to the data is copied.
3. At L3, the function `name.push_str(" Jr.")` resizes the string's heap allocation. This frees the original heap memory, creates a new allocation, and writes `"Ferris Jr."` into the new location. first now points to deallocated memory.
4. At L4, the frame for `add_suffix` is gone. This function returned name, transferring ownership of the string to `full`.

## Summary
- All heap data must be owned by exactly one variable.
- Rust deallocates heap data once its owner goes out of scope.
- Ownership can be transferred by moves, which happen on assignments and function calls.
- Heap data can only be accessed through its current owner, not a previous owner.

