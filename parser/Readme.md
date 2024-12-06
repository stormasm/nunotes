
#### evaluate_source is the final stop for the whole concept

To see critical stuff run the following command in `nunotes`

```rust
rg eval_source
```

- nu_cli/src/repl.rs
- fn do_run_cmd

All of the notes here are about the old parser.

- [fn do_run_cmd](https://github.com/stormasm/nunotes/blob/main/brainstream.md#120624-how-does-a-new-parser-get-kicked-off-)

For notes on the new parser go [here](./../new-nu-parser)

Start to write up the flow of the parser...

- parse
- parse_block
- parse_pipeline   
- parse_pipeline_element
- parse_builtin_commands [parse_call]  
- parse_expression

#### Notes on parse_pipeline()

It starts out with just one if/else conditional which encompasses the whole method.

```rust
if pipeline.commands.len() > 1 {
  ```

### References

- [entity component systems](https://en.wikipedia.org/wiki/Entity_component_system)
