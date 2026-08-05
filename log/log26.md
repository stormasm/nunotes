
### Debug Adapter Protocol

- https://zed.dev/docs/debugger
- now available in nushell 2026 summer

### Reserved Special Variables / Words in Nushell

In keeping with the flavor of our reserved special variables / words
- $in
- $nu
- $env
- $it

We are going to add
- $ans

---

- [How the Repl gets called](https://github.com/stormasm/nunotes/blob/main/repl.md)

---

```rust
help commands | where {|row| $row.command_type == keyword}
help commands | where command_type == "keyword"
help commands | where command_type == keyword
help commands | where command_type == custom
help commands | where category == filters
```

- [this is when we first introduced ratatui](https://github.com/nushell/nushell/pull/8952)
