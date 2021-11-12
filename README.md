# nunotes

##### append-simple

```rust
use nu_engine::CallExt;
use nu_protocol::ast::Call;
use nu_protocol::engine::{Command, EngineState, Stack};
use nu_protocol::{
    IntoInterruptiblePipelineData, PipelineData, ShellError, Signature, SyntaxShape, Value,
};

#[derive(Clone)]
pub struct Append;

impl Command for Append {
    fn name(&self) -> &str {
        "append"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("append").required("row", SyntaxShape::Any, "the row to append")
    }

    fn usage(&self) -> &str {
        "Append a row to the table."
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let val: Value = call.req(engine_state, stack, 0)?;
        let iter = input.into_iter().chain([val]);
        Ok(iter.into_pipeline_data(engine_state.ctrlc.clone()))
    }
}
```

##### more matching of values

```rust
// will remove this once append lands...
fn signature(&self) -> nu_protocol::Signature {
    Signature::build("append").required("row", SyntaxShape::Any, "the row to append")
}

fn run(
    &self,
    engine_state: &EngineState,
    stack: &mut Stack,
    call: &Call,
    input: PipelineData,
) -> Result<PipelineData, ShellError> {
    let val: Value = call.req(engine_state, stack, 0)?;

    match val {
        Value::List { vals, .. } => {
            dbg!("Got a List val");
        }
        _ => {
            dbg!("Fall to the bottom on val");
        }
    }

    match input {
        PipelineData::Value(Value::List { vals, .. }) => {
            dbg!("Got a Value input");
        }
        PipelineData::Stream(stream) => {
            dbg!("Got a Stream input");
        }
        _ => {
            dbg!("Fall to the bottom on input");
        }
    }

    Ok(PipelineData::Value(Value::Nothing { span: call.head }))
}
}
```

##### nushell table example

```rust
[[a b]; [1 a] [2 b]] | append [[a b]; [3 c]]
```

##### nu json syntax

[discord](https://discord.com/channels/601130461678272522/683070703716925568/908095523133751356)

##### engine-q table example
```rust
[[a b]; [1 a] [2 b]]

\\ both cases work the same
[[a, b]; [1, a] [2, b]]
```

ls | pivot | str capitalize Column0

ps | sort-by pid | first 10

##### the way to show value::record
$t1x | get 0 | grid

##### How do you run CI checks locally ?

```
cargo fmt
cargo check
cargo clippy
cargo test --all
```

[CI workflow steps manually](https://github.com/nushell/engine-q/blob/main/.github/workflows/ci.yml); &nbsp;&nbsp;
[discord link](https://discord.com/channels/601130461678272522/889232844101156914/904688334578794516)
