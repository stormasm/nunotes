

### Conclusion

I think we should land the code you have so far and then keep going
with the following goals in mind...

### See if the AI can figure this problem out for us

We are going to need the Ai to help us get over the line
just like we used it to get to this point with the winnow parser.

However, I believe if we can guide it further on its journey and tell it
what we want it to do we can be in a much better place with the
winnow parser.

### Goal 01 - Completely remove the fallback mechanism

We can direct the AI to take out the fallback mechanism and just use
the code currently in place with no fallback.


### Goal 02 - Generate the IR from the Winnow AST

Hopefully we can direct the AI to generate out the IR from the Winnow AST
and bypass the lowering process and go directly to the IR.

### Goal 03 - One AST instead of Two

Currently there are two AST's which I don't believe we will need in the end.

- [Winnow AST](https://github.com/fdncred/nushell/blob/winnow_parser/crates/nu-winnow-parser/src/ast/mod.rs)
- [Legacy AST](https://github.com/fdncred/nushell/tree/winnow_parser/crates/nu-protocol/src/ast)

- We want to have one AST.
- Having to maintain and update two AST's is extra work and unnecessary.

### Goal 04 - One parser code base instead of two

Once we figure out all of the these steps we should be able
to remove the legacy parser completely and just have the winnow
parser going forward.
