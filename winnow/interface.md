
rust ```
rg crate::winnow
```

This shows the 4 spots where the winnow parser gets called
in the legacy parser.

Once the legacy parser is removed these calls will still exist
but nothing else...

```rust
parse_pipelines.rs
121:    if crate::winnow::enabled() {
126:        crate::winnow::record_classic_pipeline(
195:    if let Some(block) = crate::winnow::nested_block(

parse_module.rs
509:    if crate::winnow::enabled() {
510:        return crate::winnow::parse_module_block(working_set, span, module_name);

parse_calls.rs
2071:    if crate::winnow::enabled()
2073:            crate::winnow::parse_shorter_head_reading(working_set, head, input_type)

parse_captures_compile.rs
858:    let mut block = if crate::winnow::enabled() {
859:        crate::winnow::parse_file_block(working_set, contents, new_span, scoped)
```
