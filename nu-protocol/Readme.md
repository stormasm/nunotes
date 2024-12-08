
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

/// One level of access of a [`CellPath`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PathMember {
    /// Accessing a member by string (i.e. columns of a table or [`Record`](crate::Record))
    String {
        val: String,
        span: Span,
        /// If marked as optional don't throw an error if not found but perform default handling
        /// (e.g. return `Value::Nothing`)
        optional: bool,
    },
    /// Accessing a member by index (i.e. row of a table or item in a list)
    Int {
        val: usize,
        span: Span,
        /// If marked as optional don't throw an error if not found but perform default handling
        /// (e.g. return `Value::Nothing`)
        optional: bool,
    },
}
```
