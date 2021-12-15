
To get plugins up and running

```rust
cargo build --all-features

register -e capnp /j/tmp17/engine-q/target/debug/nu_plugin_inc
register -e capnp /j/tmp17/engine-q/target/debug/nu_plugin_gstat
register -e capnp /j/tmp17/engine-q/target/debug/nu_plugin_example
```

To show yourself that the commands were registered   
and is_plugin will be true

```rust
help commands
```

To get help on the plugins

```rust
help gstat
help inc
help nu-example-0
help nu-example-1
help nu-example-2
```

To run the plugins simply type

```rust
gstat
nu-example-1 2 hi
nu-example-2 3 bye
nu-example-3 4 why
1 | inc
```

### Discord Notes

[note1](https://discord.com/channels/601130461678272522/683070703716925568/919953220011425833)

### PRs

[Plugin json #475](https://github.com/nushell/engine-q/pull/475)
