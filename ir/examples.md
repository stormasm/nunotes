
### Eval

```rust
nurunn --log-level trace --log-exclude '[nu_parser, nu_cli, nu_utils, nu::config_files]'
```

```rust
do --use-ir {1 + 2}
```

### View Ir

```rust
view ir { 1 bit-shl 2 }
view ir { 1 + 2 }
```

```rust
view ir { "foo bar" | str replace "foo" "baz" }
```
