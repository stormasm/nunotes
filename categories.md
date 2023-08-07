
### There are currently twenty three (23) nushell categories

* conversions
* core
* database
* date
* debug

* default
* deprecated
* env
* experimental
* filesystem

* filters
* formats
* generators
* hash
* math

* misc
* network
* path
* platform
* random

* strings
* system
* viewers

### Reference

```rust
help commands | sort-by category
```

* [pub enum Category](https://github.com/nushell/nushell/blob/main/crates/nu-protocol/src/signature.rs)

```rust
nu --no-config-file --no-std-lib --commands "help commands | where category in [experimental, misc, default] | get name"

nu --no-config-files --commands "help commands | where category in [experimental, misc, default] and command_type != builtin | get name"
```

maybe should add and `command_type == builtin` to that one-liner, since some of those are scripts which cannot have categories.

[discord note on this subject](https://discord.com/channels/601130461678272522/683070703716925568/1138044215536926721)
