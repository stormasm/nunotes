
### How the legacy to_sqlite plugin for Nushell worked in 0.44

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

### How the legacy from_sqlite plugin for Nushell worked in 0.44

Right off the bat you have to know the filename of the database along
with the tables in that database you want to convert back to nu values.

Then you send in an sql select statement to grab those tables.

```rust
let mut meta_stmt = conn.prepare("select name from sqlite_master where type='table'")?;
let mut meta_rows = meta_stmt.query([])?;
```

For each table, for each row in the database you convert the row to a nu value

```rust
while let Some(table_row) = table_rows.next()? {
    out.push(convert_sqlite_row_to_nu_value(table_row, tag.clone()))
}
```

Each one of these values
[Rusqlite Value](https://docs.rs/rusqlite/latest/rusqlite/types/enum.Value.html)
coming out of Rusqlite has to get converted into a Nushell value...

### References

* [read_to_end](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_to_end)

### fdncred rsq implementation

[first mention](https://discord.com/channels/601130461678272522/855886335980994600/933116747630923776)

[fdncred sqlite](https://github.com/fdncred/rsq)
