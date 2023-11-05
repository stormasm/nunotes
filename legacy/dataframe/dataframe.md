
### dfr open command

list syntax must be like this for it to work

please add an example to the command so people [and myself]
can see what it is supposed to look like for future reference.

```rust
cd polars/examples/datasets
[a,b,c] | describe
dfr open foods1.csv --columns [category,calories]
dfr open foods1.parquet --columns [calories]
dfr open foods1.parquet --columns [calories,fats_g]
```

### Nushell tables can have multiple types in any given cell

it seemed like dfr tables are columnar based while nushell tables are row based for streaming. also, nushell tables can have multiple datatypes in any given cell whereas dfrs needs to have each column be a singular datatype.

[discord](https://discord.com/channels/601130461678272522/864228801851949077/966365153996210186)

```rust
let df = ([[a,b],[1,apple] [2,banana] [3,apple]]) | dfl to-lazy)
$df | dfl filter (expr col a | expr gt (expr lit 2)) | dfl collect
```

[discordcore](https://discord.com/channels/601130461678272522/683070703716925568/962740709272727552)

### What polars crates do we use ?

* Compiling parquet2 v0.10.3
* Compiling arrow2 v0.10.1
* Compiling polars-arrow v0.20.0
* Compiling polars-core v0.20.0
* Compiling polars-io v0.20.0
* Compiling polars-time v0.20.0
* Compiling polars-lazy v0.20.0
* Compiling polars v0.20.0

#### References

[Group By Tutorial for Pandas](https://realpython.com/pandas-groupby/)
