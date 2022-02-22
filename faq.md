
### how do I set nushell to be my default shell on mac

* make sure nu is in your /etc/shells
* make sure nu is in your path
* run chsh

### how can I merging two dict(or lists) into one?

```rust
let env1 = {a: "a", b: "b"}
let env2 = {c: "c", d: "d"}

#to get the type:
$env1 | describe

#to merge them:
$env1 | merge { $env2 }
```
