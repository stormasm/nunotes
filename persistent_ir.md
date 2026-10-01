# Persistent IR ("pickles")

This document records how Nushell's IR works today and how a parsed and lowered
program can be written to disk and executed later without parsing it again.
Part 1 is the investigation. Part 2 is the design that the prototype on this
branch implements.

Paths are relative to the repository root. Line numbers drift, so each claim
names the function too. `persistent_ir_small.md` covers the same design in
about a quarter of the length.

## Part 1. How the IR works today

### 1. Where the IR types live

`crates/nu-protocol/src/ir/mod.rs` defines everything:

- `IrBlock` holds `instructions`, a parallel `spans` vector, the `data` byte
  section, a parallel `ast` vector of optional `IrAstRef`s, a parallel
  `comments` vector, `register_count`, `file_count` and `scope_regions`.
- `Instruction` is a 24-byte enum. A `const` assertion enforces the size.
- `Literal` covers inline constants.
- `RedirectMode` covers stdout/stderr routing.
- `DataSlice` is a `(start, len)` pair into `IrBlock::data`.
- `crates/nu-protocol/src/ir/call.rs` holds the runtime `ir::Call` that
  `eval_call` builds. It is not persisted.

`IrBlock`, `Instruction`, `Literal` and `RedirectMode` already derive serde.
`view ir --json` uses that, and so does the plugin engine call `get_block_ir`.
Two fields are `#[serde(skip)]`: `IrBlock::scope_regions` and
`ast::Block::scope_bindings`. Both hold process-local ids.

### 2. Where the AST is lowered

`nu_engine::compile` (`crates/nu-engine/src/compile/mod.rs`) turns one
`ast::Block` into one `IrBlock`. The work is split across these files:

- `expression.rs` lowers expressions.
- `call.rs` lowers calls, externals and `--help`.
- `keyword.rs` lowers `if`, `match`, `let`, `mut`, `try`, `loop`, `while`,
  `for`, `break`, `continue` and `return`.
- `operator.rs` lowers operators.
- `redirect.rs` lowers redirections.
- `builder.rs` holds `BlockBuilder`, which allocates registers, data and
  labels.

Lowering is not a separate phase after parsing. The parser calls the compiler
as soon as each block is finished:

- the top-level block in `parse_with_block_cache`
  (`crates/nu-parser/src/parse_captures_compile.rs`)
- closures and blocks in `parse_expressions.rs` (the `compile_block` calls)
- `def` bodies in `parse_def.rs` (`compile_block_with_id`)
- module `export-env` blocks in `parse_module.rs`
- sourced files in `parse_source.rs`

The compiler reads the working set while it runs. It looks at decls
(`is_keyword`, `name`, `requires_ast_for_arguments`, `pipe_redirection`) and
nested blocks. It also resolves helper commands by name: `run-external`,
`run-internal`, `help` and `do`.

### 3. Where the IR is stored

The IR lives in `ast::Block::ir_block: Option<IrBlock>`
(`crates/nu-protocol/src/ast/block.rs`). Blocks live in
`StateWorkingSet::delta.blocks` during parsing and move into
`EngineState::blocks` on `merge_delta`. The top-level block of a parse is
returned to the caller and is not in `delta.blocks`.

Subexpressions and keyword bodies (`if`, `for`, `try` and the rest) have no
IR of their own. The compiler inlines their instructions into the parent
`IrBlock`.

### 4. How the IR is executed

`eval_block` (`crates/nu-engine/src/eval.rs`) calls `eval_ir_block`
(`crates/nu-engine/src/eval_ir.rs`). It takes a `&Block`, not an `&IrBlock`,
because it needs the block's `span` and `scope_bindings` too. It allocates
`register_count` registers and `file_count` file slots, then
`eval_ir_block_impl` steps through the instructions.

`eval_call` dispatches `Instruction::Call`. A decl with `block_id()` is a
custom command. The evaluator loads that block, checks input types against
`block.signature`, binds arguments into the variables named in the signature,
gathers `block.captures` and recurses. Any other decl runs through
`Command::run`.

### 5. How `view ir` finds and prints IR

`crates/nu-command/src/debug/view_ir.rs` accepts three targets:

- a closure, which gives its `block_id`
- a command name, looked up with `find_decl` and then `block_id()`
- an int, which is a decl id with `--decl-id` and a block id otherwise

It prints `IrBlock::display` (`ir/display.rs`). The display looks up each decl
id in the live engine state to print `decl 893 "ls"`. The name is not stored
in the IR.

### 6. Registers

`RegId` is `Id<marker::Reg, u32>`. It indexes the register file that
`eval_ir_block_inner` allocates for each invocation. `BlockBuilder`
allocates registers and reuses freed ones. `%0` always holds the block
input. Register numbers only mean something inside their `IrBlock`, so they
serialize as plain integers.

### 7. Instructions

`Instruction` is a flat enum. Every register operand is a `RegId`. Every
branch target is an instruction index inside the same `IrBlock`, so control
flow is structural. The table below classifies every field.

### 8. Literals and the data section

`data: Arc<[u8]>` only holds bytes that a `DataSlice` points at. That covers
string literals, raw strings, binary literals, paths, globs, flag names and
env var names. The "16 bytes of data" in the `ls | sort-by | grid` example
are `ignore-case` (11 bytes) and `color` (5 bytes). Everything else is inline:
ints, floats, filesizes, durations, `Box<CellPath>`, `Box<DateTime>`, match
patterns and parser info expressions.

The compiler never puts a `Value` in the IR. `Instruction::LoadValue` exists
but nothing emits it. Constants are not folded into the IR (see 14).

### 9. Declarations

A `DeclId` is an index into `EngineState::decls: Vec<Box<dyn Command>>`.
`StateWorkingSet::add_decl` appends and returns `num_decls() - 1`. See
"Declaration ids" below.

### 10. Blocks

A `BlockId` is an index into `EngineState::blocks`. The evaluator needs these
`ast::Block` fields at run time:

- `ir_block` holds the code.
- `signature` supplies parameter variables, types and defaults for custom
  commands.
- `captures: Vec<(VarId, Span)>` lists the variables a closure captures when
  its literal is evaluated.
- `redirect_env` decides whether env changes flow back to the caller.
- `span` is used for errors.

`pipelines` (the AST) is not needed to run the block. A few introspection
commands read it: `explain`, `ast`, completions and syntax highlighting.

### 11. Nested blocks, closures and custom commands

A program is not one IR tree. It is a graph of `Block`s that point at each
other through ids:

