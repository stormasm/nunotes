
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

- [kubouch notes on parsing et al](https://discord.com/channels/601130461678272522/683070703716925568/1316492246505361508)

```rust
[[foo bar]; [1 2][2 3]]
```

- is parsed as math expression because it starts with [.
- Math expression means "expression that is not a call". For example, foo is parsed as a call, but in ls foo, the foo is a math expression (because it's not in a call position) and is eventually parsed as a string.
- [entity component systems](https://en.wikipedia.org/wiki/Entity_component_system)
