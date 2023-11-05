
### Cool Scripts

```rust
ls nu-* | each { |file| open ($file.name + "/Cargo.toml")  | get dependencies? | { name: $file.name, dependencies: $in }}
```

```rust
ls | where <condition goes here> | each { |file| mv $file.name new_dir }
```

```rust
seq 1 10 | into string | str lpad --character 0 --length 3
```
[discord](https://discord.com/channels/601130461678272522/614593951969574961/996048885095071844)

You first have to register the query plugin

```rust
fetch https://en.wikipedia.org/wiki/Wikipedia:Database_reports/Unbelievable_life_spans | query web -t [No. Page "Birth year" "Death year" "Life span"]
```

Then you can run the above command.
