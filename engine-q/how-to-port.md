
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

```rust
fn run(
    &self,
    context: &EvaluationContext,
    call: &Call,
    input: Value,
) -> Result<nu_protocol::Value, nu_protocol::ShellError> {
```




```rust
pub struct Call {
    /// identifier of the declaration to call
    pub decl_id: DeclId,
    pub head: Span,
    pub positional: Vec<Expression>,
    pub named: Vec<(String, Option<Expression>)>,
}
```
