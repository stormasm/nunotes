
```rust

test reject::path_274_tests_fixtures_reject_variables_double_question_nu ... ok

failures:

---- accept::path_414_tests_fixtures_accept_pipelines_crlf_line_endings_nu stdout ----

thread 'accept::path_414_tests_fixtures_accept_pipelines_crlf_line_endings_nu' (12931033) panicked at tests/fixtures.rs:54:9:
/Users/ma/j/tmp17/nu_winnow/tests/fixtures/accept/pipelines/crlf-line-endings.ast differs from the parser's output.
--- expected
Block 0..29
  Pipeline 0..12
    Call `ls` 0..2
    | 4..5
    Call `length` 6..12
  Pipeline 14..23
    let x 14..23
      Value 22..23
        Pipeline 22..23
          Int 1 22..23
  Pipeline 25..27
    Var $x 25..27

--- actual
Block 0..25
  Pipeline 0..11
    Call `ls` 0..2
    | 3..4
    Call `length` 5..11
  Pipeline 12..21
    let x 12..21
      Value 20..21
        Pipeline 20..21
          Int 1 20..21
  Pipeline 22..24
    Var $x 22..24

Run `UPDATE_FIXTURES=1 cargo test --test fixtures` if the change is intended.
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    accept::path_414_tests_fixtures_accept_pipelines_crlf_line_endings_nu

test result: FAILED. 849 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.34s

error: test failed, to rerun pass `--test fixtures`
     Running tests/language.rs (target/debug/deps/language-e3a5e075decaa4aa)

running 364 tests
```rust