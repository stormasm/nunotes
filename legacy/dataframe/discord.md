
Yeap, that **Object datatype** is a curse and a blessing. It let us convert anything non standard into a df. You can have dfs inside dfs thanks to it but at the same time it gives the illusion that all DF commands should work on it

[discord](https://discord.com/channels/601130461678272522/683070703716925568/987336961926791208)

[if your want to see all dfr commands you have to use the scope](https://discord.com/channels/601130461678272522/683070703716925568/985489091179188284)

```rust
$nu.scope.commands | where category =~ "lazyframe"
```

[what are your thoughts on dataframes. Do you still want to migrate to datafusion ?](https://discord.com/channels/601130461678272522/683070703716925568/982368444269862972)

[You don't need a command (it would deactivate the overlay anyway at the end of the scope). You create a dfr module that contains all the dfr commands, then either do use dfr which brings the commands as they are now, or call overlay add dfr which would bring it in as an overlay.](https://discord.com/channels/601130461678272522/683070703716925568/982401998299201576)

---

I want to see more rows than nushell will display. For example, if I use '| last 30', I see rows 0-9 .. 20-29. Is there any way to configure or change the default elision behavior to actually show me all 30 rows, like a table does?

There's no configuration setting to do that. The code would have to be changed. However, a hack work-around can be demonstrated like this ls | into df - now this is a dataframe displayed the way you describe. If you want to see all the records, one was is to take it back to a nushell table like ls | into df | into nu.

```rust
ls | into df | into nu
```

[discord](https://discord.com/channels/601130461678272522/864228801851949077/1048317462770372730)
