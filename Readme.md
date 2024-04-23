
### Building Nushell

```rust
cargo test --workspace --exclude nu_plugin_*
```
```rust
toolkit test
```

- [ref discord](https://discord.com/channels/601130461678272522/683070703716925568/1232125170714677339)

#### Contributor Book

* [Contributor Book which contains Plugin Stuff](https://www.nushell.sh/contributor-book/plugin_protocol_reference.html)

### Bare Words

bare words: foo, foo-bar, bar_baz   
bare words with spaces:

```
`foo bar` `hello world`
```

strings: "foo", 'foo-bar'   
bash: /foo\ bar/

bash:   
```
cd Some\ Dir
```

Nushell:   
```
cd `Some Dir`
```

### A creative hack to allow config reloads

* [Config Reloads](https://github.com/nushell/nushell/issues/10736)

### The latest and greatest way to source nu code

```rust
source $'($nu.default-config-dir)/config.nu'
source $'([($nu.default-config-dir) config.nu] | path join)'
source ($"($nu.default-config-dir)" + '/config.nu')
```

[Description of the concept which is constants](https://discord.com/channels/601130461678272522/601130461678272524/1199014467980251237)

The key takeaway here is source and use work with constants. Now that some string interpolations are constants, you can do this type of thing. You can also do this type of thing

```rust
const f = "foo.nu"
source $f
```

again, because f is constant.

---

* [testing](./legacy23/testing.md)
* [legacy23](./legacy23/README.md)

---

### toolkit.nu

```rust
source toolkit.nu
fmt
clippy
test
test stdlib
```

---

### Latest Stuff

This gets moved out to another location after some time

```rust
# Note you have to put the --env after def in order for this to work

def --env changedir [] {
  cd /Users/ma/j/tmp17/nunotes
}
```

[ref](https://github.com/stormasm/nuscripts/blob/main/changedir.nu)

---

* [awesome rust repo stats](https://github.com/emanuelef/awesome-rust-repo-stats)

---

* [notrace](https://github.com/stormasm/rust-examples/tree/main/notrace)
* [nushell 2023 survey results](https://www.nushell.sh/blog/2023-11-16-nushell-2023-survey-results.html)

ok
