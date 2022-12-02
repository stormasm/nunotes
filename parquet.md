
[discord parquet initial hit](https://discord.com/channels/601130461678272522/864228801851949077/1046404011928080446)

```rust
register ~/.cargo/bin/nu_plugin_from_parquet
open -r sample.parquet | from parquet | first 10
```

if you have dataframes installed, open-df should work on parquet files