- `Literal::Block`, `Literal::Closure` and `Literal::RowCondition` carry a
  `BlockId`. The referenced block has its own `IrBlock`, signature and
  captures.
- A call to a custom command carries a `DeclId`. The decl's `block_id()`
  points at the body.
- `def` compiles to nothing (`load_empty`). The only thing that makes
  `foo` callable is the `BlockCommand` the parser pushed into
  `delta.decls`.
- `source` and `source-env` store the sourced file's `BlockId` in parser
  info.
- Modules store `env_block: Option<BlockId>`.

### 12. Spans

`Span { start, end }` is a pair of global byte offsets into one address
space. Every file the engine sees, including REPL lines and std library
virtual files, gets the next range (`add_file`, `next_span_start`). A span is
therefore meaningless without the file table of the process that created it.

`IrBlock::spans[i]` is the span of instruction `i`. Every AST `Expression`
also carries a `span` and a `span_id: SpanId` (an index into the engine's
`spans` table, used by IDE features). Every `Value` carries a span.

### 13. Variables and captures

A `VarId` is an index into `EngineState::vars`. Ids 0 to 3 are fixed:

| id | variable |
| -- | -------- |
| 0 | `$nu` |
| 1 | `$in` |
| 2 | `$env` |
| 3 | `$ans` |

At run time the stack stores `(VarId, Value)` pairs, so ids act as keys.
The evaluator still reads `Variable` metadata. `gather_arguments` checks
`engine_state.get_var(id).ty`, and `get_var` falls back to `const_val`.
Captures are `(VarId, Span)` pairs on the block.

### 14. Constants

`const` is evaluated at parse time (`eval_constant` in `parse_bindings.rs`).
The parser stores the result in `Variable::const_val` and the `const` call
compiles to `load_empty`. A use of the constant compiles to `LoadVariable`,
which misses on the stack and falls back to `engine_state.get_constant`. So
constant values live in the variable table, not in the IR. `use` copies a
module's constants onto the stack at run time from the same table.

### 15. Modules and overlays

`use`, `overlay use` and `module` do almost all of their work at parse time.
They parse the module file, add its blocks, decls and `Module` to the delta
and bind names in the current overlay. At run time the IR still calls them:

- `use` reads the `import_pattern` parser info (`ImportPattern` with
  `ModuleId` and `VarId`s), copies constants and runs the module's
  `env_block`.
- `overlay use` reads `overlay_expr` (`Expr::Overlay(ModuleId)`), runs the
  `env_block` and activates the overlay on the stack.
- `def`, `module`, `hide` and `extern` do nothing at run time.

`std` modules are embedded virtual files, added at startup by
`load_standard_library`. Only `std/prelude` is parsed then. `use std/assert`
in a script parses the module at that point, so its decls and blocks land in
the script's delta while its spans point into a file that already existed.

### 16. Plugins

`PluginDeclaration` (`crates/nu-plugin-engine/src/declaration.rs`) is
registered at startup from the plugin registry file. Its signature comes from
the registry, so the plugin does not run until it is called. `plugin use` in a
script adds plugin decls at parse time. Plugin decls get ordinary `DeclId`s in
registry order. Adding, removing or updating a plugin shifts every id after
it.

### 17. Externals

`^git status`, `git status` (when `git` is not a known command) and `./foo`
all become `Expr::ExternalCall`. `compile_external_call` rewrites that into a
normal `Call` to `run-external`. The compiler resolves `run-external` by name
at compile time. The head and arguments are pushed as positional
`GlobPattern` or `String` literals. PATH lookup happens at run time inside
`run-external`, so nothing about the external program is baked into the IR.
`extern` declarations (`KnownExternal`, in nu-parser) forward to
`run-external` at run time.

### 18. How the evaluator receives a program

For `nu script.nu`, `evaluate_file` (`crates/nu-cli/src/eval_file.rs`) runs
these steps:

1. It reads the file and sets `FILE_PWD`, `CURRENT_FILE` and `PROCESS_PATH`.
2. It creates a `StateWorkingSet`, calls `parse`, and exits on parse or
   compile errors.
3. It renames `main` and `main *` signatures to the script name and adds
   `script.nu` entries to the overlay.
4. It calls `engine_state.merge_delta(working_set.delta)`.
5. It calls `eval_block(&block)`. If the script defines `main`, it then parses
   and evaluates the one-line command `main <args>`.

The REPL, `-c` and `source` follow the same path: parse into a delta, merge,
evaluate the returned block.

### The key question: what lies between parsed source and IR?

The pipeline is roughly source, then parser, then AST, then IR, then
evaluator. The IR is not self-contained, though. Every cross-reference in it
is a dense index into an `EngineState` table:

- `DeclId` indexes `decls`.
- `BlockId` indexes `blocks`.
- `VarId` indexes `vars`.
- `ModuleId` indexes `modules`.
- `Span` is an offset into the file address space.
- `SpanId` indexes `spans`.
- `FileId` indexes `files`.

The parser fills those tables while it runs. It mutates a `StateDelta`, and
the compiler reads the same working set. Serializing an `IrBlock` alone
therefore gives you code that points into tables the next process does not
have.

What `parse` actually produces is the delta plus the top-level block:

| delta part | why the evaluator needs it |
| ---------- | -------------------------- |
| blocks | IR, signatures and captures of closures, custom commands and module env blocks |
| decls | `BlockCommand`s that make `def`s callable, plus `KnownExternal`s and `Alias`es |
| vars | types checked during argument binding, and `const` values |
| modules | `env_block`, constants and decl maps for `use` and `overlay use` |
| files | source text for error labels and for `view source` |
| spans | the `SpanId` table |
| scope (overlays) | name to id maps that `main` lookup, `help`, `which`, `scope` and `run-internal` read after the merge |

So the pickle sits after lowering and before execution, as intended. Its unit
is "delta plus top-level block", with the IR at the center. Loading it
reproduces the same `Block`/`IrBlock` structures, and the existing
`eval_ir_block` runs them. There is no second evaluator.

### Declaration ids

- **Where ids come from.** `add_decl` appends to one vector. Startup order is
  `EngineState::new`, then commands from nu-cmd-lang, nu-cmd-plugin,
  nu-command, nu-cmd-extra, nu-cli, nu-explore and nu-tui, then the std
  prelude, then plugins from the registry, then env/config files, then
  whatever the script defines.
- **They are process-local.** The same binary gives different ids depending on:
  - cargo features (`#[cfg]` in `default_context.rs`)
  - experimental options (`native-clip` registers extra commands)
  - `-n`
  - `--no-std-lib`
  - REPL vs script mode
  - plugin registry contents
  - config files

  This branch's debug build prints `decl 242 "ls"` for the example that
  printed `decl 893 "ls"` above.
