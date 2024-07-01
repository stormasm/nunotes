
#### nu-cli/src/repl.rs

comment out these lines of code [lines 149 - 160]

```rust
if load_std_lib.is_none() && engine_state.get_config().show_banner {
    eval_source(
        engine_state,
        &mut unique_stack,
        r#"use std banner; banner"#.as_bytes(),
        "show_banner",
        PipelineData::empty(),
        false,
    );
}
```

#### nu_engine/src/eval_ir.rs

```rust
-        log::trace!(
+        println!(
```
