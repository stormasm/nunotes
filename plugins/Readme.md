

[Go here for more details on how to build and run a plugin](https://github.com/stormasm/nunotes/blob/main/legacy23/oneliners.md#nu_plugin_query)

```rust
$env.config.color_config.leading_trailing_space_bg = {bg: sandybrown}
```

```rust
cargo search nu_plugin --limit 10 | lines | parse "{crate_name} = {version} #{description}"
```
