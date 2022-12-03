
I want to see more rows than nushell will display. For example, if I use '| last 30', I see rows 0-9 .. 20-29. Is there any way to configure or change the default elision behavior to actually show me all 30 rows, like a table does?

There's no configuration setting to do that. The code would have to be changed. However, a hack work-around can be demonstrated like this ls | into df - now this is a dataframe displayed the way you describe. If you want to see all the records, one was is to take it back to a nushell table like ls | into df | into nu.

```rust
ls | into df | into nu
```

[discord](https://discord.com/channels/601130461678272522/864228801851949077/1048317462770372730)