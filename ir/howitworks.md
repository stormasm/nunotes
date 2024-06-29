
Devyn adds to the end of the [parse_block function](https://github.com/devyn/nushell/blob/ir/crates/nu-parser/src/parser.rs#L5793) in the parser this code...

```rust
// Do not try to compile blocks that are subexpressions, or when we've already had a parse
 // failure as that definitely will fail to compile
 if !is_subexpression && working_set.parse_errors.is_empty() {
     match nu_engine::compile(working_set, &block) {
         Ok(ir_block) => {
             block.ir_block = Some(ir_block);
         }
         Err(err) => working_set
             .parse_warnings
             .push(ParseWarning::IrCompileError {
                 span,
                 errors: vec![err],
             }),
     }
 }
```
