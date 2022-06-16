
### Notes on what crates nushell uses for config

[discord](https://discord.com/channels/601130461678272522/614593951969574961/986790484204736522)

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

you can either edit your config file, or if you want a temporary config change you can do:

```rust
let config = ($config | update my.setting <my value>)
```

[discord-core](https://discord.com/channels/601130461678272522/683070703716925568/945440961713041499)
