
### More details on $in

another way to think of it:

```rust
ls | $in
```

becomes:

```rust  
ls | collect { |x| $x }
```

Collect takes the iterator, drains it, and turns it into a value that it assigns to the variable. From there it calls the block and runs it
so:

```rust
3 | $in + 4
```

becomes:  

```rust
3 | collect { |x| $x + 4 }
```

(it's not always an iterator, like the above sometimes it's a single value)
but the general case is that collect runs and turns the input into a value. Wherever that data came from, if it has state, will now be drained/emptied

[discord](https://discord.com/channels/601130461678272522/614593951969574961/969846798627774505)

### How to access nu variable

* sorry but i cant figure out how to access the nu variable 😛 any hints?
[discord](https://discord.com/channels/601130461678272522/615962413203718156/947101719358238750)

* is it possible to declare a function so its usable by the user inside a block?

* I believe def-env makes it global, and let-env for variables.
[discord](https://discord.com/channels/601130461678272522/601130461678272524/947077158084444230)

* What is the difference between a PipelineMetadata::Value and PipelineMetadata::Stream ?
[discord](https://discord.com/channels/601130461678272522/889232844101156914/917872317747589131)
    * the Value case is a fully-known, finite value, and stream is a set of values of unknown length (possibly infinite) --- meaning its an Iterator.


* In the source command it uses position 1 for the block_id
[discord](https://discord.com/channels/601130461678272522/889232844101156914/894312997638512692)

* cargo bloat --all-features --time
[discord](https://discord.com/channels/601130461678272522/683070703716925568/918192921860243456),
[cargo-bloat](https://github.com/RazrFalcon/cargo-bloat)

* How engine-q works, JT goes through the whole logic of eq parsing about how the parser works
[discord](https://discord.com/channels/601130461678272522/889232844101156914/893316285037936730)
    * lite_parse takes tokens and groups them for you, but doesn't do a full parse
    * lite_parse and parse both run at keystroke and submit
    * To do syntax highlighting we do a full parse that includes classifying what everything is so we know the correct style for each span

##### Ref
* [github markdown note on how to do double indents as shown here :)](https://commonmark.org/help/tutorial/10-nestedLists.html)
