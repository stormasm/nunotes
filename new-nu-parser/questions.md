
### Question 01

What is a tuple struct in the context of NodeId.

Why do we use a Tuple Struct

Where does the NodeId gets its number

### Answer 01

```rust
pub struct Compiler {
  ast_nodes: Vec<AstNode>
}
```

The NodeId is simply a number which is an index into the Vector of AstNode's

```rust
pub fn push_node(&mut self, ast_node: AstNode) -> NodeId {
     self.ast_nodes.push(ast_node);

     NodeId(self.ast_nodes.len() - 1)
 }
```
