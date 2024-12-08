
#### ast/cell_path.rs

```rust
pub struct FullCellPath {
    pub head: Expression,
    pub tail: Vec<PathMember>,
}

/// Represents the potentially nested access to fields/cells of a container type
///
/// In our current implementation for table access the order of row/column is commutative.
/// This limits the number of possible rows to select in one [`CellPath`] to 1 as it could
/// otherwise be ambiguous
///
/// ```nushell
/// col1.0
/// 0.col1
/// col2
/// 42
/// ```
pub struct CellPath {
    pub members: Vec<PathMember>,
}
```
