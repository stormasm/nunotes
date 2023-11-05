
* put this at the end of dataframe/eager/open.rs

* this returns a hardcoded dataframe in place of running the code...

```rust
use polars::datatypes::UInt32Chunked;
use polars::datatypes::Float64Chunked;
use polars::prelude::{DataFrame, IntoSeries, NamedFrom};

let a = UInt32Chunked::new("a", &[1, 2, 3]).into_series();
let b = Float64Chunked::new("b", &[10., 8., 6.]).into_series();
let ab = DataFrame::new(vec![a, b]).unwrap();

Ok(ab)
```
