
# engine-q notes

* [fdncred](./engine-q/fdncred.md)
* [table syntax](./engine-q/table-syntax.md)

### Send back no data

```rust
Ok(PipelineData::Value(Value::Nothing { span: call.head }))
```

#### How do you run CI checks locally ?

```
alias ciman='cargo fmt; cargo check; cargo clippy; cargo test --all'

cargo fmt
cargo check
cargo clippy
cargo test --all
```

[CI workflow steps manually](https://github.com/nushell/engine-q/blob/main/.github/workflows/ci.yml); &nbsp;&nbsp;
[discord link](https://discord.com/channels/601130461678272522/889232844101156914/904688334578794516)

#### trait CustomValue

[discord](https://discord.com/channels/601130461678272522/889232844101156914/911337922890985512)

#### follow_cell_path

```rust
PathMember::Int
PathMember::String
```

The String arguments to follow_cell_path are for the column names.  
The Int arguments to follow_cell_path are for the rows.

```rust
[[a,b];[rick,pete], [bill,paul]] | get a
[[a,b];[rick,pete], [bill,paul]] | get b
[[a,b];[rick,pete], [bill,paul]] | get 0
[[a,b];[rick,pete], [bill,paul]] | get 1
```

For more details...
```rust
rg follow_cell_path
```

```rust
fn run(
    &self,
    engine_state: &EngineState,
    stack: &mut Stack,
    call: &Call,
    input: PipelineData,
) -> Result<nu_protocol::PipelineData, nu_protocol::ShellError> {
    let columns: Vec<CellPath> = call.rest(engine_state, stack, 0)?;
    let span = call.head;
```

In lots of commands you see the "call.rest" feature, this functionality
is defined in the nu-engine crate in [call_ext](https://github.com/nushell/engine-q/blob/main/crates/nu-engine/src/call_ext.rs) 

```rust
pub trait CallExt {
```
