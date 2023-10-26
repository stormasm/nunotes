

yeah, you have to
```rust
use nu-git-manager *
```
in config.nu 👍

* repos are stored in a global location

* [antoine note on location of glob functionality](https://github.com/amtoine/nu-git-manager/blob/main/nu-git-manager/fs/store.nu#L39)

```rust
glob  **/*HEAD
```
