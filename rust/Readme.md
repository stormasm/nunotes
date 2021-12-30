
[Slicing](https://doc.rust-lang.org/std/vec/struct.Vec.html#slicing)

A Vec can be mutable. On the other hand, slices are read-only objects. To get a slice, use &.

In Rust, it’s more common to pass slices as arguments rather than vectors when you just want to provide read access. The same goes for String and &str.

It took me awhile to understand this code in

 * nu-command/src/viewers/table.rs

In the example code below this is a read only vector [of Values] called a slice.

```rust
fn get_columns(input: &[Value]) -> Vec<String> {
    let mut columns = vec![];

    for item in input {
        if let Value::Record { cols, vals: _, .. } = item {
            for col in cols {
                if !columns.contains(col) {
                    columns.push(col.to_string());
                }
            }
        }
    }

    columns
}
```
