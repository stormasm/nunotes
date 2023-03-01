

default_env.nu is being called when the tests run even though
its not set in your local config file...

default_env.nu has commands in it that are not in the default context
so we get a stack overflow when this file gets evaluated or run.

see src/run.rs

for more details 

```rust
rg default_env
rg default_config
```