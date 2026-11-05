

Everything starts
[here](https://github.com/stormasm/nushell/blob/winnow_parser/crates/nu-parser/src/winnow/syntax/mod.rs)

And then goes
[here](https://github.com/stormasm/nushell/blob/winnow_parser/crates/nu-parser/src/winnow/syntax/parser/mod.rs)

```rust
/// Parse `source` into an AST plus any diagnostics (nu's `parse`).
pub(crate) fn parse<'a>(source: &'a str, config: &ParseConfig) -> (Ast<'a>, Vec<Diagnostic>) {
}

/// Parse the block covering `span` of `source` statement by statement (see
/// [`crate::winnow::syntax::parse_block_streaming`]).
pub(crate) fn parse_block_streaming<'a>(
```

---

```rust
rg WINNOW_PARSER
```

To see how the `nu-winnow-parser` is integrated into the Nushell code base

---

### What are captures ?

In language design, captures are values (often variables) that a block/closure/function-like object “remembers” from an outer scope.

Example conceptually:

- Outer scope has x.
- Inner block uses x without defining it locally.
- That inner block captures x.
- So they’re called “captures” because the inner construct is capturing bindings from its surrounding environment.
