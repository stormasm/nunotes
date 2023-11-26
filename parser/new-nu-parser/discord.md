
* [11/25/2023](https://discord.com/channels/601130461678272522/683070703716925568/1178085955215831111)

Question about new-nu-parser ?

I am trying to get into my mind how the integration is going to work and I was wondering...

Is the AstNode https://github.com/sophiajt/new-nu-parser/blob/main/src/parser.rs#L40

The same as the https://github.com/nushell/nushell/blob/main/crates/nu-protocol/src/ast/expr.rs

#### sophia

@storm close but different. AstNode is all of the forms in one place, where Expr is only expressions

#### storm

when we do the integration work will AstNode replace Expr or will we still keep Expr around ?

I am trying to write Expression and was curious what I should do ?

https://github.com/nushell/nushell/blob/main/crates/nu-protocol/src/ast/expression.rs

#### sophia
It'll replace it

#### storm
cool ---- for now that is all I need to know 🙂  Thank you !
