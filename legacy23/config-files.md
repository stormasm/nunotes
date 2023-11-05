
Note: Thanks to Riley for reviewing this document prior to publication.

There was a question yesterday in the core team meeting about
which config files get loaded when the nu! macro runs...

The answer is just the env file as Riley noted.

Here is why.

The nu! macro runs a command and so run_commands get called...

run_commands which is called in main.rs is the method that loads the env config file

```rust
if parsed_nu_cli_args.env_file.is_some() {
    config_files::read_config_file(engine_state, &mut stack, parsed_nu_cli_args.env_file, true);
} else {
    config_files::read_default_env_file(engine_state, &mut stack)
}
```

Note in the code above that if no env file is passed into the nushell binary as an argument then

```rust
read_default_env_file
```

gets called...

### setup_config is only called from the repl

setup_config is only called from the repl and that is the only place where the default_config.nu gets loaded into the nushell process space along with the default_env.nu.  Of course that can be avoided by passing in the "no-config-file" flag.
