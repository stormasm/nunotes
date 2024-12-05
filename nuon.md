
### How parsing Nuon works

- parser.rs: parse_table_expression

#### Reference

```rust
[[name, age]; [rick 23] [bill 34]]
```

- [windsoilder answer to my nuon questions about how parsing works](https://discord.com/channels/601130461678272522/683070703716925568/1314117178735329300)

##### Why does an empty return in repl trigger parse_math_expression to get called four (4) times ?

The behavior actually happened before repl, nushell add menus to line_editor, in nu-cli/src/reedline_config:add_menus function, there are 4 default menus to add, each menu triggers one parse_math_expression call.

---

##### Why does this nuon code get parsed in parse_math_expression ?

It's because it's a math expression, in is_math_expression_like function, it returns true if the input starts with [

I think it's because in nushell, many things are semantic mathematical expressions.

Obvious example: 3 + 4 is a math expression.
Not obvious example: the number 3 itself is also a math expression, it's just an expression with no rhs.

---

And then after the table gets displayed I see the parse_math_expression get called the four (4) times as noted above

After table gets displayed, a new loop_iteration begins.  So 4 default menus are parsed, evaluated, and added.
