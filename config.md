
### Notes on how to work with config.nu files

How to assign a F(5) key...

```rust
{
        name: reload_config
        modifier: none
        keycode: f5
        mode: emacs
        event: [
            { edit: { cmd: clear } }
            { edit: { cmd: insertString value: $"source '($nu.config-path)'" } }
            { send: Enter }
        ]
}
```
