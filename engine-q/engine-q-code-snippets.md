
### Latest matching in the append command

```rust
    match input {
        PipelineData::Value(Value::List { vals, .. }) => {
            dbg!("Got a Value");
        }
        PipelineData::Stream(stream) => {
            dbg!("Got a Stream");
        }
        _ => {
            dbg!("Fall to the bottom");
        }
    }
```


### Example of how to use from_value

```rust
let span = Span::unknown();
let val = Value::Int { val: 2, span };
let cell_path = CellPath::from_value(&val).unwrap();
```

from **nu-engine/src/from_value.rs**
```rust
impl FromValue for CellPath {
```

from an older implementation of the last command
```rust
match input {
         PipelineData::Stream(stream) => {
             dbg!("Stream");
             Ok(stream
                 .skip(beginning_rows_to_skip.try_into().unwrap())
                 .into_pipeline_data())
         }
         PipelineData::Value(Value::List { vals, .. }) => {
             dbg!("Value");
             Ok(vals
                 .into_iter()
                 .skip(beginning_rows_to_skip.try_into().unwrap())
                 .into_pipeline_data())
         }
         _ => {
             dbg!("Fall to the bottom");
             Ok(PipelineData::Value(Value::Nothing { span: call.head }))
         }
     }

     // Ok(PipelineData::Value(Value::Nothing { span: call.head }))
```

### Clippy Notes

rg needless

```rust
#[allow(clippy::needless_collect)]
```
