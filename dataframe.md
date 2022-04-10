
```rust
let df = ([[a,b],[1,apple] [2,banana] [3,apple]]) | dfl to-lazy)
$df | dfl filter (expr col a | expr gt (expr lit 2)) | dfl collect
```

[discordcore](https://discord.com/channels/601130461678272522/683070703716925568/962740709272727552)

We use these 3 polars crates:

* Compiling polars-arrow v0.19.1
* Compiling polars-time v0.1.1
* Compiling polars-core v0.19.1
* Compiling polars-io v0.19.1
* Compiling polars-lazy v0.19.1
* Compiling polars v0.19.1

#### References

[Group By Tutorial for Pandas](https://realpython.com/pandas-groupby/)
