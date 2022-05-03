
### polars top level

```rust
DataFrame is a Vec<Series>
Series
ChunkedArray Vec<dyn ArrowArray>
```

The
[Series struct](https://docs.rs/polars/latest/polars/series/struct.Series.html)
consists of typed ChunkedArray’s. To quickly cast a Series to a ChunkedArray you can call the method with the name of the type:
