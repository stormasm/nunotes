
### Example of how to use from_value

```rust
let span = Span::unknown();
let val = Value::Int { val: 2, span };
let cell_path = CellPath::from_value(&val).unwrap();
```

from **nu-engine/src/from_value.rs**
```rust
impl FromValue for CellPath {
```
