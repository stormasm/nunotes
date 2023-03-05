
```rust
setup_config(
    &mut engine_state,
    &mut stack,
    #[cfg(feature = "plugin")]
    parsed_nu_cli_args.plugin_file,
    parsed_nu_cli_args.config_file,
    parsed_nu_cli_args.env_file,
    parsed_nu_cli_args.login_shell.is_some(),
);
```

All I have to do is pass in a -z --zero config file flag similar to the

```rust
login_shell.is_some()
```

### All I have to do

* in setup_config it gets the parameters config_file and env_file

pass in the zero config files here...

* parsed_nu_cli_args.config_file,
* parsed_nu_cli_args.env_file,



### read_config_file

If there is no config file goes through the process of asking you what you want to do...


craete two small files with just comments that do nothing,
but have data so it can live on github...

### src/run.rs
```rust
run_commands
run_file
run_repl
```

```rust
rg read_config_file
```

config stuff is referenced in

```
rg default_config
```

nu_utils
src/config_files.rs
