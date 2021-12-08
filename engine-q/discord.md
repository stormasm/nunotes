
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
