

# Nushell

This is the command sort-by before **sort_by_cached_key** is called...

```rust

println!("{:?}",vec);
vec.sort_by_cached_key(calc_key);

[
Value { value: Row(Dictionary { entries: {"a": Value { value: Primitive(Int(1)), tag: Tag { anchor: None, span: Span { start: 31, end: 32 } } }, "b": Value { value: Primitive(Int(2)), tag: Tag { anchor: None, span: Span { start: 34, end: 35 } } }, "c": Value { value: Primitive(Int(3)), tag: Tag { anchor: None, span: Span { start: 37, end: 38 } } }} }), tag: Tag { anchor: None, span: Span { start: 18, end: 59 } } },
Value { value: Row(Dictionary { entries: {"a": Value { value: Primitive(Int(40)), tag: Tag { anchor: None, span: Span { start: 41, end: 43 } } }, "b": Value { value: Primitive(Int(50)), tag: Tag { anchor: None, span: Span { start: 44, end: 46 } } }, "c": Value { value: Primitive(Int(60)), tag: Tag { anchor: None, span: Span { start: 47, end: 49 } } }} }), tag: Tag { anchor: None, span: Span { start: 18, end: 59 } } },
Value { value: Row(Dictionary { entries: {"a": Value { value: Primitive(Int(4)), tag: Tag { anchor: None, span: Span { start: 52, end: 53 } } }, "b": Value { value: Primitive(Int(5)), tag: Tag { anchor: None, span: Span { start: 54, end: 55 } } }, "c": Value { value: Primitive(Int(6)), tag: Tag { anchor: None, span: Span { start: 56, end: 57 } } }} }), tag: Tag { anchor: None, span: Span { start: 18, end: 59 } } }
]
```

# Engine-q

This is the command select on age.

```rust
[
Record { cols: ["age"], vals: [Int { val: 1, span: Span { start: 6295, end: 6296 } }], span: Span { start: 6277, end: 6317 } },
Record { cols: ["age"], vals: [Int { val: 3, span: Span { start: 6301, end: 6302 } }], span: Span { start: 6277, end: 6317 } },
Record { cols: ["age"], vals: [Int { val: 2, span: Span { start: 6308, end: 6309 } }], span: Span { start: 6277, end: 6317 } }, Record { cols: ["age"], vals: [Int { val: 3, span: Span { start: 6314, end: 6315 } }], span: Span { start: 6277, end: 6317 } }
]
```
