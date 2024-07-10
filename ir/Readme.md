
### Startup

Set environment variable to: *NU_USE_IR*
[see discord](https://discord.com/channels/601130461678272522/683070703716925568/1260135306628304907)

In the code its checked in `src/run.rs`

```rust
rg use_ir
```

for further details...

```rust
devir
nuir
nurunn
```

### Eval

nu-engine/src/eval_ir.rs

```rust
nurunn --log-level trace --log-exclude '[nu_parser, nu_cli, nu_utils, nu::config_files]'
```

```rust
do --use-ir {1 + 2}
do --use-ir {print -e ((ansi gb) ++ ("hello world !" | str upcase) ++ (ansi reset))}
```

### View Ir

```rust
view ir { 1 bit-shl 2 }
view ir { 1 + 2 }
```

```rust
view ir { "foo bar" | str replace "foo" "baz" }
```
