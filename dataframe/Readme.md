
Notes for porting dataframe code over to a crate as part of the cratification effort.

### nu-cmd-dataframe

```rust
mkdir nu-cmd-dataframe
cd nu-cmd-dataframe
mkdir src
cd src
git mv dataframe from the nu-command crate to here
grab Cargo.toml, LICENSE, lib.rs from dfport03
in Cargo.toml do a global replace from 0.79.1 to 0.80.1
```




update the top level Cargo.toml file with the new workspace members
