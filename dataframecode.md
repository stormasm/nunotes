
### Methods used by all dataframe commands on NuDataFrame

See all of the methods in *dataframe/values/nu_dataframe/mod.rs*

* try_from_columns
* try_from_iter
* try_from_pipeline
* try_from_series
* try_from_value

* dataframe_into_value
* into_value

For more details on the Polars code tied to dataframes go
[here](./polars.md).

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











### /values/nudataframe/custom_value.rs

these methods are referenced here.

### /values/nudataframe/operations.rs

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

### /values/nu_dataframe/between_values.rs

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
