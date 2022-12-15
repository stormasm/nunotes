
### Simple explanation of $in

* $in refers to the value passed from the pipe
* in case of each $in represents each row

[discord](https://discord.com/channels/601130461678272522/614593951969574961/969480626564636702)

### $in notes

when you want to just grab what the next to last pipeline is outputting and test it (for whatever reason)...

And $in is the variable that allows you to work
with all of the data coming in from the pipeline in one place.

The `$in` variable will collect the pipeline into a value for you, allowing you to access the whole stream as a parameter.

```rust
"john ran to the store" | str length | $in > 25
```

For more details....

```rust
rg -F '$in'
tutor -f "$in"
tutor var
```
