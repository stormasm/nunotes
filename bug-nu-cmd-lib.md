

default_env.nu is being called when the tests run even though
its not set in your local config file...

default_env.nu has commands in it that are not in the default context
so we get a stack overflow when this file gets evaluated or run.

### to reproduce a test bug -> cnr break_for_loop

### src/run.rs

* run_commands
* run_file
* run_repl


for more details 

```rust
rg default_env
rg default_config
```