
### crate: nu_engine, file: eval.rs

##### eval_expresssion
when working with new operators like {* / -} you need to add more cases here or create a new function that deals with all of the cases...

```rust
Expr::BinaryOp(lhs, op, rhs) => {
    let op_span = op.span;
    let lhs = eval_expression(state, lhs)?;
    let op = eval_operator(op)?;
    let rhs = eval_expression(state, rhs)?;

    match op {
        Operator::Plus => lhs.add(op_span, &rhs),
        _ => Ok(Value::Nothing { span: expr.span }),
    }
}
```
