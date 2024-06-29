
By default Devyn parses each block and compiles it into the IR...

Then if you pass in the *--use-ir* flag it gets evaluated as well...

### nu-engine/src/eval.rs

```rust
pub fn eval_block<D: DebugContext>(
    engine_state: &EngineState,
    stack: &mut Stack,
    block: &Block,
    mut input: PipelineData,
) -> Result<PipelineData, ShellError> {
    // Remove once IR is the default.
    if stack.use_ir {
        return eval_ir_block::<D>(engine_state, stack, block, input);
    }
```

### nu-parser/src/parser.rs

See [parser.rs](https://github.com/devyn/nushell/blob/ir/crates/nu-parser/src/parser.rs) parse_block

```rust
pub fn parse_block(
    working_set: &mut StateWorkingSet,
    tokens: &[Token],
    span: Span,
    scoped: bool,
    is_subexpression: bool,
) -> Block {
```

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
