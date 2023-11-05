
## zero out both config files so tests pass

if the nu macro calls into the default config files
things will fail because the current default config
files reference commands that do not exist.

some of the current tests are failing because they
reference commands that do not exist at this level.

## bug happens when the nu macro is called

* does the nu macro access the default env and config files ?

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