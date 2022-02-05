
[discord](https://discord.com/channels/601130461678272522/889232844101156914/939357502414409758)

```rust
$nu.cwd
```

[discord](https://discord.com/channels/601130461678272522/889232844101156914/938711460505325649)

is it possible to skip some kinds of comments line when open a csv file ?   
i got some bank's transaction file which include some lines begin with #

I'd do something like

```rust
open foo.csv --raw | lines | where ($it | str starts-with '#') == $false | from csv
```