- **They are not stable between versions.** Adding any command shifts
  everything after it.
- **Names are not unique.** Name to id is many-to-many:
  - `use spam` inserts `spam foo` as another key for `foo`.
  - `hide` keeps the decl but masks it.
  - Overlays can shadow each other.
  - The `main` rename leaves two keys for one decl.
- **No stable identity exists.** There is no hash, no fingerprint and no
  qualified key anywhere in nu-protocol, nu-engine, nu-parser or nu-cli.
- **Aliases disappear from the IR.** The parser replaces an alias call with
  the aliased `Call` (or `ExternalCall`), so the IR only references the
  target. The `Alias` decl still exists in the delta for `help` and `scope`.
- **The compiler binds some names itself.** `run-external`, `run-internal`,
  `help` and `do` are resolved by name at compile time, with scope
  visibility. A user `def do` changes what `try` compiles to.
- **Signatures matter.** The parser shaped the call's arguments from the
  signature. It decided which argument is a closure, which flag takes a value
  and what `...rest` swallows. The compiler also used `is_keyword`,
  `requires_ast_for_arguments` and `pipe_redirection`. If the decl a name
  resolves to later has a different shape, the old IR is wrong for it even
  though the name matches.

There are four ways a pickle could store `call decl 893 "ls"`:

- **A. Name only.** Store `call "ls"` and look the name up when loading.
- **B. Stable identity plus runtime id.** Store the name, some identity
  that stays the same between processes, and the id the decl had when the
  pickle was made.
- **C. Rebind.** Store enough about the decl to find the same decl in the
  engine that loads the pickle, and refuse to load if it isn't there.
- **D. Reuse an existing identity.** Use whatever stable key Nushell already
  has for a decl.

A doesn't work: a name can mean several decls, and it can't tell that a
command's signature changed. D doesn't work because Nushell has no stable
key to reuse (see the list above). The pickle uses C, storing the fields B
describes. Each decl the program uses from outside itself is recorded once
as `(producer id, name, shape)`:

- The producer id is the decl's id when the pickle was made. Instructions
  keep it, and the loader maps it to the new id.
- The shape is what parsing and lowering relied on: the command type, plus
  a hash of the signature, `requires_ast_for_arguments` and
  `pipe_redirection` (see "What the loader rebinds, and when it refuses").
  It stands in for B's stable identity.

The loader looks the name up in its own engine and requires the shape to
match exactly.

### Program data vs engine state

Nothing in `IrBlock` or `Block` refers to OS resources. The following is
created per invocation and never stored:

- registers (`PipelineExecutionData`)
- open redirection files
- the argument stack
- error handlers
- the `Stack`
- env vars
- plugin processes
- jobs

The only process-local things in the program data are the id spaces listed
above. They are what needs relocating or rebinding.

Per-instruction classification. "Plain" means the field is program data that
round-trips as is. "Relocate" means an id into a table the pickle carries
(`id >= base` at pickle time). "Rebind" means an id into a table the loading
process owns. "Source" means span or debug metadata.

| instruction | fields |
| ----------- | ------ |
| `unreachable`, `pop-error-handler`, `begin-finally`, `end-finally` | none |
| `load-literal` | `dst` plain, `lit` see the literal table |
| `load-value` | `dst` plain. `val` is a `Value`, so spans are source and closures relocate. The compiler does not emit it |
| `move`, `clone` | registers, plain |
| `collect`, `try-collect`, `span`, `drop`, `drain`, `drain-if-end`, `not` | register, plain |
| `load-variable`, `store-variable`, `drop-variable` | registers plain, `var_id` relocate (ids 0 to 3 are fixed) |
| `update-var-cell-path` | `var_id` relocate, registers plain |
| `load-env`, `load-env-opt`, `store-env` | `key` is a `DataSlice`, plain |
| `push-positional`, `append-rest` | register, plain |
| `push-flag`, `push-short-flag`, `push-named`, `push-short-named` | `DataSlice` and register, plain |
| `push-parser-info` | `name` plain. `info` is an `Expression`: typed ids relocate or rebind, spans and span ids are source. Three keys hide ids in `Expr::Int` (below) |
| `redirect-out`, `redirect-err` | `RedirectMode`, plain (`File { file_num }` is a local slot) |
| `check-err-redirected`, `open-file`, `write-file`, `close-file` | file slots and registers, plain |
| `call` | `decl_id` relocates (script-defined) or rebinds (builtin, plugin, prelude). `src_dst` plain |
| `string-append`, `glob-from`, `list-push`, `list-spread`, `record-insert`, `record-spread` | registers and flags, plain |
| `binary-op` | `Operator` and registers, plain |
| `follow-cell-path`, `clone-cell-path`, `upsert-cell-path` | registers, plain |
| `jump`, `branch-if`, `branch-if-empty`, `iterate`, `unwind-jump`, `on-error`, `on-error-into`, `finally-into` | local instruction indexes and registers, plain |
| `match` | `Pattern` binds `VarId`s (relocate) and carries spans (source). `src` and `index` plain |
| `check-match-guard`, `return`, `return-early` | register, plain |

| literal | fields |
| ------- | ------ |
| `Bool`, `Int`, `Float`, `Filesize`, `Duration`, `Date`, `Nothing`, `Empty` | plain |
| `Binary`, `String`, `RawString`, `Filepath`, `Directory`, `GlobPattern` | `DataSlice`, plain |
| `Block`, `Closure`, `RowCondition` | `BlockId`, relocate (the target block must travel with the pickle) |
| `Range` | registers, plain |
| `List`, `Record` | capacity, plain |
| `CellPath` | members are plain, member spans are source |

| `IrBlock` field | class |
| --------------- | ----- |
| `instructions` | see above |
| `spans` | source |
| `data` | plain |
| `ast` (`IrAstRef`) | AST expressions, so ids relocate or rebind and spans are source |
| `comments` | debug only, plain |
| `register_count`, `file_count` | plain |
| `scope_regions` | derived at compile time from `scope_bindings`. It holds process-local ids and is `serde(skip)` |

Three parser-info keys carry ids as untyped `Expr::Int`. No type-driven
mechanism can find them:

| key | kind | set by |
| --- | ---- | ------ |
| `block_id` | `BlockId` | `parse_source.rs` for `source`, `source-env` and `run` |
| `main_block_id` | `BlockId` | `parse_source.rs` for `run` |
| `help_decl_id` | `DeclId` | `compile/call.rs` for the `cmd --help` rewrite |

