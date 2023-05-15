
Notes for porting dataframe code over to a crate as part of the cratification effort.

```rust
mkdir nu-cmd-dataframe
cd nu-cmd-dataframe
mkdir src
cd src
git mv dataframe from the nu-command crate to here
grab Cargo.toml from the nu-command crate and LICENSE
add in your lib.rs file

update the top level Cargo.toml file with the new workspace members
```
