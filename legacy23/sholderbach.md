
## The Nushell Record API

This is great work and something that should be clearly understood as it improves
the ease of use for Nushell developers moving forward.

Most of the documentation for the PRs are in the PR itself so this simply outlines
and supplements what @sholderbach and @ianmanske has already documented.

### Usage

To see the actual underlying Record data structures in Nushell

```rust
[[a b]; [1 2] [3 4]] | debug -r
```

### PRs in order

* [Create Record type](https://github.com/nushell/nushell/pull/10103)
* [Add common map-like API to nu_protocol::Record](https://github.com/nushell/nushell/pull/10841)

### History

Nushell 0.84 was the last release with the old Record structure...

before

```rust
pub enum Value {
  String {
       val: String,
       span: Span,
   },
   Record {
       cols: Vec<String>,
       vals: Vec<Value>,
       span: Span,
   },
   List {
       vals: Vec<Value>,
       span: Span,
   },
}
```

Nushell 0.85 has the new Record structure

```rust
pub enum Value {
  String {
      val: String,
      // note: spans are being refactored out of Value
      // please use .span() instead of matching this span value
      internal_span: Span,
  },
  Record {
      val: Record,
      // note: spans are being refactored out of Value
      // please use .span() instead of matching this span value
      internal_span: Span,
  },
  List {
      vals: Vec<Value>,
      // note: spans are being refactored out of Value
      // please use .span() instead of matching this span value
      internal_span: Span,
  },
}
```
