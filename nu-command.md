
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

References

[default_context.rs](https://github.com/stormasm/nushell/blob/two_com_crates_a/crates/nu-command-core/src/default_context.rs)