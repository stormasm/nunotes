
So the findings for the day are the following...

This code is like a jigsaw puzzle...

* core
* env
* system
* shells
* viewers

Pretty much all go together and compiles...

### More details

Core can live by itself but we actually will need let-env

So then you have to pull in env...

But then you need run-external

Which means pulling in system...

And run-external has a reference to table

So you have to pull in viewers...

### It compiles, but running it is interesting...

It works if you env.nu and config.nu are both empty files...

```rust
help commands
```

brings up 68 commands...

However, the env.nu causes

```rust
thread 'main' has overflowed its stack
fatal runtime error: stack overflow
```

If you try and install env.nu

References

[default_context.rs](https://github.com/stormasm/nushell/blob/two_com_crates_a/crates/nu-command-core/src/default_context.rs)