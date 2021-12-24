

Notes on our fork of [hjson-rust](https://github.com/hjson/hjson-rust)
to nu-json.

nu-json in nushell and engine-q are no longer the same
because of this change that was made in this
[commit](https://github.com/nushell/engine-q/commit/624edce4f75f9ce01587fe29e21d8e823371ec77) to the file ser.rs

For this reason all of [these tests](https://github.com/nushell/nushell/blob/main/crates/nu-json/tests/main.rs) are failing...

##### Some examples that should work

```rust
[["a c" b]; ["jim smith" "susie roberts"] [3 4]] | to json -r
```
