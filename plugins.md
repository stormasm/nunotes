
### Plugins

```rust
register -e json ./nu_plugin_query
register --encoding json ./nu_plugin_query
```

**REMEMBER** you must be in the binary directory usually located here

```rust
./nushell/target/debug/nu_plugin_query
```

and you must **BUILD** your plugin first !

If you try and register a plugin that is located in the src directory or github repo directory...

```rust
./nushell/crates/nu_plugin_query
```

It will not work !


### Legacy Notes

To get plugins up and running

```rust
cargo build --all-features

register -e capnp /some/path/nu_plugin_inc
register -e capnp /some/path/nu_plugin_gstat
register -e capnp /some/path/nu_plugin_example
register -e json -s /path/to/python3 /path/to/nu_plugin_python/plugin.py
```

If you are using python2 instead of python3 here is your [solution PR 524](https://github.com/nushell/engine-q/pull/524)

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
help nu-python
```

To run the plugins simply type

```rust
gstat
nu-example-1 2 hi
nu-example-2 3 bye
nu-example-3 4 why
1 | inc
echo "this is input" | nu-python 1 abc 2 -f -n dog now is the time
```

nu-example-3 is supposed to throw an error

### PRs

* [Plugin signature #520](https://github.com/nushell/engine-q/pull/520)
* [Plugin json #475](https://github.com/nushell/engine-q/pull/475)
