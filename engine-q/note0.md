
# Note on Engine-q

JT did all of the work for engine-q (so far), I am just writing some of my own
personal thoughts about what I have learned so far in working with
engine-q.

[Engine-q](https://github.com/jntrnr/engine-q)
is the next step in the evolution of nushell.

Its a cool new project with a slightly different twist on the core underpinnings of the decoupling and integration of the ParserState and the Eval Loop that underlies what happens every time you hit return at your nushell prompt.

The cool thing about shells in general is that at its core its a way to
interactively continue to change the state of memory in your running process.
This process can stay in memory for days; or until you kill it.

Nushell does many things well including enabling data visualization
but at its core all nushell does is give you a way to interact with the state
of the system over time with a set of custom commands that you define.

Engine-q delineates that state very cleanly with a **ParserState** that over
time is easy to change.  Every time you get a command prompt in enqine-q
you get the opportunity to continually modify the ParserState by creating
a **ParserDelta** which when you hit return at the prompt gets "merged" back
into the ParserState.

The Eval loop takes the ParserState and a new Stack along with the most recent block of code you just last typed at the command prompt and returns to you a visualized Value. That value can take many forms and depending on the form of it you see different things.

### Crate Dependencies

Here is one of the things about engine-q I really like, it will enable moving
forward --- a very succinct small memory footprint that builds fast and has no other dependencies.  At the moment engine-q's total footprint is

* cargo build = 65 crates
* cargo test  = 89 crates

The nu-parser has only one dependency, and nu-engine's only dependency is the parser.

This enables developers to use engine-q as their programming language for building application that use and need scripting functionality.  

All of the other custom commands inside nushell get added on top of engine-q along with packages sit on top of engine-q and use its api
but it does not influence or add dependencies to it.

In the current nushell here are the stats

* nu-parser has 118 crates as dependencies
* nu-engine has 226 crates as dependencies
* nushell has 553 crates as dependencies

* nushell has 329 crates as dependencies with only these default features

```rust
default = [
    "nu-cli/shadow-rs",
    "ctrlc-support",
    "term-support",
    "rustyline-support",
]
```

Moving forward having a really lightweight, blazing fast set of design patterns
or crates called nu-parser and nu-engine that is easy to modify as the nushell language evolves is exciting.  

### The Details

##### If - else in the current nushell

In the current nushell if you type this it works...

```rust
if $true { echo $true } { echo $false }
```

But if you type this

```rust
if $true { echo $true }

error: Type Error
  ┌─ shell:3:10
  │
3 │ if $true { echo $true }
  │          ^^^^^^^^^^^^^^ Expected operator, found { echo $true }
```

The reason you get this error is because if expects an else clause
as it is required from the if command signature.

```rust
fn signature(&self) -> Signature {
    Signature::build("if")
        .required(
            "condition",
            SyntaxShape::MathExpression,
            "the condition that must match",
        )
        .required(
            "then_case",
            SyntaxShape::Block,
            "block to run if condition is true",
        )
        .required(
            "else_case",
            SyntaxShape::Block,
            "block to run if condition is false",
        )
}
```

But the error you receive without the else clause is not really
accurate or correct.  It says "expected operator", but ideally
you would like the error message to say something along the lines
of "you forgot your else clause which is required"

However, right now the "condition" of the if clause is a
**SyntaxShape::MathExpression** where ideally you would like
it to be more of a **SyntaxShape::Expression**, where any type of Condition
should work for an if condition as long as it returns some type
of a Boolean.

##### If - else in engine-q

In engine-q if is defined like this:

```rust
let sig = Signature::build("if")
    .required("cond", SyntaxShape::Expression, "condition")
    .required("then_block", SyntaxShape::Block, "then block")
    .optional(
        "else",
        SyntaxShape::Keyword(b"else".to_vec(), Box::new(SyntaxShape::Expression)),
        "optional else followed by else block",
    );
working_set.add_decl(sig.into());
```

One nice thing is that the else block is no longer required but
is optional and secondly the SyntaxShape::Expression is more generic
and not tied to just MathExpressions.

Also the following syntax is available:

```rust
if $true {10} else if $true {20} else if $true {10} else {20}

if <cond>     else if <cond>     else if <cond>     else <cond>
```

##### Things left to be done in engine-q

JT has put up a
[Todo List](https://github.com/jntrnr/engine-q/blob/main/TODO.md)
in the engine-q repo but as I dig deeper
into engine-q it becomes apparent to me things left to be done as well.

```rust
match op {
       Operator::Equal
       | Operator::NotEqual
       | Operator::LessThan
       | Operator::GreaterThan
       | Operator::LessThanOrEqual
       | Operator::GreaterThanOrEqual => {
           value::compare_values(op, left, right).map(UntaggedValue::boolean)
       }
       Operator::Contains => string_contains(left, right).map(UntaggedValue::boolean),
       Operator::NotContains => string_contains(left, right)
           .map(Not::not)
           .map(UntaggedValue::boolean),
       Operator::Plus => value::compute_values(op, left, right),
       Operator::Minus => value::compute_values(op, left, right),
       Operator::Multiply => value::compute_values(op, left, right),
       Operator::Pow => value::compute_values(op, left, right),
       Operator::Divide => value::compute_values(op, left, right).map(|res| match res {
           UntaggedValue::Error(_) => UntaggedValue::Error(ShellError::labeled_error(
               "Evaluation error",
               "division by zero",
               &right.tag.span,
           )),
           _ => res,
       }),
       Operator::Modulo => value::compute_values(op, left, right).map(|res| match res {
           UntaggedValue::Error(_) => UntaggedValue::Error(ShellError::labeled_error(
               "Evaluation error",
               "division by zero",
               &right.tag.span,
           )),
           _ => res,
       }),
       Operator::In => inside_of(left, right).map(UntaggedValue::boolean),
       Operator::NotIn => inside_of(left, right).map(|x| UntaggedValue::boolean(!x)),
       Operator::And => match (left.as_bool(), right.as_bool()) {
           (Ok(left), Ok(right)) => Ok(UntaggedValue::boolean(left && right)),
           _ => Err((left.type_name(), right.type_name())),
       },
       Operator::Or => match (left.as_bool(), right.as_bool()) {
           (Ok(left), Ok(right)) => Ok(UntaggedValue::boolean(left || right)),
           _ => Err((left.type_name(), right.type_name())),
       },
   }
}
```

Port all of the
[operator](https://github.com/nushell/nushell/blob/main/crates/nu-engine/src/evaluate/operator.rs) code over to engine-q which would enable

```rust
if (3 > 4) { 10 } else { 20 }
```

currently only this is available

```rust
if $true { 10 } else { 20 }
```

once the operator code lands then the above statement will work as well.

There are many other things like this I could outline, but for now
hopefully this gives you an idea for where things are headed...

### Digging Deeper

I have put together an outline of all of the current engine-q datastructures

[engine-q-ds.md](./engine-q-ds.md)

For the following statement in engine-q

```rust
if $true { 10 } else { 20 }
```

Here is what the call stack looks like currently in the
[most recent codebase commit](https://github.com/jntrnr/engine-q/commit/8ab7b27d4fb2461e512581c3d15bb9ffc93f9a4f)

[stack.rs](./stack.rs)   
[stack.txt](./stack.txt)

### Reedline

An added bonus of using engine-q is that you get to use
[reedline](https://github.com/jntrnr/reedline).   
You can play around with it as well as see how
[syntax_highlighting](https://github.com/jntrnr/engine-q/blob/main/crates/nu-cli/src/syntax_highlight.rs) works.

### Benchmark comparisons between engine-q and nushell

JT provided me the following benchmark test he ran comparing
engine-q with nushell...   
Based on his test this appears to
be a speed up of almost 10 times or **one order of magnitude** !

This is a benchmark from engine-q running on JT's machine.

> benchmark { for x in 1000000 { } }   
70 ms

This is a benchmark from nushell running on JT's machine.

> benchmark { for x in 0..1000000 {} }   
757 ms

**On my slow macbook** with 8GB of memory there is even a more impressive speed up...

841 ms for engine-q and almost 13 seconds for nushell...
