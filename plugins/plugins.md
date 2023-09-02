
### Creating the binary file of your plugin

```rust
cargo install --path crates/nu_plugin_from_parquet
register ~/.cargo/bin/nu_plugin_from_parquet  
```

The register command must point to a binary file...

And there are several ways to create the binary file...

You can issue the *cargo install* command above which will put the binary
file inside your default *~/.cargo/bin* directory or

```rust
cargo install --path crates/nu_plugin_from_parquet --target-dir /tmp/crates
```

Which will put the binary file inside the *target-dir/release* directory

When you are inside the plugin directory you can issue the command

```rust
cargo build
```

This will put the binary file in the *target/debug* directory of the plugin's parent repository.

### Plugins Legacy Notes

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

##### To show yourself that the commands were registered   

```rust
help commands | where is_plugin == true
```

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
help commands | where is_plugin == true
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

```rust
nu-example-1 -f -n 555 78 why 666 newmexico -n "ralph"
nu-example-2 -f -n 555 78 why 666 arkansas -n "ralph"
nu-example-3 -f -n 555 78 why 666 delaware -n "ralph"
```

### PRs

* [Plugin signature #520](https://github.com/nushell/engine-q/pull/520)
* [Plugin json #475](https://github.com/nushell/engine-q/pull/475)