The other ID-carrying parser info is typed and needs no special case:

| key | form |
| --- | ---- |
| `import_pattern` | `ImportPattern` with `ModuleId` and `VarId`s |
| `overlay_expr` | `Expr::Overlay(ModuleId)` |
| `dirs_var` | `Expr::Var` |

## Part 2. Design

### Overview

```text
process A                                process B
---------                                ---------
source                                   pickle bytes
  -> parse + compile                       -> check header and checksum
  -> StateDelta + top-level Block          -> resolve imports against EngineState B
  -> serialize, collecting imports         -> deserialize, relocating every id and span
  -> pickle bytes                          -> replay into a StateWorkingSet
                                           -> same main rename, merge_delta, eval_block
```

The model is an object file with a link step:

- Every id `>= base` (the table size when the parse started) is local to
  the delta and relocates to `new_base + (id - old_base)`.
- Every id `< base` is an import. The loader must resolve it against the
  current engine, or refuse to load.

### Relocation through serde

Ids and spans appear in hundreds of places: every `Expression`, every
`Value`, signatures, patterns, cell paths, modules and overlay maps. A
hand-written visitor would need one arm for each of about 40 `Expr` variants
plus every `Value` variant. It would also go stale silently the first time
someone adds a field that holds an id. A missed id does not crash. It points
at an unrelated declaration.

So relocation happens inside serde. `Id<M>` already has hand-written serde
impls, and `Span` gets one with the same wire shape. Both call into
`nu_protocol::relocation`, which keeps a scoped thread-local context:

- On serialize, an `ImportCollector` records every id below its base, keyed
  by `IdKind`. It never changes what gets written.
- On deserialize, an `IdRelocator` maps each id. Local ids and spans shift.
  Imports go through tables the loader built beforehand, one per `IdKind`,
  indexed by the old id. An unresolvable import is a deserialization error,
  never a guessed index.
- While no thread has a context, as in the plugin protocol and
  `view ir --json`, the hooks read one atomic counter and do nothing.

This covers every typed id by construction. The three `Expr::Int` parser-info
keys get an explicit pass over `PushParserInfo` instructions on both sides.
The pass only looks at `PushParserInfo` instructions. One case stays
unrelocated. A `Call` that carries one of those keys can be nested inside an
`IrAstRef` argument expression, for example an AST-requiring command given
`(source x.nu)` as an argument.

### What the loader rebinds, and when it refuses

| import | rule |
| ------ | ---- |
| `DeclId` | Resolved by the names the pickling shell calls it by, all of them when it has several (`use m; use m *` names one command `m foo` and `foo`). Each has to name the same decl in the loading engine, so `m foo` never links to some other `foo`. The shape must match: the command type (which covers keywords and plugins) in the clear, plus a 64-bit FNV-1a hash of `requires_ast_for_arguments`, `pipe_redirection` and the signature with names, descriptions, var ids, defaults and completions removed. Otherwise loading fails, naming the command. A builtin that a definition of the same name hides has no name in the shell; it keeps its own, which only it has, and the loader accepts the single builtin with that name and shape. Any other command without a name makes the pickler refuse |
| `VarId` | 0 to 3 (`$nu`, `$in`, `$env`, `$ans`) keep their fixed ids. Constants from the startup environment are resolved by the name the pickling shell makes them visible under, like the source would. `use` and `source` reference `$NU_LIB_DIRS` this way. Captures of outside constants are dropped, as a constant is read from the engine where it runs: a called command passes its captures up to the caller, so calling a shell command whose body reads a module-private constant would otherwise need that constant. Any other outside variable makes the pickler refuse |
| `ModuleId` | Only 0, the root module of the default overlay. Anything else makes the pickler refuse. `pickle` parses into the default overlay, because a pickle runs as a script, so an overlay active in the shell that ran `pickle` doesn't count |
| `BlockId` | Never allowed as an import. Blocks are reached through decls |
| `FileId` and spans | Never imports. The program is parsed standalone (`StateWorkingSet::standalone`): every file it reads is added again instead of reusing the shell's copy, so the blocks and modules parsed from it are created again too, and the pickle carries all of them, std library modules included. Loading adds its files the same way. A span into a file outside the program, which only an expanded shell alias leaves behind, becomes `Span::unknown()` |
| `SpanId` | Not carried: every span id becomes `UNKNOWN_SPAN_ID`, and an expression without a span id reports its own `span`, which is relocated like any span. So values the loader computes again keep their spans. Only IDE features read the span id table otherwise |

The pickler also refuses when the delta holds:

- virtual paths
- a decl that is not a custom command, `extern` or alias. That covers plugin
  decls from `plugin use` and leftover predeclarations.

For plugins, "exists and matches" loads. "Missing" and "signature changed"
are refused at load time, with a message naming the command. "Exists but
fails to start" behaves as it would for source: the error comes when the
command is called. The plugin's identity is not part of the shape, because
`plugin_identity()` only exists with the `plugin` feature. A different plugin
that provides a command with the same name and shape would be accepted.

### File format

```text
+----------------------------------------------+
| magic     "\0nu-pickle\0"                    |
| format    u32, little endian                 |
| header    postcard: nushell version, git     |
|           commit, every experimental option  |
|           and whether it was enabled         |
| checksum  u64, little endian: FNV-1a over    |
|           the linkage and the body           |
| linkage   postcard: producer bases,          |
|           decl and var imports               |
| body      LZ4 block (size-prefixed) of       |
|           postcard, relocated: files, vars,  |
|           parse-time values, decls, blocks,  |
|           modules, scope, top-level block    |
+----------------------------------------------+
```

- The codec is `postcard` (MIT or Apache-2.0, pure Rust). It writes integers
  as varints and enum variants as their index, and nothing else about the
  types: no field names and no variant names, so it is compact and decodes
  without string comparisons. `rmp-serde`, which the workspace already has,
  writes enum variants by name, and the IR is mostly enums.
- The format version sits at a fixed position, so any later layout can still
  be recognized and refused cleanly.
- Neither postcard nor an LZ4 block notices damage, and a damaged id can point
  anywhere and panic the loader or the evaluator. The checksum turns damage
  into an error before anything is decoded. It hashes eight bytes at a time,
  which takes 0.15 ms for the linkage and body of `synthetic.nu`. A crafted
  pickle can carry a matching checksum, but it is code, trusted like a
  script.
- Postcard has no recursion limit, so the body is decoded through a reader
  that fails once decoding has used 1 MiB of stack. Every level of nesting
  reads at least a byte, so it checks on every byte read. That allows about
  9,000 levels of `list<...>` in a release build and 760 in a debug build,
  far more than a program nests, and makes decoding `synthetic.nu` about
  0.55 ms (5%) slower. A crafted pickle nested deeper fails to load instead
  of overflowing the stack.
