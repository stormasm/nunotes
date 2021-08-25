
JT did all of the work for engine-q (so far), I am just writing some of my own
personal thoughts about what I have learned so far in working with
engine-q.

Engine-q is the next step in the evolution of nushell.

Its a cool new project with a slightly different twist on the core underpinnings of the decoupling and integration of the ParserState and the Eval Loop that underlies what happens every time you hit return at your nushell prompt.

The cool thing about shells in general is that at its core its a way to
interactively continue to change the state of memory in your running process.
This process can stay in memory for days; or until you kill it.

Nushell does many things well including enabling data visualization
but at its core all nushell does is give you a way to interact with the state
of the system over time with a set of custom commands that you define.

Engine-q delineates that state very cleanly with a ParserState that over
time is easy to change.  Every time you get a command prompt in enqine-q
you get the opportunity to continually modify the ParserState by creating
a ParserDelta which when you hit return at the prompt gets "merged" back
into the ParserState.

The Eval loop takes the ParserState along with the most recent block of code
you just last typed at the command prompt and returns to you a visualized Value.
That value can take many forms and depending on the form of it you see different
things.

Here is one of the things about engine-q I really like, it will enable moving
forward a very succinct small memory footprint that builds fast and has no other
dependencies.  At the moment engine-q's total footprint is 67 crates.  The parser has only one dependency, and nu-engine's only dependency is the parser.

This enables developers to use engine-q as their programming language for building application that use and need scripting functionality.  

All of the other custom commands inside nushell get added on top of engine-q along with packages like dataframes sit on top of engine-q and use its api
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

Moving forward having a really lightweight, blazing fast core engine that is
easy to modify as the nushell language evolves is exciting.  If nushell
developers understand this delineation and the core design pattern
of nushell being engine-q; then the product can be evolved over time without handicapping the future core engine with unneeded dependencies.
