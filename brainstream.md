
##### 12/13/2024 Keep going with IrBlock

##### 12/10/2024 working on IR

```rust
eval_pipeline_without_terminal_expression
```

##### 12/07/2024 looking at the ast produced from a nuon table

- FullCellPath
- CellPath
- PathMember

#### Conclusion: *evaluate_source* is the final stop for the whole concept

##### 12/06/24: How does a new parser get kicked off ?

For more info run the following command in `nunotes`

```rust
rg eval_source
```

- nu_cli/src/repl.rs
- fn do_run_cmd

```rust
rg "parse\("
nu_parser::parse
```

How do you kick off a new StateWorkingSet ?

```rust
StateWorkingSet::new(&engine_state);
```

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