- The compatibility rule is deliberately conservative. The loader rejects the
  pickle unless it matches on:
  - format version
  - Nushell version
  - the git commit of the build (`NU_COMMIT_HASH`, which nu-parser's
    `build.rs` gets from git the way nu-cmd-lang's does for `version`'s
    `commit_hash`)

  A refused pickle gets one line per difference, for example
  `expected version 0.116.0 and got 0.115.1` or
  `expected commit 2459fdd… and got 1a4cc5a…`. `debug pickle` shows the
  same list.
- Experimental options can change what the parser and compiler emit, so the
  header records every option and whether it was enabled, and a pickle runs
  with those instead of being refused over them.
  - `nu` reads the header at startup, right after it sets the options from
    `NU_EXPERIMENTAL_OPTIONS` and `--experimental-options`, and sets every
    option to the pickle's value. This happens before the engine is built,
    because some options register commands. It only looks at a regular file, found the way
    `evaluate_file` finds the script, so a script read from a pipe or FIFO
    doesn't lose its first bytes.
  - An explicit `--experimental-options` is overridden with a warning. The
    environment variable is overridden silently, since it applies to every
    run.
  - So a pickle made in a REPL where `NU_EXPERIMENTAL_OPTIONS` enables
    options runs with them as `nu script.nupkl`, although `nu` ignores the
    variable when it runs a script without config.
  - `load` doesn't compare options, because `nu` has set them by the time it
    runs. `debug pickle` links into the current shell's engine with the
    current shell's options. A pickle that calls a command only one of the
    two option sets registers (a `native-clip` command, say) shows a missing
    command there and still runs as `nu script.nupkl`.

  Postcard can't tell whether the types it decodes are the ones it encoded,
  so the commit is what protects against a pickle from another build being
  misread. The commit doesn't cover uncommitted local edits to the
  serialized types; a build without git (for example from crates.io) has an
  empty commit and is checked by version only. There is no cross-version
  support.
- A pickle is decoded once right after it is written, and `save` fails if
  that doesn't work. Postcard can't decode every type serde can describe
  (plugin custom values need a self-describing format), and this check
  catches that when the pickle is made, not when it is run.
- The body is LZ4-compressed with `lz4_flex` (pure Rust, MIT), which makes
  it 2 to 3 times smaller. Loading is the operation that has to be fast, and
  LZ4 decompresses about 7 times faster than brotli (see "Compression"
  below).
  The size prefix is checked against LZ4's maximum expansion before
  allocating, so a damaged file can't ask for gigabytes.
- Sources are embedded. The runtime reads span contents for error labels,
  `view source` and `metadata`. Embedding also makes local span relocation a
  constant offset.
- Decls are not serializable (`Box<dyn Command>`). The pickle stores an enum
  instead:
  - `Custom` stores its signature and rebuilds with
    `Signature::into_block_command`. (It usually differs from its block's
    signature: the parser adds `any -> any` to the block's.)
  - `Extern` rebuilds `KnownExternal`.
  - `Alias` rebuilds `Alias`, taking `command` from the relinked target with
    `clone_box`. Aliases can't see later definitions, so one pass in id order
    works.
- Blocks don't keep their AST (`pipelines`), except the ones a parse-time
  expression reaches (see "Portability"):
  - Evaluation runs IR. The AST of a compiled block is only read by
    `explain`.
  - Blocks without IR (subexpressions, keyword bodies) are inlined into
    their parent's IR and never run on their own.
  - The AST evaluator could only reach them through `IrAstRef` arguments,
    and no command asks for those. nu-command's
    `requires_ast_for_arguments_inventory` test keeps it that way.

  IR, signatures, captures and parser-info expressions are all kept.

### Portability

A pickle runs on another machine or from another directory without the
script or any file it `use`s or `source`s. Two things make that work.

**Paths are stored relative to the script's directory** (with `/`
separators) and resolved against the pickle's directory when it's loaded. So
the layout next to the pickle mirrors the layout next to the script. Three
places hold paths:

| place | rule |
| ----- | ---- |
| file table names | Stored with a `relative` flag, because names like `program.nu` or `<commandline>` aren't paths |
| `Module.file` (`ParserPath::RealPath`) | The parser only records absolute paths, so a relative one at load time is the pickle's |
| the `block_id_name` parser info of `source`, `source-env` and `run` | Same rule, for the files the program carries. `run --full-reparse` reads its file again, so a file only it names keeps the path the parser found |

A path that can't be made relative (another drive on Windows) stays absolute.
These paths show up in error messages, in `$env.FILE_PWD` and
`$env.CURRENT_FILE`, and in `scope modules`.

**Commands that run a file the parser compiled take its path from the
parser** instead of looking for the file again when they run:

- **`source`, `source-env` and `run`** use the `block_id_name` path for
  `FILE_PWD` and `CURRENT_FILE`, for parsed scripts and pickles alike.
  - `source` and `run` don't check that the file still exists, and
    `source-env` doesn't search the library dirs again. So a sourced file
    that disappears between parsing and running no longer fails, and
    `source std/util/mod.nu` works; that check failed it, since a standard
    library file only exists inside nushell.
  - `run --full-reparse` reads its file again, and fails if it's gone.
- **`use`, `export use` and `overlay use`** set `FILE_PWD` and
  `CURRENT_FILE` for `export-env` from the file the module was parsed from
  (`Module::real_file`), instead of searching for the module's name. The
  search could find the wrong file: `use $path` with a `const` path looked
  for the text `$path`, and a module defined inline picked up any file or
  directory in the current directory with its name.

**Values the parser computed are computed again when the pickle loads**, on
the machine that runs it, as parsing the source there would:

- A standalone parse records every value the parser computes and how it
  computed it, in `StateWorkingSet::parse_time_values`. Other parses record
  nothing, so they pay nothing for it.
  - `const` values and parameter defaults, as the expression it evaluated,
    and the type a `const` declares
  - the `$module` records of `use` and `overlay use`, as the module they
    come from
  - Not the throwaway signature parse of a def's predeclaration, which runs
    before the constants the defaults may use
- A `const` with a declared type is checked against it again, like parsing
  checks it.
- The pickle stores that list, and the AST of any block those expressions
  reach, since the constant evaluator walks the AST of `(...)`.
- The loader adds the program's blocks, then evaluates the list in order:
  - constants update their variable
  - defaults are written into the signatures before the commands are built
    from them
