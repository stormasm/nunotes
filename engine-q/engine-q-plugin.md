
To get plugins up and running

```rust
cargo build --all-features

register -e capnp /some/path/nu_plugin_inc
register -e capnp /some/path/nu_plugin_gstat
register -e capnp /some/path/nu_plugin_example
```

To show yourself that the commands were registered   

```rust
help commands | where is_plugin == $true
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

nu-example-3 is supposed to throw an error

### PRs

[Plugin json #475](https://github.com/nushell/engine-q/pull/475)
