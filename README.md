
#### $in notes

when you want to just grab what the next to last pipeline is outputting and test it (for whatever reason)...

```rust
"john ran to the store" | str length | $in > 25
```

#### Alias Notes

```rust
alias "dfr describe" = dfr describe -q [0.5 0.90 0.95 0.99]
```

#### Env Notes
[discord](https://discord.com/channels/601130461678272522/614593951969574961/947802107325587518)

#### Need to look into optional arguments

here is an example [discord](https://discord.com/channels/601130461678272522/614593951969574961/946807063927988266)

more details [discord](https://discord.com/channels/601130461678272522/614593951969574961/946810405894897746)

more research on nu as a language [discord](https://discord.com/channels/601130461678272522/615329862395101194/946859683015180298)

#### First cut at exit codes

[discord-core](https://discord.com/channels/601130461678272522/683070703716925568/946842065998860348)

and the [PR](https://github.com/nushell/nushell/pull/4647)

#### Value and Strings

[discord-core](https://discord.com/channels/601130461678272522/683070703716925568/946312855668543538)

is there a difference between Value::as_string and Value::into_string?

as_string() converts any value that supports it to a string. into_string() formats the value and prints it, even lists etc. Maybe into_string() could be called print() or format() or something.

yeah, in theory there's a difference, so we don't accidentally convert something to string that shouldn't be but, like your external args, we should be able to safely convert ints to strings


#### There are two types of Streams in Nushell

* ListStream
* RawStream

ListStreams are used for Values any time you have an iterator

RawStreams are used for externals as well as the
open command when you don't have a file extension that
you know about.  In other words, it goes to raw when you do not know what
you are going to get back...

Both of these streams are referenced in nu-protocol in
* pipeline_data.rs
* value/stream.rs

#### what is the **exempt** label in issues

exempt from the stale bot

#### what is --testbin

--testbin String  
what does it mean to run internal test binary ?

It's a known external we can call from our tests, since not all platforms have echo, cat, etc...

ok so that means an end user would never use that feature (except if they were writing tests as a nushell developer for our code base)

[discord core](https://discord.com/channels/601130461678272522/683070703716925568/944671442371485756)

#### changes in nushell from legacy nushell

* pivot is now transpose
* insert (is now part of update)
* and nth (is now part of select)

#### Rolling towards release day

https://github.com/nushell/nushell/issues/4300

https://github.com/nushell/nushell/issues/4305

https://github.com/nushell/nushell/issues/4314

https://github.com/nushell/nushell/issues/4356

#### Location of the file config.nu
* $nu.config-path
* on mac: /Users/username/Library/Application Support/nushell

#### use metadata to create valid spans

[discord](https://discord.com/channels/601130461678272522/738253041546952705/939546327426162688)

but these spans need to be valid spans for this to work, and you can't create valid spans yourself (you need to use helpers like metadata).

#### append command

To see raw input values print val in the **append** command

```rust
let val: Value = call.req(engine_state, stack, 0)?;
println!("{:?}",val);
let vec: Vec<Value> = process_value(val);
```

#### engine-q notes

* [kubouch list of configurable settings](https://github.com/nushell/engine-q/issues/459)
* [fdncred](./engine-q/fdncred.md)
* [table syntax](./engine-q/table-syntax.md)
* [table syntax nushell old](./engine-q/nushell-table-syntax.md)

#### How to run engine-q

If you want to see all the plugins you need to do

```rust
cargo build --features=extra
cargo run --features=extra

nurun --help
```

#### Json with no formatting chars

```rust
[[a b]; [jim susie] [3 4]] | to json | str find-replace '\n' '' -a | str trim -a
```

#### Send back no data

```rust
Ok(PipelineData::Value(Value::Nothing { span: call.head }, None,))

// Or this way if Span is passed into the function

Ok(PipelineData::Value(Value::Nothing { span: *span }, None))

```

#### trait CustomValue

[discord](https://discord.com/channels/601130461678272522/889232844101156914/911337922890985512)

#### Getting column data

* get_data_by_key

commands that use get_data_by_key include **empty?** and **compact**

If you review empty? and debug the code you will see that given a particular column name

```rust
for column in column_paths.clone() {
    let path = column.into_string();
    let data = input.get_data_by_key(&path);
    println!("{:?} {:?}",path,data);
```

It goes through and prints each value going down the column...

* follow_cell_path

```rust
PathMember::Int
PathMember::String
```

The String arguments to follow_cell_path are for the column names.  
The Int arguments to follow_cell_path are for the rows.

```rust
[[a,b];[rick,pete], [bill,paul]] | get a
[[a,b];[rick,pete], [bill,paul]] | get b
[[a,b];[rick,pete], [bill,paul]] | get 0
[[a,b];[rick,pete], [bill,paul]] | get 1
```

For more details...
```rust
rg follow_cell_path
```

```rust
fn run(
    &self,
    engine_state: &EngineState,
    stack: &mut Stack,
    call: &Call,
    input: PipelineData,
) -> Result<nu_protocol::PipelineData, nu_protocol::ShellError> {
    let columns: Vec<CellPath> = call.rest(engine_state, stack, 0)?;
    let span = call.head;
```

In lots of commands you see the "call.rest" feature, this functionality
is defined in the nu-engine crate in [call_ext](https://github.com/nushell/engine-q/blob/main/crates/nu-engine/src/call_ext.rs)

```rust
pub trait CallExt {
```

#### Random commands

```rust
[[name,age];[bill,20],[rick,21]] | append [[name,age]; [paul, 40], [hb,70], [sally, 33], [sam,46]] | prepend [[name,age]; [sarah,40],[jane,50]]
```

#### How do you run CI checks locally ?

```
alias ciman='cargo fmt; cargo check; cargo clippy; cargo test --all'

cargo fmt
cargo check
cargo clippy
cargo test --all
```

[CI workflow steps manually](https://github.com/nushell/engine-q/blob/main/.github/workflows/ci.yml); &nbsp;&nbsp;
[discord link](https://discord.com/channels/601130461678272522/889232844101156914/904688334578794516)

#### Other useful cargo commands

```rust
cargo update --package reedline
```
* [discord](https://discord.com/channels/601130461678272522/855886335980994600/918604480965120081)

#### Other useful clippy commands

```rust
cargo clippy -- -W clippy::pedantic
```
* [discord](https://discord.com/channels/601130461678272522/855886335980994600/924724885039763468)

#### Rust Embed

[discord](https://discord.com/channels/601130461678272522/683070703716925568/921313094049882172)

we use this thing called rust-embed to embed things into the binary. Right now it looks like we're embedding themes for doing html, but I think you can use it for this as well

#### General Questions

#### what is our convention about commands having underscores ?

It's because commands like move_ is also a Rust keyword.

So commands like **let, do, if** have **underscore**, otherwise when you want to include them as a module, you'd type mod if and the compiler would complain that you cannot put if after mod.

#### Code Questions

example 24-bit terminals on mac including kitty, tabby, iterm2,
the default mac terminal is not 24-bit.

#### Anyone has an idea how to check if a value's type is Record?

You can pattern-match on the value:
```rust
if let Value::Record { .. } = value { <code> }
```

#### Convert tables and rows to Strings

>>> Does anyone know offhand if there is code in nushell to convert tables and rows to Strings?  I know there is the autoview command, but it uses a bunch of config stuff, etc. which I am trying to avoid

[discord](https://discord.com/channels/601130461678272522/615329862395101194/917099551058427904)

the engine-q table command will convert to strings   
in nushell, viewers don't return anything

>>> getting columns names in engine-q similar to nushell

[discord](https://discord.com/channels/601130461678272522/614593951969574961/921375934551048232)

#### Do not use...

```rust
Span::unknown()
```

[As discussed here in PR 242](https://github.com/nushell/engine-q/issues/242#issuecomment-997017183)

When working on commands, you can use call.head (this is a good option for new values created within the command) or reuse spans that come with Values from the input stream or command arguments. To get it right requires a bit of playing around so a good idea is to also purposefully trigger the errors and see how the messages look like.

#### References

* [ref 2022](./ref/ref22.md)

### Difference between a shell and a scripting language

[discord](https://discord.com/channels/601130461678272522/601130461678272524/939617182529241128)

* nakst Philosophically, what is the difference between a shell and a scripting language REPL? (Sorry if this is the wrong place to ask)
My intuition is that the former is designed for programmatic coordination of other applications, while the latter is instead aimed primarily at computation. But I'm interested to hear what people who have spent more time thinking about shells would argue.

* jt
for nushell, we're trying to merge the two concepts into one
traditionally a shell would be for interacting with the system directly and a REPL would be for interacting with the language's engine directly. For us, we'd like to do both equally well

* nakst
yeah, I'm trying to make a REPL for my scripting language but give it some shell-like capabilities
I find a little difficult to strike the right balance between the two

#### How to get input from the user ?
See the input command.

#### Lots of details about events and how order matters
[discord start here, coreteam](https://discord.com/channels/601130461678272522/683070703716925568/939977870258876486)

#### Move nth to select PR

This moves nth into select. This works by looking at the cell path we're given. If the cell path is a number, we follow the same logic as get: instead of a column name, use this as a row number.

The end result is that now select works like get, but instead of extracting data, it down-selects data and keeps the original shape intact. I think this will help teaching, as you can remember that one commands down-selects and one extracts and that works either for colum...

ref: [#4385](https://github.com/nushell/nushell/pull/4385)

#### When do I use Snake and when do I use Kebab ?

We know we didn't like either extreme, so after chatting with folks I think we should have a balance between the two: Here's my proposal:

Snake:
  * Column names
  * Cell paths (cell paths are column names)
  * Record fields (record fields are column names)
  * Variables (less confusing if you have math, eg) a_b - c_d is easier than a-b - c-d
  * env vars

Kebab:
  * Command names
  * Subcommand names
  * Flags (kebab flags appear to be the standard for most of the apps I checked)

[discord](https://discord.com/channels/601130461678272522/615329862395101194/943862648666222652)

#### How do I set up custom prompts ?

[discord](https://discord.com/channels/601130461678272522/601130461678272524/943882097175707718)

#### Long flags do not include "="

[discord](https://discord.com/channels/601130461678272522/614593951969574961/943957860373651536)

#### How do I build up a string ?

```rust
let age = 10
echo "my age is " $age | str collect
```
