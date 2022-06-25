
```rust
open <file>
| into dB
| select *
| from history 
```

being able to do this is so nice 

```rust
open-db $nu.history-path | select * | from history | limit 10 | collect
```

[discord](https://discord.com/channels/601130461678272522/683070703716925568/990242225663402004)

### Spring 2022

April 23 Weekend Discussion on Future Directions and **compile the pipeline**

[discord](https://discord.com/channels/601130461678272522/683070703716925568/967678930742026251)

Some JT notes in speaking to Andres

nope, we're gradually typed, like TypeScript, so somewhere in between static and dynamic

also some other interesting points on blocks etc...

[discord](https://discord.com/channels/601130461678272522/683070703716925568/965109979503800321)

```rust
{foo: bar} | describe
record<foo: string>
[{foo: bar}] | describe
table<foo: string>
[[foo]; [([[bar]; [baz]])]] | describe
table<foo: table<bar: string>>

ah okay to nuon turns it into:

[[foo]; [[[bar]; ["baz"]]]]
```
which is also valid... so a list of one value, the table makes sense

### Earlier in the spring

[JT on the cons of eval in nushell 2022 early March](https://discord.com/channels/601130461678272522/683070703716925568/950568672290816021)
