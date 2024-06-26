
### take it from the top given a nushell script

- [evaluate_source](https://github.com/nushell/nushell/blob/main/crates/nu-cli/src/util.rs#L243)

### pub trait Eval

- nu-protocol/src/eval_base.rs

```rust
fn eval<D: DebugContext>(
     state: Self::State<'_>,
     mut_state: &mut Self::MutState,
     expr: &Expression,
 ) -> Result<Value, ShellError> {
     match &expr.expr {
         Expr::Bool(b) => Ok(Value::bool(*b, expr.span)),
         Expr::Int(i) => Ok(Value::int(*i, expr.span)),
         Expr::Float(f) => Ok(Value::float(*f, expr.span)),
```

### eval.rs pub fn

*eval_expression* uses the *fn eval* in the above trait

- eval_call
- eval_expression
- eval_expression_with_input
- eval_block_with_early_return
- eval_block
- eval_subexpression
- eval_variable
