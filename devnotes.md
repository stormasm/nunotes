
### how does the data get into PipelineData

* see the repl loop in nu-cli
* [nu_cli::eval_source](https://github.com/nushell/nushell/blob/main/crates/nu-cli/src/repl.rs)
* [nu_engine::eval_block](https://github.com/nushell/nushell/blob/main/crates/nu-engine/src/eval.rs)

### A good example of seeing this is in the command

[from nuon](https://github.com/nushell/nushell/blob/main/crates/nu-command/src/formats/from/nuon.rs)

```rust
let (mut block, err) = nu_parser::parse_block(&mut working_set, &lite_block, true, &[]);
```

* the parser puts the data in a block

```rust
let result = convert_to_value(expr, head, &string_input);
```

* then the expression emerges from the pipeline
* and convert_to_value gets called...

### References for further research

[nu-protocol data structures](./engine-q/engine-q-nu-protocol.md)
