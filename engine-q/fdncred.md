```rust
let x = [
    [name value];
    [foo 123]
    [bar 456]
    [baz 789]
    [qux 111]
]
```

> $x | nth 0  
returns a 1 row table   
this is nushell syntax - not in e-q yet as of 11/23/2021

```rust
╭───┬──────┬───────╮
│ # │ name │ value │
├───┼──────┼───────┤
│ 0 │ foo  │   123 │
╰───┴──────┴───────╯
```

> $x | get 0  
returns a record

```rust
╭───────┬─────╮
│ name  │ foo │
│ value │ 123 │
╰───────┴─────╯
```

> $x | get 0 | describe   
record<name: string, value: int>

> $x | select value  

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

> $x | select value | describe

```rust
╭───┬────────────────────╮
│ 0 │ record<value: int> │
│ 1 │ record<value: int> │
│ 2 │ record<value: int> │
│ 3 │ record<value: int> │
╰───┴────────────────────╯
```

> $x | get Value  
returns a list  

```rust
╭───┬─────╮
│ 0 │ 123 │
│ 1 │ 456 │
│ 2 │ 789 │
│ 3 │ 111 │
╰───┴─────╯
```

> $x | get value | describe  

```rust
╭───┬─────╮
│ 0 │ int │
│ 1 │ int │
│ 2 │ int │
│ 3 │ int │
╰───┴─────╯
```
###### summary

* get returns a list `ls | get 0` or `ls | get name`
* `select` returns a table with 1 column, which is a list of records
* `nth` returns a 1 record table? or a 1 record list? - not in e-q yet
* `nth 0` and `select value` have row/column headers - `get 0` or `get value` does not
