
### check to see if your plugin is installed

```rust
help commands | where command_type == "plugin"
```

### nu_plugin_query_git

```rust
query git "select * from commits limit 20" | into value | reject message email repo | update commit_id {str substring 0..6}
```

[discord](https://discord.com/channels/601130461678272522/683070703716925568/1248842093829423136)

### nu_plugin_inc

```rust
"1.1.1" | inc -M
"2.1.3" | inc -p
456 | inc
"345" | inc
```

### Random notes

- [Overhaul the plugin cache file with a new msgpack+brotli format](https://github.com/nushell/nushell/pull/12579)
- register becomes plugin add and there's a plugin rm now? amazing
- [discord ref](https://discord.com/channels/601130461678272522/683070703716925568/1231603041896108174)
- [windsoilder plugin diagram](https://drive.google.com/file/d/19g7GktaRYlz_pKKbnnvV0Zjblz3x6Xvg/view)
- [discord ref](https://discord.com/channels/601130461678272522/683070703716925568/1226769385281290281)

### Plugins before devyn

- [legacy old plugins *** details on how to build and run a plugin](https://github.com/stormasm/nunotes/blob/main/legacy23/oneliners.md#nu_plugin_query)

```rust
$env.config.color_config.leading_trailing_space_bg = {bg: sandybrown}
```

```rust
cargo search nu_plugin --limit 10 | lines | parse "{crate_name} = {version} #{description}"
```
