
### How to Port a Command from Nushell to Engine-q

This note needs lots of help in making it succinct, clear,
and most importantly easy to understand so if you see ways
to make this better please don't hesitate to improve this
document.

This document makes some assumptions :
* that you already have some familiarity with Nushell

All commands have the standard boilerplate methods including

* name
* usage
* signature
* run

Name and usage is fairly obvious

#### Signature

The signature can have required parameters, optional parameters, or no parameters.  An example of a command with no parameters is the
[command length](https://github.com/nushell/engine-q/blob/main/crates/nu-command/src/filters/length.rs).

```rust
fn signature(&self) -> nu_protocol::Signature {
  Signature::build("length")
}
```

#### Run

```rust
fn run(
    &self,
    context: &EvaluationContext,
    call: &Call,
    input: Value,
) -> Result<nu_protocol::Value, nu_protocol::ShellError> {
```

Run is where all of the action happens when porting over a command from nushell or writing a new command.

```rust
pub struct Call {
    /// identifier of the declaration to call
    pub decl_id: DeclId,
    pub head: Span,
    pub positional: Vec<Expression>,
    pub named: Vec<(String, Option<Expression>)>,
}
```
