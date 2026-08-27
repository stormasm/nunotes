# IR-only call standardization plan

Status: **Phase 1 done** — next is Phase 2 (const IR-shaped `run_const`)  
Related: dual `engine::Call` / `CallImpl`, `requires_ast_for_arguments`, const eval, PR #14293 follow-up.

## Summary

Runtime block evaluation is already IR-only (`eval_block` → `eval_ir_block`; missing IR is a hard error). Parser AST remains the compile-time intermediate and is **not** being deleted. The goal is to standardize on **IR for command invocation** (`Command::run` / dual `CallImpl`) and finish residual AST call paths.

| Goal | Feasible? |
|------|-----------|
| Runtime blocks only on IR | Already done |
| Remove every AST artifact | No — AST → compile → IR VM is the architecture |
| IR-only command invocation / drop dual `Call` | Yes, intended — blocked by const eval shape, a few AST-arg commands, helper AST `eval_call` sites |

---

## Inventory (baseline before cleanup)

### `requires_ast_for_arguments` overrides

| Command | Why AST was needed | Phase |
|---------|-------------------|-------|
| `timeit` | Set but unused | **0 — removed** |
| `unlet` | `run()` walked AST for `VarId`; IR already uses `compile_unlet` | **0 — removed** |
| `metadata` | Pattern-match arg as var / cell path for origin | **1 — removed** (`LoadVariable { preserve_origin }` + value-only `run`) |
| `export-env` | `positional_nth(...).as_block()` for `BlockId` | **1 — removed** (evaluated `Closure`) |
| `default` | Deprecation warning when closure passed via variable | **1 — removed** (span-source `$` heuristic) |
| `attr example` | Block source text via Expression | **1 — removed** runtime AST need; `run_const` still uses `assert_ast_call` until const closures (phase 2) |

**Current allowlist:** empty (`crates/nu-command/tests/requires_ast_for_arguments.rs`).

Compiler gate: `crates/nu-engine/src/compile/call.rs` only retains `IrAstRef` on args when `requires_ast_for_arguments()` is true.

### Direct `assert_ast_call` sites

| Site | Role | Phase |
|------|------|-------|
| `engine::Call::{has_flag_const, get_flag_const, req_const, rest_const}` | Force AST for all `*_const` helpers | 2 |
| `if` `run_const` | Control-flow structure at parse time | 2 (keep AST const special path) |
| `attr example` `run_const` | Example block source | 1 / 2 |

### Residual AST `eval_call` producers (not dual-Call consumers only)

| Producer | Phase |
|----------|-------|
| `open` (`from <ext>` empty `ast::Call`) | 3 |
| `custom_completions` synthetic `ast::Call` | 3 |
| `documentation` example runner | 3 |
| `engine::Call::new` → empty `AstBox` | 3 |
| `known_external` Ast branches | 3–4 |
| `EvaluatedCall::try_from_ast_call` | 3–4 |

---

## Const evaluation recommendation

| Layer | Decision |
|-------|----------|
| Expression / pipeline const walker | **Keep AST-based** (`EvalConst` / `eval_constant`). Parse-time already has AST; a full const IR VM is optional later, not required for dual-Call removal. |
| `run_const` argument surface | **Move to IR-shaped values**: evaluate args with `eval_constant`, build temporary `ir::Call` + `ArgumentStack`, pass IR `engine::Call` into `run_const`. |
| Structural keywords in const (`if`) | **Dedicated AST const path** (or non-`Call` helper). Do not force full IR compile at parse time first. |

Rationale: `ir::Call` assumes pre-evaluated `Value`s on a stack. Const eval must still *produce* those values; the producer is naturally an expression walker. Collapsing `CallImpl` only requires `run_const` to receive IR-shaped calls, not that the whole const interpreter become IR.

---

## Phases

### Phase 0 — quick wins (this phase)

