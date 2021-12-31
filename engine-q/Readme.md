
##### How to update config

```rust
let $config = ($config | update color_config $base16_theme)
```

* [discord](https://discord.com/channels/601130461678272522/889232844101156914/926245471066927195)

JT description of the flow in the pipeline...

the flow right now is that commands are composable. As you connect commands together with pipes, what flows in the pipes are different kinds of values, all coming from the Value enum. Along the way in the pipeline, other commands might make changes to these values (filtering, updating, etc), and then finally at the end of the pipeline you'll optionally be able to print or save this output
