
### Question 01

Where does the NodeId gets its number

What is a tuple struct in the context of NodeId.

Why do we use a Tuple Struct

### Answer 01

```rust
pub fn push_node(&mut self, ast_node: AstNode) -> NodeId {
     self.ast_nodes.push(ast_node);

     NodeId(self.ast_nodes.len() - 1)
 }
```