1. Remove unused `requires_ast_for_arguments` from **`timeit`**.
2. **`unlet`**: IR path is `compile_unlet` only; clear `requires_ast`, stub `run` like other IR-specialized commands.
3. Document **`default`** deprecation: keep `requires_ast` until the closure-via-variable warning is removed.
4. Add an **inventory test** listing commands that still require AST args so new opt-ins are deliberate.

### Phase 1 — redesign remaining AST-arg commands

1. **`export-env`**: take block/closure from evaluated arg (blocks already load as closure-like values under IR).
2. **`metadata`**: origin lookup without full `Expression` (value/var metadata or compiler-emitted info).
3. **`attr example`**: const-closure or parse-time string extraction (TODO already in source).
4. **`default`**: remove deprecation AST check when warning is retired.

When the set is empty: stop storing `IrAstRef` on call arguments; remove `Command::requires_ast_for_arguments` if unused.

### Phase 2 — IR-shaped const `run_const`

1. Implement const helpers on `ir::Call` that read Values from a const-time argument stack.
2. Change `eval_const_call` to build IR call after evaluating AST args.
3. Keep `if` (and similar) on a dedicated AST const path.
4. Delete `assert_ast_call` and AST-only `*_const` proxies on `engine::Call`.

### Phase 3 — migrate residual AST `eval_call` producers

Migrate `open`, completions, documentation, and empty `Call::new` to IR synthetic calls / `CallEval`.

### Phase 4 — collapse `CallImpl`

Delete `CallImpl::AstRef` / `AstBox`, simplify `CallExt`, IR-only plugins / known-externals.

### Phase 5 — optional cleanup

- Parser info as enum (not `Expression`).
- Fate of public AST `eval_expression` / `eval_call` for const, explain, tests.
- Keep `IrBlock.ast` for debugger if useful (orthogonal).
- Do **not** delete `nu_protocol::ast`.

### Dependency graph

```text
Phase 0 (timeit / unlet / inventory)
    │
Phase 1 (export-env, metadata, attr example, default)
    │  → empty requires_ast set
    │
Phase 2 (const → IR-shaped Call for run_const)
    │  → delete assert_ast_call
    │
Phase 3 (open, completions, docs, Call::new → IR)
    │
Phase 4 (delete CallImpl Ast variants)
    │
Phase 5 (parser_info enum, leftover eval_expression)  [optional]
```

---

## Key source files

| Area | Path |
|------|------|
| Dual Call | `crates/nu-protocol/src/engine/call.rs` |
| Command trait | `crates/nu-protocol/src/engine/command.rs` |
| IR Call | `crates/nu-protocol/src/ir/call.rs` |
| CallExt | `crates/nu-engine/src/call_ext.rs` |
| Compile call / unlet | `crates/nu-engine/src/compile/call.rs` |
| Runtime IR | `crates/nu-engine/src/eval_ir.rs` |
| Const eval | `crates/nu-protocol/src/eval_const.rs` |
| Inventory test | `crates/nu-command/tests/requires_ast_for_arguments.rs` |

---

## Progress log

- **2026-08-09**: Plan written; Phase 0 implemented:
  - Removed `requires_ast_for_arguments` from `timeit` (unused).
  - `unlet`: cleared AST opt-in; stubbed `Command::run` (IR uses `compile_unlet` only).
  - Documented remaining opt-ins (`default`, `metadata`, `export-env`, `attr example`).
  - Inventory test: `crates/nu-command/tests/requires_ast_for_arguments.rs`.
- **2026-08-09**: Phase 1 implemented:
  - **`export-env`**: `call.req::<Closure>()` for block id; no AST.
  - **`metadata`**: value-only `run`; bare vars compile with `LoadVariable { preserve_origin: true }`; `PushPositional`/`AppendRest` no longer rewrite value spans.
  - **`default`**: deprecation warning via span source starting with `$`.
  - **`attr example`**: runtime uses string/closure values; `run_const` still `assert_ast_call` for block source until const closures.
  - Inventory allowlist is empty — no command sets `requires_ast_for_arguments`.
