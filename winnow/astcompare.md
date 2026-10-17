
- [ref](https://github.com/stormasm/nushell/tasks/9a98e547-c882-4525-815c-c624f6137bec)

# Comparing AST outputs from nu-winnow-parser and nu-parser

## Conclusion

The ASTs are intentionally different, though structurally analogous. nu-winnow-parser produces a source-oriented syntax tree (nu\_winnow\_parser::ast::Ast); nu-parser produces Nushell’s engine-oriented nu\_protocol::ast::Block. They are not directly equal or fully isomorphic Rust structures.

In the experimental integration, the winnow tree is lowered into the same nu\_protocol AST type that Nushell consumes. The dedicated frontends compare harness checks those final results in a normalized textual form—not by comparing the two crates’ AST types directly.

## Parse entrypoints and integration

* Standalone nu-winnow-parser::parse returns Result\<Ast\<'\_\>, ParseError\>, while parse\_lenient returns that AST with diagnostics. Its streaming entrypoint, parse\_block\_streaming, hands individual statements to a sink. \[ /home/runner/work/nushell/nushell/crates/nu-winnow-parser/src/lib.rs:42–85 \]  
* nu-parser publicly re-exports its parse function, whose implementation returns Arc\<Block\>. \[ /home/runner/work/nushell/nushell/crates/nu-parser/src/lib.rs:39–43; /home/runner/work/nushell/nushell/crates/nu-parser/src/parse\_captures\_compile.rs:594–600 \]  
* With the winnow-parser experimental option enabled, nu-parser invokes the winnow front end for a file block. If it cannot handle the block, the caller lexes and parses it through the classic parser. \[ /home/runner/work/nushell/nushell/crates/nu-parser/src/parse\_captures\_compile.rs:760–780 \]  
* The integration parses blocks statement by statement, then lowers each winnow statement. When lowering encounters an error or unsupported construct, it hands that statement to the classic parser. \[ /home/runner/work/nushell/nushell/crates/nu-parser/src/winnow/driver.rs:1–14; /home/runner/work/nushell/nushell/crates/nu-parser/src/winnow/lower/statement.rs:48–78 \]  
* The winnow-parser experimental option is explicitly described as lowering the winnow syntax tree into nu-parser’s AST. \[ /home/runner/work/nushell/nushell/crates/nu-experimental/src/options/winnow\_parser.rs:18–25 \]

## Structural similarities and differences

Both trees share the broad shape block → pipelines → pipeline elements → expressions. The winnow AST documents that structure and follows nu-protocol’s AST closely. \[ /home/runner/work/nushell/nushell/crates/nu-winnow-parser/src/ast/mod.rs:10–13, 81–127 \]

But their node data and responsibilities differ:

* Source fidelity: nu-winnow-parser::Ast retains the source reference, comments, shebang, and spans of certain ignored source text. Its pipelines also retain leading/trailing comments, terminators, dangling pipes, and a pipeline span. \[ /home/runner/work/nushell/nushell/crates/nu-winnow-parser/src/ast/mod.rs:28–50, 81–113 \] nu\_protocol::ast::Block instead carries engine data such as captures, scope state, redirect behavior, and compiled IR; its Pipeline contains elements but no corresponding comment or terminator fields. \[ /home/runner/work/nushell/nushell/crates/nu-protocol/src/ast/block.rs:10–33; /home/runner/work/nushell/nushell/crates/nu-protocol/src/ast/pipeline.rs:109–114, 166–169 \]  
* Syntax nodes versus semantic nodes: The winnow tree has syntax-specific variants such as Duration, Filesize, Assignment, DynamicCall, Let, Def, and If. \[ /home/runner/work/nushell/nushell/crates/nu-winnow-parser/src/ast/mod.rs:245–364 \] The protocol Expr instead includes engine-facing forms such as Var(VarId), VarDecl(VarId), Call, Operator, RowCondition(BlockId), and compiled block references. \[ /home/runner/work/nushell/nushell/crates/nu-protocol/src/ast/expr.rs:12–58 \]  
* Expression metadata: A winnow Expression holds a span and syntax expression. A protocol Expression additionally has a SpanId and inferred Type. \[ /home/runner/work/nushell/nushell/crates/nu-winnow-parser/src/ast/mod.rs:196–204; /home/runner/work/nushell/nushell/crates/nu-protocol/src/ast/expression.rs:11–18 \]  
* Calls and arguments: The winnow call tree stores a textual head and source-order syntax arguments, including EndOfOptions and explicit spread spans; the protocol call stores a resolved DeclId, a head span, parser metadata, and arguments categorized for engine use. \[ /home/runner/work/nushell/nushell/crates/nu-winnow-parser/src/ast/mod.rs:1055–1080, 1132–1149; /home/runner/work/nushell/nushell/crates/nu-protocol/src/ast/call.rs:10–34, 119–126 \]

## What lowering changes

Lowering is not just a field-by-field conversion. It uses the working set to resolve declarations and variables, apply command signatures, infer and check types, and create engine AST nodes. \[ /home/runner/work/nushell/nushell/crates/nu-parser/src/winnow/lower/mod.rs:1–30, 71–115 \]

Concrete examples:

* A winnow If becomes a protocol Call to if; lowering constructs its condition and branch arguments and computes an output type. \[ /home/runner/work/nushell/nushell/crates/nu-parser/src/winnow/lower/keyword.rs:105–149 \]  
* A winnow let binding becomes a protocol call with positional arguments, after lowering the value, creating the variable, and checking its type. \[ /home/runner/work/nushell/nushell/crates/nu-parser/src/winnow/lower/keyword.rs:458–522 \]  
* Calls resolve a declaration and assign their syntax arguments according to its signature. \[ /home/runner/work/nushell/nushell/crates/nu-parser/src/winnow/lower/call.rs:133–199, 202–220 \]  
* Variables and paths are resolved during lowering; for leaf values whose meaning depends on the expected shape, lowering invokes classic leaf parsing. \[ /home/runner/work/nushell/nushell/crates/nu-parser/src/winnow/lower/value.rs:22–40, 63–118, 121–145 \]  
* WSpan values are translated into protocol spans with the source offset applied. \[ /home/runner/work/nushell/nushell/crates/nu-parser/src/winnow/lower/mod.rs:82–93 \]

## Equality and compatibility checks

There is no direct cross-crate AST equality assertion. The standalone winnow AST derives PartialEq, but the protocol Block does not; their types and purposes differ. \[ /home/runner/work/nushell/nushell/crates/nu-winnow-parser/src/ast/mod.rs:28–31; /home/runner/work/nushell/nushell/crates/nu-protocol/src/ast/block.rs:10–11 \]

The strongest in-repository comparison is frontends compare. It parses each file once with the classic nu-parser front end and once with the experimental winnow front end, then compares custom rendered output. That renderer resolves IDs to names and includes the block, declarations, errors, and highlighting, allowing different internal ID allocation order to compare equal. \[ /home/runner/work/nushell/nushell/crates/nu-winnow-parser/tools/nushell-harness/src/bin/frontends.rs:14–21, 172–198, 499–528 \] The runbook says the comparison should report every file identical. \[ /home/runner/work/nushell/nushell/crates/nu-winnow-parser/TESTING.md:141–158 \]

Other coverage is compatibility-oriented rather than a direct AST comparison: the standalone crate’s fixture tests compare its own output to golden AST/error files, and the runbook specifies running the Nushell workspace test suite with the experimental option enabled. \[ /home/runner/work/nushell/nushell/crates/nu-winnow-parser/TESTING.md:44–52, 141–154 \]

Bottom line: the standalone crate’s AST and nu-parser’s AST are not exactly the same and are not fully structurally equivalent. They are deliberately different representations with overlapping syntax structure. The integration’s goal is compatibility at the resulting nu\_protocol AST/engine level, validated by lowering, fallback to the classic parser, and the normalized front-end comparison harness.

