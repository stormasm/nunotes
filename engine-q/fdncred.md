```rust
let x = [
    [name value];
    [foo 123]
    [bar 456]
    [baz 789]
    [qux 111]
]
```

`> $x | nth 0`  
returns a 1 row table   
this is nushell syntax - not in e-q yet as of 11/23/2021

```rust
╭───┬──────┬───────╮
│ # │ name │ value │
├───┼──────┼───────┤
│ 0 │ foo  │   123 │
╰───┴──────┴───────╯
```

`> $x | get 0`  
returns a record

```rust
╭───────┬─────╮
│ name  │ foo │
│ value │ 123 │
╰───────┴─────╯
```

`> $x | get 0 | describe`   
record<name: string, value: int>

`> $x | select value`  
returns a 1 column table  

```rust
╭───┬───────╮
│ # │ value │
├───┼───────┤
│ 0 │   123 │
│ 1 │   456 │
│ 2 │   789 │
│ 3 │   111 │
╰───┴───────╯
```

`> $x | select value | describe`

```rust
╭───┬────────────────────╮
│ 0 │ record<value: int> │
│ 1 │ record<value: int> │
│ 2 │ record<value: int> │
│ 3 │ record<value: int> │
╰───┴────────────────────╯
```

`> $x | get value`  
returns a list  

```rust
╭───┬─────╮
│ 0 │ 123 │
│ 1 │ 456 │
│ 2 │ 789 │
│ 3 │ 111 │
╰───┴─────╯
```

`> $x | get value | describe`  

```rust
╭───┬─────╮
│ 0 │ int │
│ 1 │ int │
│ 2 │ int │
│ 3 │ int │
╰───┴─────╯
```

`> $x | debug`

```
╭───┬─────────────────────────╮
│ 0 │ {name: foo, value: 123} │
│ 1 │ {name: bar, value: 456} │
│ 2 │ {name: baz, value: 789} │
│ 3 │ {name: qux, value: 111} │
╰───┴─────────────────────────╯
```

`> $x | debug --raw`

```
╭───┬──────────────────────────────╮
│ 0 │ Record {                     │
│   │     cols: [                  │
│   │         "name",              │
│   │         "value",             │
│   │     ],                       │
│   │     vals: [                  │
│   │         String {             │
│   │             val: "foo",      │
│   │             span: Span {     │
│   │                 start: 3183, │
│   │                 end: 3186,   │
│   │             },               │
│   │         },                   │
│   │         Int {                │
│   │             val: 123,        │
│   │             span: Span {     │
│   │                 start: 3187, │
│   │                 end: 3190,   │
│   │             },               │
│   │         },                   │
│   │     ],                       │
│   │     span: Span {             │
│   │         start: 3159,         │
│   │         end: 3234,           │
│   │     },                       │
│   │ }                            │
│ 1 │ Record {                     │
│   │     cols: [                  │
│   │         "name",              │
│   │         "value",             │
│   │     ],                       │
│   │     vals: [                  │
│   │         String {             │
│   │             val: "bar",      │
│   │             span: Span {     │
│   │                 start: 3197, │
│   │                 end: 3200,   │
│   │             },               │
│   │         },                   │
│   │         Int {                │
│   │             val: 456,        │
│   │             span: Span {     │
│   │                 start: 3201, │
│   │                 end: 3204,   │
│   │             },               │
│   │         },                   │
│   │     ],                       │
│   │     span: Span {             │
│   │         start: 3159,         │
│   │         end: 3234,           │
│   │     },                       │
│   │ }                            │
│ 2 │ Record {                     │
│   │     cols: [                  │
│   │         "name",              │
│   │         "value",             │
│   │     ],                       │
│   │     vals: [                  │
│   │         String {             │
│   │             val: "baz",      │
│   │             span: Span {     │
│   │                 start: 3211, │
│   │                 end: 3214,   │
│   │             },               │
│   │         },                   │
│   │         Int {                │
│   │             val: 789,        │
│   │             span: Span {     │
│   │                 start: 3215, │
│   │                 end: 3218,   │
│   │             },               │
│   │         },                   │
│   │     ],                       │
│   │     span: Span {             │
│   │         start: 3159,         │
│   │         end: 3234,           │
│   │     },                       │
│   │ }                            │
│ 3 │ Record {                     │
│   │     cols: [                  │
│   │         "name",              │
│   │         "value",             │
│   │     ],                       │
│   │     vals: [                  │
│   │         String {             │
│   │             val: "qux",      │
│   │             span: Span {     │
│   │                 start: 3225, │
│   │                 end: 3228,   │
│   │             },               │
│   │         },                   │
│   │         Int {                │
│   │             val: 111,        │
│   │             span: Span {     │
│   │                 start: 3229, │
│   │                 end: 3232,   │
│   │             },               │
│   │         },                   │
│   │     ],                       │
│   │     span: Span {             │
│   │         start: 3159,         │
│   │         end: 3234,           │
│   │     },                       │
│   │ }                            │
╰───┴──────────────────────────────╯
```

###### summary

* `ls | get 0` returns a record
* `ls | get name` returns a list
* `select` returns a table with 1 column, which is a list of records
* `nth` returns a 1 record table? or a 1 record list? - not in e-q yet
* `nth 0` and `select value` have row/column headers - `get 0` or `get value` does not
