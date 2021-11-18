
engine-q notes

### Table Syntax

[discord](https://discord.com/channels/601130461678272522/683070703716925568/911013704378765322)

These 2 are equivalent

```rust
[[a,b]; [1,2]]
[{ "a": 1, "b": 2}]
```

These 2 are slightly different

```rust
{"a" : 1, "b" : 2}
{a:1, b:2}
```

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
