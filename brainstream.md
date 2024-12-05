
##### 12/05/24: This is our test parse for parsing Nuon tables

```rust
[[name, age]; [rick 64] [hb 74]]
```

- see parser.rs: parse_full_cell_path
- see parser.rs: parse_table_expression

This code is executed starting at around line 4886

```rust
SyntaxShape::Any => {
    if bytes.starts_with(b"[") {
        //parse_value(working_set, span, &SyntaxShape::Table)
        parse_full_cell_path(working_set, None, span)
```

This code is executed starting at around line 2315

```rust
} else if bytes.starts_with(b"[") {
    trace!("parsing: table head of full cell path");
    let output = parse_table_expression(working_set, head.span, &SyntaxShape::Any);
    tokens.next();
    (output, true)
```
