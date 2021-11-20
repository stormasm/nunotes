
# engine-q notes

[table syntax](./engine-q/table-syntax.md)

### Send back no data

```rust
Ok(PipelineData::Value(Value::Nothing { span: call.head }))
```

##### How do you run CI checks locally ?

```
alias ciman='cargo fmt; cargo check; cargo clippy; cargo test --all'

cargo fmt
cargo check
cargo clippy
cargo test --all
```

[CI workflow steps manually](https://github.com/nushell/engine-q/blob/main/.github/workflows/ci.yml); &nbsp;&nbsp;
[discord link](https://discord.com/channels/601130461678272522/889232844101156914/904688334578794516)

##### trait CustomValue

[discord](https://discord.com/channels/601130461678272522/889232844101156914/911337922890985512)
