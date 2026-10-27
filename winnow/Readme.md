

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
