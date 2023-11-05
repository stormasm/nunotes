
```rust
def ols3 [...rest] {
  for i in $rest --numbered {
    print $'arg_index=($i.index) argument=($rest | get ($i.index))'
  }
}
```

[discord core](https://discord.com/channels/601130461678272522/683070703716925568/954722216120057867)
