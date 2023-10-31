
To get up and running make sure you have *nupm* installed and available in nushell.

Grab the nu-git-manager repo and cd into it

```rust
nupm install --path --force .
use nu-git-manager *
gm list
gm update-cache
gm list
gm status
```

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
