
### How the old sqlite plugin for Nushell worked in 0.44

The to_sqlite plugin worked by opening a connection into a temporary file
and then streaming all of those bytes into a binary out buffer which then
would get passed along in the pipeline.  

```rust
let mut out = Vec::new();
tempfile.read_to_end(&mut out)?;
Ok(UntaggedValue::binary(out).into_value(tag))
```

From there I guess you can call save and save those bytes to a file ?

generate_statements would do this :

```rust
let create = format!("create table {}({})", table_name, columns?);
let insert = format!("insert into {} values {}", table_name, insert_values?);
```

* [read_to_end](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_to_end)

[first mention](https://discord.com/channels/601130461678272522/855886335980994600/933116747630923776)

[fdncred sqlite](https://github.com/fdncred/rsq)
