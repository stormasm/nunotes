
So the findings for the day are the following...

This code is like a jigsaw puzzle...  

The code and how it is organized will drive how the crates will fit together.  If we want to get really selective on what stays and goes at least for the following Categories its going to be a lot of work...

* core
* env
* system
* shells
* viewers

Pretty much all go together and compiles...

My recommendation is to make this all one crate...
And possibly add more stuff if we want...

### More details

Core can live by itself but we actually will need let-env

So then you have to pull in env...

But then you need run-external

Which means pulling in system...

And run-external has a reference to table

So you have to pull in viewers...

### It compiles, but running it is interesting...

Nushell runs if env.nu and config.nu are both empty files...

```rust
help commands
```

brings up 68 commands...

However, if you want to install env.nu from the command prompt by answering yes you will get a

```rust
thread 'main' has overflowed its stack
fatal runtime error: stack overflow
```

References

[default_context.rs](https://github.com/stormasm/nushell/blob/two_com_crates_a/crates/nu-command-core/src/default_context.rs)