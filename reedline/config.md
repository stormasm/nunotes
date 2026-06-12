

- [discord ref](https://discord.com/channels/601130461678272522/683070703716925568/1514996899437088850)

```rust
$env.config.keybindings ++= [{
    name: erase_to_x
    modifier: control
    keycode: char_e
    mode: emacs
    event: { edit: erase, motion: find, char: "x", direction: forward, stop: before }
}]
```
