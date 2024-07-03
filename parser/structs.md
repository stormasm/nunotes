
#### block.rs, pipeline.rs, expression.rs

```rust
pub struct Block {
    pub signature: Box<Signature>,
    pub pipelines: Vec<Pipeline>,
    pub captures: Vec<VarId>,
    pub redirect_env: bool,
    /// The block compiled to IR instructions. Not available for subexpressions.
    pub ir_block: Option<IrBlock>,
    pub span: Option<Span>, // None option encodes no span to avoid using test_span()
}

pub struct Pipeline {
    pub elements: Vec<PipelineElement>,
}

pub struct PipelineElement {
    pub pipe: Option<Span>,
    pub expr: Expression,
    pub redirection: Option<PipelineRedirection>,
}

pub struct Expression {
    pub expr: Expr,
    pub span: Span,
    pub span_id: SpanId,
    pub ty: Type,
    pub custom_completion: Option<DeclId>,
}
```

### Examples

```rust
ast '
"01/22/2021" |
parse "{month}/{day}/{year}" |
get year'
```

- [ir examples](./../ir/examples.md)
