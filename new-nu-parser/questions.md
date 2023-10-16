
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

This code lives inside the Compiler.

```rust
pub fn push_node(&mut self, ast_node: AstNode) -> NodeId {
     self.ast_nodes.push(ast_node);
     NodeId(self.ast_nodes.len() - 1)
 }
```

There is only one reference to the *push_node* method in the parser and that is inside the method *create_node*

```rust
pub fn create_node(&mut self, ast_node: AstNode, span_start: usize, span_end: usize) -> NodeId {
    self.compiler.span_start.push(span_start);
    self.compiler.span_end.push(span_end);
    self.compiler.push_node(ast_node)
}
```
