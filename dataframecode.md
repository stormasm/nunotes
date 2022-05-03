
This is about the nushell side of dataframes...

For polars notes go [here](./polars.md)

### ChunkedArray

The only place in the nushell code where
[ChunkedArray](https://docs.rs/polars/latest/polars/chunked_array/struct.ChunkedArray.html)
is referenced is

* eager/describe.rs
* values/nu_dataframe {between_values.rs, conversion.rs}
