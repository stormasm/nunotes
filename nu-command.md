
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

JT and I debugged the issue Monday night and discovered that if you comment out
the external command; then we no longer get a stack overflow meaning that the env.nu config file is referencing an external that is **not** in the crate.

I imagine that running the config code is needing something at the moment that
is not inside nu-command-core, but is inside nu-command...

All the tests are passing, plugin tests are failing, but that should be able to get fixed... Just haven't tracked that down yet...

#### References

[default_context.rs](https://github.com/stormasm/nushell/blob/two_com_crates_a/crates/nu-command-core/src/default_context.rs)

### Branch History

The latest branch is at the top

* buc_version1

Latest code base as of Feb 15 along with I figured out why there was an issue with version in the previous branches...

### Issue with version resolved

* You need the file build.rs in the crate where the version command is located.
* Cargo.toml has to have this next line of code in it and the shadow crate needs to be referenced in there twice.

```rust
build = "build.rs"
```

### Working branches older than above

* two_com_crates
* two_com_crates_a
