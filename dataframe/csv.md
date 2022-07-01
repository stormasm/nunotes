
[issue](https://github.com/nushell/nushell/issues/5920)

#### Solution

```rust
ls | into df | to csv <filename>
```