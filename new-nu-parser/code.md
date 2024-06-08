
#### src/compiler.rs

```rust
pub fn push_node(&mut self, ast_node: AstNode) -> NodeId {
        println!("{:?}", ast_node);
        self.ast_nodes.push(ast_node);

        println!("{:?}", self.ast_nodes.len() - 1);
        NodeId(self.ast_nodes.len() - 1)
}
```

#### src/test.rs

```rust
#[test]
fn test_node_output() {
    insta::glob!("../tests", "*.nu", |path| {
        println!("{:?}", path);
        insta::assert_snapshot!(evaluate_example(path));
    });
}
```
