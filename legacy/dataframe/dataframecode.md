
For more details on the Polars code tied to dataframes go
[here](./polars.md).

#### Methods used by all dataframe commands on NuDataFrame exist in mod.rs

See all of the [pub fn] methods in *dataframe/values/nu_dataframe/mod.rs*

```rust
pub fn try_from_series(columns: Vec<Series>, span: Span) -> Result<Self, ShellError> {
```

All of these methods return the *Result<Self, ShellError>*

* try_from_columns
* try_from_iter
* try_from_pipeline
* try_from_series
* try_from_value
* column

These return a Value

* dataframe_into_value
* into_value

### NuDataFrame Code Details

*dataframe/values/nu_dataframe/conversion.rs*

```rust
pub fn create_column(
    series: &Series,
    from_row: usize,
    to_row: usize,
    span: Span,
) -> Result<Column, ShellError> {

// Adds a separator to the vector of values using the column names from the
// dataframe to create the Values Row

pub fn add_separator(values: &mut Vec<Value>, df: &DataFrame, span: Span) {

// Inserting the values found in a Value::List
pub fn insert_record(
    column_values: &mut ColumnMap,
    cols: &[String],
    values: &[Value],
) -> Result<(), ShellError> {

pub fn insert_value(
    value: Value,
    key: String,
    column_values: &mut ColumnMap,
) -> Result<(), ShellError> {

// The ColumnMap has the parsed data from the StreamInput
// This data can be used to create a Series object that can initialize
// the dataframe based on the type of data that is found

pub fn from_parsed_columns(column_values: ColumnMap) -> Result<NuDataFrame, ShellError> {
```

* dataframe/values/nu_dataframe/custom_value.rs

these methods are referenced here.

* dataframe/values/nu_dataframe/operations.rs

* compute_with_value is referenced only in **custom_value.rs**

* append_df is referenced in **between_values.rs** and **eager/append.rs**

```rust
impl NuDataFrame {

    pub fn compute_with_value(
        &self,
        lhs_span: Span,
        operator: Operator,
        op_span: Span,
        right: &Value,
    ) -> Result<Value, ShellError> {

      pub fn append_df(
          &self,
          other: &NuDataFrame,
          axis: Axis,
          span: Span,
      ) -> Result<Self, ShellError> {

```

* dataframe/values/nu_dataframe/between_values.rs

has 3 public methods all of which are ONLY referenced in **operations.rs**

* between_dataframes
* compute_between_series
* compute_series_single_value

#### more code details on dataframe/values/nu_dataframe/mod.rs

```rust
pub fn as_series(&self, span: Span) -> Result<Series, ShellError> {
    let series = self
        .0                 // get the dataframe
        .get_columns()     // get the series
        .get(0)            // get the first series

    Ok(series.clone())
}
```
### ChunkedArray

The only place in the nushell code where
[ChunkedArray](https://docs.rs/polars/latest/polars/chunked_array/struct.ChunkedArray.html)
is referenced is

* eager/describe.rs
* values/nu_dataframe {between_values.rs, conversion.rs}

### Dataframe panics

```rust
[[a b c d]; [pete bill paul mike] [100 200 300 400]] | dfr to-df | dfr to-dummies
```

```rust
[[a b]; [c d] [1 2]] | dfr to-df | dfr to-dummies
```

### Dataframe code cleanup

In the dataframe directory fix the error messages for these series commands.

```rust
rg str-slice
```
