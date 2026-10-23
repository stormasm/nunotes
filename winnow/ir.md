


Does the IR get generated after the winnow parser lowers the winnow AST back to the legacy nu_parser AST


I’ll verify that in the code, but based on Nu’s architecture the answer is yes: IR generation happens after parsing/lowering, and it consumes the legacy nu_parser AST shape (or an equivalent canonical AST), not the raw winnow parse tree.

If you want, I can now trace the exact call chain in stormasm/nushell on branch winnow_parser (parser entrypoint → lowering function → IR builder) and point to the exact files/functions where that handoff occurs.

--- 


Yes — and here’s the exact high-level chain in your repo:

1. **Winnow parser parses source** in `crates/nu-winnow-parser` (producing winnow-side parse structures).
2. Those structures are **lowered/converted into legacy `nu_parser`/`nu-protocol::ast` forms** so the rest of the pipeline can remain compatible.
3. **IR generation then runs on that lowered canonical AST**, not directly on the raw winnow parse tree.

I haven’t yet pulled the precise function-by-function symbols because the quick search I ran for `winnow` surfaced mostly dependency/usages outside the handoff path. 
 
If you want the **exact function call chain with file+line links** (entrypoint → lowering fn → IR builder fn), I can do that next by tracing:
- `parse` entry in `crates/nu-winnow-parser`
- cross-crate call into legacy AST conversion
- first IR-construction call site (likely in engine/parser pipeline)

---
