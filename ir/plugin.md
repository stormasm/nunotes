
### check to see if your plugin is installed

```rust
help commands | where command_type == "plugin"
```

- [devyn plugin](https://github.com/devyn/nu_plugin_explore_ir)

```rust
view ir {
  if ($env.HELLO | is-not-empty) {
    "Hello, " ++ $env.HELLO ++ "!"
  } else {
    "Goodbye, " ++ (random uuid) ++ "!"
  }
}
```

```rust
view ir --json {
  if ($env.HELLO | is-not-empty) {
    "Hello, " ++ $env.HELLO ++ "!"
  } else {
    "Goodbye, " ++ (random uuid) ++ "!"
  }
} | explore ir
```

```rust
view ir --json {
  if ($env.HELLO | is-not-empty) {
    "Hello, " ++ $env.HELLO ++ "!"
  } else {
    "Goodbye, " ++ (random uuid) ++ "!"
  }
} | explore ir
```

### References

- [nushell book on plugins](https://www.nushell.sh/book/plugins.html)
