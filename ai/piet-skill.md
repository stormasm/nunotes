---
name: nushell-test-refactor
description: Refactor Nushell integration tests from deprecated nu_test_support::nu! usage to nu_test_support::test() with the right prelude imports and migration patterns
license: MIT
compatibility: opencode
metadata:
  audience: nushell-contributors
  workflow: refactoring
  language: rust
---

# Skill: nushell-test-refactor

Refactor Nushell integration tests away from deprecated `nu_test_support::nu!` calls and toward `nu_test_support::test()` usage.

## When to use this skill

Use this skill when you are updating Nushell integration tests that still use `nu_test_support::nu!`, especially when:

- tests are being migrated to the newer test harness
- imports need to be normalized around `nu_test_support::prelude::*`
- many files need the same refactor pattern
- you have example refactors, a git diff, or known-good migrated files to follow

Do not use this skill for unrelated Rust refactors or for unit tests that do not depend on the Nushell test support helpers.

## Core migration rule

Prefer `nu_test_support::test()` over `nu_test_support::nu!` for integration tests.

Use `nu_test_support::prelude::*` as the starting import when possible, since it includes most commonly needed test support items.

For API behavior and exact call shapes, inspect:

- `crates/nu-test-support/src/tester/mod.rs`
- these known successful refactorings:
  - `crates/nu-command/tests/format_conversions/*.rs`
  - `crates/nu-command/tests/commands/network/*.rs`
- any user-provided already-refactored files
- any user-provided git diff that shows accepted migration patterns

## What to inspect first

Before editing, inspect the target test file for:

1. current imports from `nu_test_support`
2. all `nu!` macro usages
3. assertions on stdout, stderr, exit code, or filesystem side effects
4. helper functions or local wrappers around `nu!`
5. per-file conventions already used nearby

Then inspect the canonical implementation in:

- `crates/nu-test-support/src/tester/mod.rs`

Treat that file as the main API reference for `test()` behavior.

## Refactor workflow

### 1. Normalize imports

Start by replacing narrow or older `nu_test_support` imports with:

```rust
use nu_test_support::prelude::*;
```

Keep any extra imports that are still needed and not covered by the prelude.

Remove imports that become unused after the migration.

### 2. Replace `nu!` call sites

Convert each `nu!` integration test invocation into the equivalent `test()` form.

Preserve the original test intent exactly:

* same pipeline or command under test
* same setup and fixtures
* same output assertions but type updates are allowed (a `"true"` may be replaced with a test for `true`)
* errors should be matched by types directly, check `crates\nu-protocol\src\errors\shell_error\mod.rs` for the `ShellError` types

Do not rewrite test semantics just to make the migration easier.

### 3. Reuse proven patterns

If the user provides known-good migrated files or a git diff, treat those as the preferred style guide for:

* import shape
* `test()` call structure
* assertion style
* usage of `.expect_value_eq()`
* usage of `.expect_*_error()`
* fixture setup
* naming conventions

Follow local consistency over inventing a new migration style.

## Rules for safe migrations

* Do not guess the `test()` API from memory when the implementation file can be inspected.
* Do not mix old and new styles in the same file unless there is a clear reason.
* Do not silently weaken assertions.
* Do not replace exact assertions with vague substring checks unless the original test already allowed that.
* Do not remove setup code, temp directories, or fixtures without proving they are unnecessary.
* Do not rewrite unrelated test helpers unless the migration requires it.

## How to use example refactors

If the user provides already-refactored files:

1. compare the old and new structure
2. identify repeated migration patterns
3. apply those patterns consistently to the new target files
4. call out any file that does not fit the established pattern

If the user provides a git diff:

1. extract the common edit pattern
2. mirror that pattern in untouched files
3. stay consistent with naming, imports, and assertions from the diff

## Expected output when using this skill

When performing the refactor, produce:

1. a short summary of the migration pattern used
2. the edited files
3. any places where the `test()` mapping was unclear
4. any follow-up checks the user should run

If a call site is ambiguous, explain the ambiguity briefly and base the edit on the closest confirmed local example.

## Good working style

* inspect first, then edit
* prefer local examples over generic assumptions
* keep diffs small and mechanical
* explain any non-mechanical change
* flag uncertainty instead of faking confidence

## Checklist

For each migrated file, verify:

* `nu!` usage was removed or intentionally left in place
* `use nu_test_support::prelude::*;` is present when appropriate
* unused imports were removed
* assertions still test the same behavior
* the file matches nearby migrated test style
* the `test()` shape aligns with `crates/nu-test-support/src/tester/mod.rs`

## Notes for future improvement

This skill becomes stronger when paired with reference material such as:

* a list of successfully refactored files
* a representative git diff
* examples of tricky migrations like error assertions, env setup, or filesystem fixtures

When those are available, prefer them as the migration baseline.
