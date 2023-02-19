```rust
fn read(y: bool) {}

fn main() {
    read(x);
    let x = true;
  
    let x = true;
    read(x);
}
```
- This will throw errors (and the program is unsafe) because read() fn expects x to be a bool, but x hasn't been defined on line 4. the right way is after line 7 ad 8.

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

##### Summary upto now
- All heap data must be owned by exactly one variable.
- Rust deallocates heap data once its owner goes out of scope.
- Ownership can be transferred by moves, which happen on assignments and function calls.
- Heap data can only be accessed through its current owner, not a previous owner.

## References and Borrowing
```rust
    fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(m1, m2);
    let s = format!("{} {}", m1, m2);
    }
    fn greet(g1: String, g2: String) {
    println!("{} {}!", g1, g2);
    }
```
`greet` moves the strings from `m1` and `m2` into the parameters of `greet` and thus cannot be used within the `main`. In case we want to read these *strings* twice, we can do the following:
```rust
    fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    let (m1_new, m2_new) = greet(m1, m2);
    let s = format!("{} {}", m1_new, m2_new);
    }
    fn greet(g1: String, g2: String) -> (String, String) {
    println!("{} {}!", g1, g2);
    (g1, g2)
    }
```
Here, we are returning the ownership of both the strings to `m1_new` and `m2_new`. However, this style of program is quite verbose. Rust provides a more concise style of reading and writing without moves through references.
**References** are kind of pointers here. Here's an example:
```rust
    fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(&m1, &m2); // note the ampersands
    let s = format!("{} {}", m1, m2);
    }
    fn greet(g1: &String, g2: &String) { // note the ampersands
    println!("{} {}!", g1, g2);
    }
```
`&m1` creates a reference to `m1`. The type of the `greet` parameter `g1` is changed to `&String`, meaning "a reference to a String". `g1` in `greet` is a pointer to `m1` (which is on the stack) that containg the string "Hello" (which is on the heap).
While `m1` owns the heap data "Hello", `g1` does not own either `m1` or "Hello". Therefore after greet ends, no heap data has been deallocated. Only the stack frame for greet disappears. This fact is consistent with our *Moved Heap Data Principle*: because `g1` did not own "Hello", Rust did not deallocate "Hello" on behalf of `g1`.
Thus, references are **non-owning pointers**, because they do not own the data they point to.


