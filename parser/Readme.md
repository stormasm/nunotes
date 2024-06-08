
All of the notes here are about the old parser.

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
