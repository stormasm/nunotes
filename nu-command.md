
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

Core can live by itself but we actually will need let-env if we want config files.  If we don't want config files core works.

So then you have to pull in env...

But then you need run-external

Which means pulling in system...

And run-external has a reference to table

So you have to pull in viewers...

### nu-command core compiles, and is now running...

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

However

### If you pull in both nu-command-core and nu-command everything runs fine.

I imagine that running the config code is needing something at the moment that
is not inside nu-command-core, but is inside nu-command...

All the tests are passing, plugin tests are failing, but that should be able to get fixed... Just haven't tracked that down yet...

References

[default_context.rs](https://github.com/stormasm/nushell/blob/two_com_crates_a/crates/nu-command-core/src/default_context.rs)