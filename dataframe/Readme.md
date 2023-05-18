
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

### top level Cargo.toml file changes

```rust
add in a new workspace member

"crates/nu-cmd-dataframe",

and in the new crate

nu-cmd-lang = { path = "./crates/nu-cmd-lang", version = "0.79.1" }
nu-cmd-dataframe = { path = "./crates/nu-cmd-dataframe", version = "0.79.1", optional=true}
nu-command = { path = "./crates/nu-command", version = "0.79.1" }
```

### crates/nu-command/Cargo.toml

```rust
add this line of code

nu-cmd-lang = { path = "../nu-cmd-lang", version = "0.79.1" }
nu-cmd-dataframe = { path = "../nu-cmd-dataframe", version = "0.79.1", optional = true }
nu-color-config = { path = "../nu-color-config", version = "0.79.1" }

swap out this line
dataframe = ["num", "polars", "sqlparser"]

for this line
dataframe = ["dep:nu-cmd-dataframe"]
```

### crates/nu-command/src/default_context.rs

```rust
add this code

#[cfg(feature = "dataframe")]
use nu_cmd_dataframe::*;
```

### crates/nu-command/src/lib.rs

```rust
remove this code

#[cfg(feature = "dataframe")]
mod dataframe;

#[cfg(feature = "dataframe")]
pub use dataframe::*;
```
