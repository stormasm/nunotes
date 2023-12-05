
* [12/04/2023](https://discord.com/channels/601130461678272522/1181295282940547173/1181295360858140794)

lemme make a quick thread of thoughts

Sophia — Today at 10:06 AM
okay, I think this algorithm will work:
Parse the first token.
If it's a bare word, we're looking at a command (see: below)
If it's delimiter like {, (, [ etc, we know what this will be. Parse as expected (using the parsing algorithms to know if it's a record vs a block, that kind of thing)
The delimiter rule and parsing simple values will put the parser in math mode and it'll parse the rest of what it sees as a math expression
Parsing a command in the first pass looks like this:
Check if the command is a known keyword. If it is, go ahead and use the special parsing rules for that keyword
Otherwise, parse first token as a command name (we don't know yet if it's internal or external)
Command arguments are parsed as bare words unless they start with a delimiter. If they do, we parse based on that delimiter. This means something like {foo} can't be a string, and that's probably a good thing.

So the last rule above also means that something like -3 as a command argument is parsed as a bare word in the first parsing pass. Actually all flags are treated as bare words at this point, as we don't know exactly how they'll be used, or if they'll be external
The job of the second parsing pass / typechecking then is to use the definitions it knows about to finish parsing the calls. This lets us decide if they're internal or external. If they're external the bare words become external args.
If they're internal, we parse those bare words based on the shape the command needs

Sophia — Today at 10:14 AM
Oh, I guess I should also mention, another thing that gets special treatment is $. Once we see a token start with that, we switch to parsing a variable with an optional cell path

Sophia — Today at 10:43 AM
so in summary:
Stage 1: Math expressions, built-in keywords (including things like def which give us our custom commands), pipelines, and identify where commands are
Stage 2: Using the knowledge of the commands, parse the commands themselves

Kubouch — Today at 1:25 PM
Sounds good. I don't really have any particular thoughts about this.
For the scoping / name binding, I found out that it probably needs to be parsed in the first pass already because the AST nodes are laid "bottom up": First is the content of the block, then the block itself. I think it's easier to create the name bindings as we follow the parser in the "natural" direction in the first pass.

Sophia — Today at 1:28 PM
@Kubouch if you're struggling with it, I can see what I can do
but yeah, you start at the back and follow the node ids
the last node is the top node

Kubouch — Today at 1:30 PM
I think I could crack it, but if you'd create a tiny sample, that would make it quicker for me to catch on.
But yeah, I don't know how I'd make it a separate pass, also considering activating an overlay: You'd first have some nodes, but only later you'd find out which overlay / scope they belong to, I found that quite awkward, but maybe I'm missing something.

---

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
