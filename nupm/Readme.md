
### How to bootstrap the nupm system

* nurun
* nupmg
* use nupm
* nuconfig
* source nupm.nu
* gm [to test and see its been installed]

### One of the mantras of nupm

* module files and their paths must be available before your script is run as parsing occurs before anything is evaluated

this means that you can NOT have

```rust
use nu-git-manager *
```

for example in a custom command or an *alias*

---

This command comes in handy to get to the *default-config-dir*

```rust
cd $nu.default-config-dir
```

or you can simply type *nuconfig*

### Other custom commands

* delete-nupm
* install-nupm
* install-ngm

### nu-git-manager

To run the tests simply type

```rust
nupm test
```

nu-git-manager/tests/mod.nu

```rust
-    use ~/.local/share/nupm/modules/nupm
+    use ~/.nupm/modules/nupm
```

```rust
    use ~/.nupm/modules/nupm
```

### Example packages

* https://github.com/amtoine/tmux-sessionizer
* https://github.com/amtoine/nu-git-manager
* https://github.com/amtoine/nu_plugin_explore
* https://github.com/amtoine/scripts
* https://github.com/nushell/nu_scripts/blob/main/package.nuon
* [discord](https://discord.com/channels/601130461678272522/1112829613920505956/1172882159959281745)

### References

* [antoine dotfiles](https://github.com/amtoine/dotfiles)
