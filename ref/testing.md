
### cargo-nextest

* [doc](https://nexte.st/)
* [discord](https://discord.com/channels/601130461678272522/683070703716925568/942909162004828191)

```rust
let next1 = (cargo nextest list -p nu --message-format json | from json)
let next2 = (cargo nextest list -p nu --features=extra --message-format json | from json)

$next1
$next1.rust-suites
$next1.rust-suites.nu
$next1.rust-suites.nu.testcases
```
