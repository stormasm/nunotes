
My motivation for doing this research was to build a *nu-command-core*
crate that enables me to develop nana without having to always pull
in a bunch of extra crates.  It paid off because I reduced the number
of crates nana needs by 127 crates from 579 down to 452...

### version 0.61.0

Nushell fully loaded without dataframe tops out at 429 crates.
With my minimal
[nu-command-core](https://github.com/stormasm/nu-command-core)
running
[nushell-core](https://github.com/stormasm/nushell-core) we get down
to 246 crates

### Nana

Nana fully loaded tops out at 579 crates
With my minimal
[nu-command-core](https://github.com/stormasm/nu-command-core)
452 crates...
