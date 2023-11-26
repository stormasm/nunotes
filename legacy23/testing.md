
### december 2023

* [discord](https://discord.com/channels/601130461678272522/1177305370805407854/1177671705800487043)

### cargo-nextest

```rust
alias cnr='cargo nextest run'
cargo nextest run --all --all-features
```

* [doc](https://nexte.st/)
* [discord1](https://discord.com/channels/601130461678272522/683070703716925568/942909162004828191)
* [discord2](https://discord.com/channels/601130461678272522/683070703716925568/942985557800288268)

```rust
let next1 = (cargo nextest list -p nu --message-format json | from json)
let next2 = (cargo nextest list -p nu --features=extra --message-format json | from json)

$next1
$next1.rust-suites
$next1.rust-suites.nu
$next1.rust-suites.nu.testcases
```