- `path self` reads the file being parsed, so each expression runs with its
  own file, resolved next to the pickle, pushed on `working_set.files`.
- A value that fails to compute here is a parse error, reported and exiting
  like one, where the `.nu` script would fail. That branch has no
  automated test: every machine-dependent constant input tried (a missing
  or unset `HOME`, for one) still computes.

So `const home = $nu.home-dir`, `def main [--dir = ($nu.home-dir | path join
x)]`, `const here = path self` and `use lib/cfg.nu; $cfg.DIR` all give the
running machine's values; even `--help` shows its defaults.

What stays as it was when the pickle was made:

- **Which files `use` and `source` resolved to, and their contents.** A pickle
  is a bundle, like a statically linked binary. The files a pickle carries
  are what it runs, even if the machine that runs it has other ones.
- **Constant `match` patterns** (`match $x { $CONST => … }`). The parser folds
  them into the IR's `match` instructions, where nothing records where they
  came from.
- **Attribute, example and completion values.** They're documentation, not
  behavior.
- **What the parser inferred from a constant's type.** If the type changes
  between machines (a constant that is an int on one OS and a string on
  another), the parser's type checks ran on the type it had when the pickle
  was made.

### Engine integration

- `pickle script.nu [--output <path>]` is a command. It parses and compiles
  `script.nu` in the shell it runs in, the way `source` would, with the
  script on the file stack so `use`, `source` and `path self` start from its
  directory. It writes `script.nupkl` next to the script unless `--output`
  says otherwise, and returns the path, the size and `warnings`.
  - It parses standalone, so what the shell loaded already (the script
    itself after a `source`, or a module the config `use`s) is parsed again,
    into the pickle, instead of being taken from the shell.
  - It never merges what it parsed, so the shell is unchanged afterwards.
  - Parse warnings and errors are reported like `nu script.nu` reports
    them, with the source snippet.
  - It replaces an existing pickle but keeps any other file, which also
    protects the script from its own pickle.
- Parsing in the current shell differs from `nu script.nu`, which has no
  config. What that means for a pickle:
  - Commands and constants from the shell's config are imports, linked by
    name like builtins. A nushell without them refuses to run the pickle,
    and the error says the shell that made the pickle defined them.
  - Aliases are expanded by the parser, so they leave no import behind. A
    pickle made in a shell with `alias ls = ...` runs the alias, which
    `nu script.nu` wouldn't. `pickle` can't prevent that, so it names every
    alias of the shell the script used in `warnings`. It finds them in the
    AST: an expanded alias keeps the alias's name as the call's head and
    calls what the alias wraps.
  - An active overlay doesn't matter (see the `ModuleId` row above).
  - The shell's experimental options are recorded, and the pickle runs
    with them (see "File format").
- `nu <file>` checks the magic bytes, whatever the file is called. For a
  pickle it loads instead of parsing, then follows the exact code path a
  parsed script takes: the `main` rename, `merge_delta`, `eval_block`, and the
  `main <args>` call.
- `FILE_PWD`, `CURRENT_FILE` and the `main` rename refer to the pickle file
  that was run, so `nu script.nupkl --help` shows `script.nupkl`.
- `depickle <pickle> [--output-dir <dir>] [--force]` writes the source files
  a pickle carries back out, with their paths relative to the script's
  directory, and returns what it wrote.
  - A `..` in a path becomes `_up`, and roots and drive prefixes are dropped,
    so a crafted pickle can't write outside the output directory.
  - It refuses to overwrite files unless given `--force`, and to write two
    files to one path (a `..` and a real `_up`).
- `debug pickle <pickle>` returns a record describing the pickle:
  - always: its format, version, commit and experimental options, plus
    `loads` and the exact `problems`
  - when it loads: it's linked into a copy of the current engine, and the
    record adds its files, commands, variables (with the values computed
    here), blocks (with their IR as `view ir` prints it), modules, and the
    commands it uses from this nushell

The loader lives in nu-parser (`crates/nu-parser/src/pickle.rs`) next to
`parse`, because it is the other way to produce a delta. It also owns
`KnownExternal`. The commands are in nu-command's `src/debug/`, one file each.
There is no `nu` flag: `pickle` works from the REPL, a script, or
`nu -c 'pickle script.nu'`.
- `source`, `source-env`, `use`, `export use` and `overlay use` of a pickle
  fail with "Can't read a pickle as source code" instead of parse errors
  about its bytes.

### Semantics that change on purpose

- Which files `use` and `source` resolved to, and their contents, are fixed
  when the pickle is made. Parse-time values are not (see "Portability").
- Parse warnings are reported when the pickle is made, not when it runs.
- Aliases of the shell that ran `pickle` are compiled in (see "Engine
  integration").
- Commands the script calls are bound at load time, as they would be for
  source. A builtin whose implementation changed but whose shape did not
  runs the new implementation.
- `explain` on a closure from a pickle returns nothing, because the AST is
  not stored.

## Part 3. Prototype status

### What exists

| piece | where |
| ----- | ----- |
| relocation hooks and `IdBases`, `ImportCollector`, `IdRelocator` | `crates/nu-protocol/src/relocation.rs`, `id.rs`, `span.rs` |
| `NU_COMMIT_HASH` | `crates/nu-parser/build.rs` |
| serde for `Variable`, `Module`, `ParserPath`, `ScopeFrame`, `OverlayFrame`, `DeclNameMap`, `Visibility`, `CustomExample` | nu-protocol |
| `pickle::{save, load, is_pickle, info, sources}` | `crates/nu-parser/src/pickle.rs` |
| `StateWorkingSet::standalone` and `parse_time_values`, recorded by the parser | nu-protocol, nu-parser (`parse_bindings`, `parse_signatures`, `parse_module`, `parse_def`) |
| `Expression::span` falls back to the expression's `span` without a span id | `crates/nu-protocol/src/ast/expression.rs` |
| `Module::constants_record` | `crates/nu-protocol/src/module.rs` |
| a pickle's experimental options set at startup | `src/experimental_options.rs` |
| `pickle`, `depickle`, `debug pickle` | `crates/nu-command/src/debug/pickle.rs`, `depickle.rs`, `debug_pickle.rs`, with `read_pickle` in `util.rs` |
| `nu script.nupkl` loads instead of parsing | `crates/nu-cli/src/eval_file.rs` |
| `source`, `source-env` and `run` use the parser's path | nu-command |
| `Module::real_file` sets `FILE_PWD` and `CURRENT_FILE` | `use`, `export use` and `overlay use` in nu-cmd-lang |
| the error for `source` or `use` of a pickle | `sourced_pickle_error` in `pickle.rs`, called from `parse_source.rs` and `parse_module.rs` |
| unit test: a pickle nested too deeply | `crates/nu-parser/src/pickle.rs` |
| engine-level tests: shifted ids, stale command, a command linked by the name the program called it, changed signature, damaged file | `crates/nu-cli/tests/pickle.rs` |
| binary-level tests: sources deleted, pickle moved to another directory (with the paths modules and sourced files see), parse-time values from the running process, a shell constant shadowing the script's, pickling what the shell loaded already, running under a config that loaded the same files, `--help`, `--output`, replacing only pickles, missing script, a script read from a pipe, `source` or `use` of a pickle, alias warnings, an active overlay, running with the pickle's experimental options, `depickle`, `debug pickle` and an exact mismatch message | `tests/shell/pickle.rs` |

These scripts ran the same from source and from pickle:

- defs with flags and rest parameters, closures with captures, `each`,
  `where`, `match` with bindings, `try`/`catch`, `for` over a `const` range,
  aliases, externals (`^echo`)
- `use std/assert`, a file module with `export-env`, `export const` and
  subcommands, `use ... [names]`, `source`, `overlay use --prefix`
- `def main` with flags, `--help` on the script and `ls --help` (the relinked
  `help_decl_id`)
- runtime errors from inside a def and from inside `std/assert`, with the
  same source snippet and location

They also ran the same across every combination of producer and consumer
started with `-n`, `--no-std-lib -n`, `--no-std-lib` and the defaults, which
move blocks, files, variables and prelude decls. The engine-level test also
moves every builtin decl id. When the relocator is disabled, that test fails.

### Measurements

Release build on the author's Mac, `hyperfine`, `nu -n`, mean of 60 runs (100
for `stdheavy.nu`). Startup alone (`nu -n -c 'print done'`) takes 11.5 ms.

