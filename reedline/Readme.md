
* [how does repaint work](./code.md)
* [setting emacs and vi as the default editor](./editor.md)
* [the insides of the menu system](./menu.md)
* [how to clear the screen](./screen.md)
* [InsertNewline in the context of the validator](./validator.md)

### Cargo.toml

Note the rev syntax...

```rust
# To use a development version of a dependency please use a global override here
# changing versions in each sub-crate of the workspace is tedious
[patch.crates-io]
reedline = { git = "https://github.com/maxomatic458/reedline", rev = "b6885642" }
#reedline = { git = "https://github.com/maxomatic458/reedline?branch=main#b6885642", branch = "main" }
# nu-ansi-term = {git = "https://github.com/nushell/nu-ansi-term.git", branch = "main"}
```

### References

* [ANSI Escape Sequences](./ansi-fnky.md)
