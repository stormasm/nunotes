
This is about the nushell side of dataframes...

For polars notes go [here](./polars.md)

### between_values.rs

has 3 public methods all of which are ONLY referenced in **operations.rs**

* between_dataframes
* compute_between_series
* compute_series_single_value

### ChunkedArray

The only place in the nushell code where
[ChunkedArray](https://docs.rs/polars/latest/polars/chunked_array/struct.ChunkedArray.html)
is referenced is

* eager/describe.rs
* values/nu_dataframe {between_values.rs, conversion.rs}