- `synthetic.nu` is 1000 generated defs with closures, `if` and `match`,
  5000 lines.
- `stdheavy.nu` is `use` of 15 std modules. The work is parsing those
  modules.
- `small.nu` is `def main [...args: string] { $args | to json }`.

| script | source | parse + run | pickle + run | pickle size | making the pickle |
| ------ | ------ | ----------- | ------------ | ----------- | ----------------- |
| `synthetic.nu` | 295 KB | 85.4 ms | 29.0 ms | 784 KB (2.7x) | 138 ms |
| `stdheavy.nu` | 216 B | 25.5 ms | 14.7 ms | 177 KB | 39 ms |
| `small.nu` | 46 B | 11.7 ms | 11.7 ms | 517 B (11x) | 15 ms |

"Making the pickle" is `nu -n -c 'pickle script.nu'`, startup included.
`stdheavy.nu`'s pickle carries the sources of the 15 std modules.

After subtracting startup, the pickle does 76% less work than the source for
`synthetic.nu` (17.5 ms against 73.9 ms) and 77% less for `stdheavy.nu` (3.2
ms against 14.0 ms).

The phases of `load`, as the median of 31 runs with temporary timing code:

| phase | `synthetic.nu` | `stdheavy.nu` |
| ----- | -------------- | ------------- |
| check the header and checksum, link imports | 0.19 ms | 0.29 ms |
| LZ4 decompression | 1.18 ms | 0.33 ms |
| postcard decode with relocation | 11.5 ms | 1.76 ms |
| relocate parser-info ids and file paths | 0.21 ms | 0.03 ms |
| add files, variables, blocks and modules | 0.71 ms | 0.12 ms |
| compute parse-time values again | none to compute | 0.14 ms |
| build decls and scope | 0.21 ms | 0.04 ms |
| total | 14.0 ms | 2.7 ms |

Decoding is 82% of the synthetic load. It is roughly one allocation per AST
node, instruction and value, plus the relocation hooks on every id and span
and the stack check on every byte (see "File format"). The other 3.5 ms of
the synthetic pickle's run are `merge_delta`, the `main` rename and running
the script.

About 180 of `small.nu`'s 517 bytes are fixed:

- 11 bytes of magic
- 4 bytes of format version
- the version string
- the 40-character commit hash
- every experimental option and whether it's enabled, 107 bytes for the
  current 7 options
- 8 bytes of checksum

The rest is the embedded source, IR, signatures, variables, scope frames and
the linkage. Large scripts land under 3 times their source. Tiny scripts
can't, because the fixed part alone is larger than they are.

### Compression

Each codec compressed the uncompressed bodies of the pickles above (2.42 MB
for `synthetic.nu`, 396 KB for `stdheavy.nu`). Decompression is the median of
21 runs in a standalone release benchmark. Only pure-Rust codecs were
considered.

| codec | synthetic size | decompress | stdheavy size | decompress | compress (synthetic) |
| ----- | -------------- | ---------- | ------------- | ---------- | -------------------- |
| `lz4_flex` | 783 KB (3.1x) | 0.65 ms | 174 KB (2.3x) | 0.19 ms | 2.6 ms |
| brotli q3, window 20 (`to msgpackz`) | 480 KB (5.0x) | 4.8 ms | 114 KB | 1.29 ms | 14 ms |
| brotli q5, window 22 | 402 KB (6.0x) | 4.6 ms | 99 KB | 1.30 ms | 43 ms |
| brotli q9, window 22 | 396 KB (6.1x) | 4.6 ms | 97 KB | 1.27 ms | 98 ms |
| brotli q11, window 22 | 318 KB (7.6x) | 5.5 ms | 86 KB | 1.53 ms | 3.0 s |
| deflate (`miniz_oxide`, level 6) | 536 KB (4.5x) | 3.4 ms | 125 KB | 1.08 ms | 50 ms |
| `ruzstd` (fastest, its only level) | 685 KB (3.5x) | 6.5 ms | 172 KB | 1.36 ms | 18 ms |

- LZ4 decompresses about 7 times faster than brotli at any quality. The
  benchmark repeats one decompression in a loop; inside `load`, a single
  decompression of the same body took 1.2 ms (see "Measurements").
- Brotli at quality 9 would halve the files (396 KB instead of 783 KB) for
  about 4 ms more per load of `synthetic.nu` and 1.1 ms more for
  `stdheavy.nu`. If size matters more than load time, it's a one-line change.
- `to msgpackz`'s setting gives the largest brotli output and doesn't
  decompress any faster.
- The pure-Rust zstd decoder is larger and slower than brotli.

NUON is not a good fit for pickles:

- `from nuon` runs the Nushell parser (`nuon::from_nuon` calls
  `nu_parser::parse`), and skipping the parser is the whole point.
