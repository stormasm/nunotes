
##### How eval works

- [excellent kubouch explanation](https://github.com/nushell/nushell/issues/2812#issuecomment-2558166296)

#### $nu

This is a critical command / variable for understanding all of your
config stuff et al in nushell !

#### nuconfig

```rust
$env.config.show_banner = false
$nu.startup-time
```

#### Cargo --locked

- cargo +1.80.1 install nu
- Always remember if things are NOT building to add --locked
- [discord ref](https://discord.com/channels/601130461678272522/683070703716925568/1308860637132361809)
- The --locked flag can be used to force Cargo to use the packaged Cargo.lock file if it is available.

#### CliFM

- Hustcer recommended [CliFM](https://github.com/leo-arch/clifm) [here on discord.](https://discord.com/channels/601130461678272522/683070703716925568/1307531577953353739)
- Jakub recommended [yazi](https://github.com/sxyazi/yazi)

### Sophia Terminal performance on the Mac

- [How to add your terminal to Developer Tools](https://nexte.st/docs/installation/macos/#how-to-add-your-terminal-to-developer-tools)

### Config file loading rules

- [Darren's Config Loading Rules](https://gist.github.com/fdncred/b87b784f04984dc31a150baed9ad2447/)

### Viewing binary data

```rust
open binaryfile.txt | into binary
```

### Improved nushell logging

```rust
nurun --log-level trace --log-exclude '[nu_parser, nu_cli, nu_utils, nu::config_files]'
```

- [Add options for filtering the log output from nu](https://github.com/nushell/nushell/pull/13044)

### Eval

- *nu-engine/src/eval.rs* is where the eval happens
- [How data in nushell gets printed out via the repl](./legacy23/print.md)

### Streaming and cheat-sheet notes on discord

[discord](https://discord.com/channels/601130461678272522/683070703716925568/1245123684205858856)

```rust
cheat-sheet --output list --input string
cheat-sheet --input list --output table
```

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

### Dataset

- https://www.kaggle.com/datasets/heesoo37/120-years-of-olympic-history-athletes-and-results
- [discord](https://discord.com/channels/601130461678272522/683070703716925568/1236806789412950099)
- https://www.kaggle.com/datasets/jessicali9530/animal-crossing-new-horizons-nookplaza-dataset
