### How data in nushell gets printed out via the repl

* This shows [table](https://github.com/nushell/nushell/blob/main/crates/nu-command/src/viewers/table.rs) is the main default command of how data gets printed.
* The table command uses the nu-table crate which calls into [tabled](https://github.com/zhiburt/tabled).


If you don't want to see the output via table simply run this command

```rust
$nu | get os-info | debug -r
```

* In [nu-cli/src/util.rs](https://github.com/nushell/nushell/blob/main/crates/nu-cli/src/util.rs) in eval_source then eval_block there is this line of code

```rust
result = pipeline_data.print(engine_state, stack, false, false);
```

* In the
[pipeline_data](https://github.com/nushell/nushell/blob/main/crates/nu-protocol/src/pipeline_data.rs) see the **print** function which calls **write_all_and_flush**.  
* And that goes ahead and finally calls [stdout_write_all_and_flush](https://github.com/nushell/nushell/blob/main/crates/nu-utils/src/utils.rs)

These are the key points in the code where everything happens...

```rust
rg eval_source
rg eval_file
rg evaluate_file
rg write_all_and_flush
rg "print\("
```