- On 537 rows of `scope commands` (name, category, description, search terms
  and signatures), `from nuon` took 43.8 ms where `from msgpack` took 3.2 ms,
  as the median of 21 `timeit` runs.
- The pickle holds Rust types (IR instructions, AST expressions, signatures).
  NUON represents `Value`s, and there is no serde bridge to `Value`.
- The `nuon` crate depends on nu-parser, so nu-parser can't use it.

### Findings from the implementation

- **Constants are imports too.** `use` and `source` put `$NU_LIB_DIRS` into
  the call's `dirs_var` parser info, so every script that imports anything
  references a variable created at startup with a non-fixed id. It has to be
  relinked by name.
- **Several commands looked at the file system at run time for files they
  compiled at parse time.**
  - `source` and `run` checked that their file still existed. The check
    dated from #17319, which replaced `canonicalize` with `absolute_with`
    plus `try_exists` and so kept `canonicalize`'s failure on missing files.
  - `source-env` searched the library dirs again.
  - `use`, `export use` and `overlay use` searched again to set `FILE_PWD`
    for `export-env`.

  None of them reads the file. They only need its path, so they now take the
  one the parser found (see "Portability").
- **`main` renaming is shared.** `evaluate_file` renames `main` after parsing.
  The pickle is saved before that rename and the rename runs again after
  loading, so both paths stay one path.
- **The parser reuses what the engine loaded.** `add_file` hands out the
  engine's file when one has the same name and contents, and the parser then
  reuses the blocks and modules it parsed from that file, because it finds
  them by span. So a script the shell had `source`d pickled into an empty
  program, and a module the config `use`d was linked instead of carried. A
  standalone working set only reuses its own files. `pickle` parses with one,
  and `load` adds the pickle's files with one, so a config that loads a file
  the pickle carries doesn't get in the way either.

### Known gaps

- **`scope` doesn't list what a block defines, inside a pickled block.** The
  program runs the same; only what it reports about its own scope differs.
  - When the parser leaves a nested block, it drops the block's table of
    local names. Running doesn't need it, because every call was resolved to
    a decl id during parsing. So that `scope` can still list those names at
    run time, the parser keeps a copy in two places:
    - `Block::scope_bindings`, for closures, custom command bodies and `do`
      blocks. It is pushed on the stack while the block runs.
    - `IrBlock::scope_regions`, for the bodies of `if`, `for` and the other
      keywords. They are inlined into the parent's IR, so their names are
      matched by instruction index.
  - Both fields are `#[serde(skip)]`, on main too, so a pickle doesn't carry
    them.
  - So inside a pickled block, `scope commands`, `scope aliases`,
    `scope externs`, `scope modules`, `help aliases`, `help externs` and
    `help modules` don't list the commands, aliases, externs and modules that
    block defines. Top-level definitions and `scope variables` are listed as
    from source, and calling a local command works.
  - Only a script that inspects its own scope from inside a block notices,
    for example one that defines `foo` in a block and then checks
    `scope commands | where name == foo`.
- The untyped parser-info ids nested inside `IrAstRef` expressions are not
  relocated (see "Relocation through serde").
- Pickles can only be run as the script of `nu`, not sourced into a live
  REPL.
- The commit check can't see uncommitted edits to the serialized types, so
  a pickle made before such an edit could be misread by a build made after
  it.
- The commit can also be stale. nu-parser's `build.rs` emits no
  `rerun-if-changed`, so cargo only reruns it when a file in nu-parser
  changes. A checkout that only touches other crates keeps the old
  `NU_COMMIT_HASH`. `version`, whose hash comes from nu-cmd-lang's
  `build.rs`, has the same problem.
- A path that couldn't be made relative stays absolute. That can only happen
  on Windows, for a file on another drive. If that pickle is loaded on Unix,
  the Windows path looks relative and gets rebased onto the pickle's
  directory. Only error messages and `FILE_PWD` are affected.
- `explain` on pickled closures returns nothing.
- Span ids are not carried, so IDE features would see unknown spans. None
  runs on pickled code today.
- Cell paths serialize as their text (#18434), which drops the spans of their
  members. An error about a member of a cell path in a pickled program
  points at an unknown span. Reading the text back also has two typos in
  `cell_path.rs` (`"Ox"` for `"0x"`, `'A'..='Z'` for `'A'..='F'`) that turn
  a member named like `OxA` into a number. Both predate pickles and affect
  plugins too; fixing them changes the plugin protocol's msgpack encoding,
  so they're left for their own change.
- A module parsed in a shell that has its commands already (a config with
  `use std *`, say) links them from the module's `@example` code, which
  calls them by their imported names. The examples never run, but the
  pickle then needs those commands where it runs. `nu -n -c 'pickle
  script.nu'` avoids it.
- The shell's overlays stay visible while `pickle` parses, like its commands.
  So `overlay hide --keep-custom` of an overlay with the name of one active
  in that shell keeps that overlay's commands too.
- Constant `match` patterns keep the value they had when the pickle was made
  (see "Portability").
- Two pickles of the same script aren't byte-for-byte equal. The scope
  frames keep variables, commands and modules by name in `HashMap`s, and
  serde writes those in iteration order, which changes between parses. They
  load the same. A cache keyed on the pickle's bytes would need a sorted
  order. Pickles made in different shells also differ in their recorded
  table sizes, which is expected.

### Next steps, roughly by value

1. **Load time.** Decoding is 82% of loading a large pickle (see
   "Measurements"). Every node is its own allocation, and the relocation
   hooks borrow a `RefCell` in a thread-local on every id and span; a slot
   without the borrow needs `unsafe`. `DeclShape::of` rebuilds and encodes
   the signature of every imported command, about 7% of loading
   `stdheavy.nu`.
2. **Carry `scope_bindings`/`scope_regions`** (see "Known gaps"). This needs
   serde derives for `ScopeBindings` and `ScopeRegion` and dropping the two
   `serde(skip)`s; the relocation hooks already handle the decl and module
   ids in them. Two things change with it:
   - A block that `use`s a module of the pickling shell (one its config
     defines, say) binds that module's commands locally, which makes them
     imports. The pickler refuses such a script when one of those commands
     has no name in the shell, where it accepts the script today.
   - `view ir --json` prints the scope regions too.
3. **Cache policy.** `script.nu` next to `script.nupkl`, invalidated by
   source hashes (the pickle already embeds every file with its name) plus
   the version and commit in the header.
4. **Loading into a live REPL.** This needs variable and module imports by
   name, and merging the pickle's overlay frame instead of replacing it.
