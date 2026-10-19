


### How to suppress linker messages

```rust
RUSTFLAGS="-Alinker-messages" cargo run   
```

This goes in the file .cargo/config.toml

```rust
[target.aarch64-apple-darwin]
# We can guarantee that this target will always run on a CPU with _at least_
# these capabilities, so let's optimize for them
rustflags = ["-Ctarget-cpu=apple-m1", "-Alinker-messages"]
```

- [located here -> config.toml](https://github.com/stormasm/nunotes/blob/main/config.toml)

### Miles amazing description of how completions work

- [Unified Completions Descriptions](https://github.com/nushell/nushell/pull/18791)


### Corporate Overview Incredibuild

- [ref incredibuild](https://docs.google.com/document/d/e/2PACX-1vT4GeXvborUk-cp2_PFguSNtqgL-F8b46ADpeqKGhAMmUDC8TCcECEEdKXYQbQeV96AZrOUAxQr1wBm/pub)

### Debug Adapter Protocol

- https://zed.dev/docs/debugger
- now available in nushell 2026 summer
- you can step through nushell scripts

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
