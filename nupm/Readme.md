
### One of the mantras of nupm

* module files and their paths must be available before your script is run as parsing occurs before anything is evaluated

This command comes in handy to get to the *default-config-dir*

```rust
cd $nu.default-config-dir   
```

```rust
nuconfig
nurun -n
~/Library/Application Support/nushell> source env.nu
~/Library/Application Support/nushell> source nupm.nu
~/Library/Application Support/nushell> delete-nupm
~/Library/Application Support/nushell> install-nupm
Root directory "/Users/ma/.nupm" does not exist. Do you want to create it? [y/n] y
2023-11-14T07:17:26.880|INF|installing package nupm
~/Library/Application Support/nushell> install-nu-git-manager
2023-11-14T07:17:44.963|INF|installing package nu-git-manager
~/Library/Application Support/nushell> exit
```

This repo contains nupm packages.

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
