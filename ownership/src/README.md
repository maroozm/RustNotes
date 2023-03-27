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

## Dereferences
```rust
let x: Box<i32> = Box::new(-1);
let x_abs1 = i32::abs(*x); // explicit dereference
let x_abs2 = x.abs();      // implicit dereference
assert_eq!(x_abs1, x_abs2);

let r: &Box<i32> = &x;
let r_abs1 = i32::abs(**r); // explicit dereference (twice)
let r_abs2 = r.abs();       // implicit dereference (twice)
assert_eq!(r_abs1, r_abs2);

let s = String::from("Hello");
let s_len1 = str::len(&s); // explicit reference
let s_len2 = s.len();      // implicit reference
assert_eq!(s_len1, s_len2);
```
## Slices
*Slices* let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it does not have ownership.
**Problem: write a function that takes a string of words separated by spaces and returns the first word it finds in that string?**
```rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);
    s.clear();
}
```
`as_bytes` converts the string into a byte array. `enumerate` creates an iterator that returns a tuple where the first element is the index and the second element is a reference to the element. `iter` creates an iterator over the slice, thats why we used `i` and `&item`. Inside the loop, we check if the byte is a space using the byte literal syntax, which is `b' '`. If it is, we return the index. If we don't find a space, we return the length of the string. We could use `word` with value `5` after `s.clear()` and use it with `s` to get the first word. But this will be a bug because `s` is changed after `word` gets a `5` value, plus the function `first_word` gets complicated if we need to use second string. In rust, we can use slices to solve this problem.
```rust
## StringSlices
