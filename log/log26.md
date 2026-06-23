

```rust
help commands | where {|row| $row.command_type == keyword}
help commands | where command_type == "keyword"
help commands | where command_type == keyword
help commands | where command_type == custom
help commands | where category == filters
```

- [this is when we first introduced ratatui](https://github.com/nushell/nushell/pull/8952)
