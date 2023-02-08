### How data in nushell gets printed out via the repl

First off we will note how to print the Block that the parser returns...

* Add these two lines of code [nu-cli/src/util.rs](https://github.com/nushell/nushell/blob/main/crates/nu-cli/src/util.rs) in the eval_source method right after the parse command.

```rust
use log::info;
info!("{:?}",block.pipelines);
```

* In [nu-cli/src/util.rs](https://github.com/nushell/nushell/blob/main/crates/nu-cli/src/util.rs) in eval_source then eval_block there is this line of code

```rust
result = pipeline_data.print(engine_state, stack, false, false);
```

* In the
[pipeline_data](https://github.com/nushell/nushell/blob/main/crates/nu-protocol/src/pipeline_data.rs) see the **print** function which calls **write_all_and_flush**.  
* And that goes ahead and finally calls [stdout_write_all_and_flush](https://github.com/nushell/nushell/blob/main/crates/nu-utils/src/utils.rs)

### Printing in nu-protocol

As noted above everything starts with the call to **eval_source** inside the nu-cli repl loop.

```rust
} else if !s.trim().is_empty() {
    info!("eval source: {}", s);

    eval_source(
        engine_state,
        stack,
        s.as_bytes(),
        &format!("entry #{}", entry_num),
        PipelineData::empty(),
    );
}

// On quick running commands everything gets printed from nu_protocol first
// prior to this next line of code firing off...
info!("after eval source and after nu-protocol printing...");
let cmd_duration = start_time.elapsed();
```

So once eval_source is called it then goes into nu-protocol [pipeline_data](https://github.com/nushell/nushell/blob/main/crates/nu-protocol/src/pipeline_data.rs) and there are one of two scenarios which can happen as noted in the eval_source method...

* pub fn print
* pub fn print_if_stream

Examples of stream commands include:
* ls
* open some csv file

If its not one of these streaming commands then a simple print gets called...

* This shows [table](https://github.com/nushell/nushell/blob/main/crates/nu-command/src/viewers/table.rs) is the main default command of how data gets printed.
* The table command uses the nu-table crate which calls into [tabled](https://github.com/zhiburt/tabled).

If you don't want to see the output via table or print_if_stream   

simply run one of these commands

```rust
$nu | get os-info | debug -r
```

```rust
1 + 3
```

```rust
help if
```

These are the key points in the code where everything happens...

```rust
rg eval_source
rg eval_file
rg evaluate_file
rg write_all_and_flush
rg "print\("
```

To see how all of this works and/or to add in some debugging / info! lines do this...

in nu-protocol Cargo.toml add this line

```rust
log = "0.4"
```

Then go ahead and add in all of your debugging lines

```rust
info!("table hit 03");
```
