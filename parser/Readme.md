
All of the notes here are about the old parser.

For notes on the new parser go [here](./new-nu-parser)

Start to write up the flow of the parser...

* parse
* parse_block
* parse_pipeline   
      - parse_builtin_commands [parse_call]


#### Notes on parse_pipeline()

It starts out with just one if/else conditional which encompasses the whole method.

```rust
if pipeline.commands.len() > 1 {
  ```
