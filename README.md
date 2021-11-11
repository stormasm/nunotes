# nunotes


##### nushell table example

```rust
[[a b]; [1 a] [2 b]] | append [[a b]; [3 c]]
```

##### nu json syntax

[discord](https://discord.com/channels/601130461678272522/683070703716925568/908095523133751356)

##### engine-q table example
```rust
[[a b]; [1 a] [2 b]]

\\ both cases work the same
[[a, b]; [1, a] [2, b]]
```

ls | pivot | str capitalize Column0

ps | sort-by pid | first 10

##### the way to show value::record
$t1x | get 0 | grid

##### How do you run CI checks locally ?

```
cargo fmt
cargo check
cargo clippy
cargo test --all
```

[CI workflow steps manually](https://github.com/nushell/engine-q/blob/main/.github/workflows/ci.yml); &nbsp;&nbsp;
[discord link](https://discord.com/channels/601130461678272522/889232844101156914/904688334578794516)
