
[JT note on discord post landing initial PR](https://discord.com/channels/601130461678272522/683070703716925568/934639445012017182)

@storm - for the sorting stuff can you use the .gt() on Value I wonder?
that'll give you at least some way to compare the floats (and other types) if you have the original Values
there should be .eq() also

[JT note 2 on discord](https://discord.com/channels/601130461678272522/683070703716925568/935156195323433070)

[reddit](https://www.reddit.com/r/rust/comments/29kia3/no_ord_for_f32/)

```rust
aliases.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
```

makes me think we could probably use partial_cmp in sort-by, and if it ever fails, we could either do what that does or we could give an error to the user that the list couldn't be sorted

# Legacy Notes

These are the branches I have so far...

* sortselecta is the latest one, everything is working except Ord
is not implemented for Value...

sortbyb --- was all of the work I did which went nowhere...

sortbya --- is just an initial copy of getting the beginnings of select
renamed


January 2022

Starting to work on sort-by...

First thing to understand is how

##### get_data_by_key

works in engine-q and why it is relevant
to sort-by in engine-q

we are making progress see the filters/compact command.
It shows you that **get_data_by_key** is used to grab each data element going down the column.

as well as the command filters/empty

[Ref02](https://stackoverflow.com/questions/60916194/how-to-sort-a-vector-in-descending-order-in-rust)

[How to sort `Vec<String>` by certain columns](https://www.reddit.com/r/rust/comments/lu2jg3/how_to_sort_vecstring_by_certain_columns/)

[Primitive Type Slice](https://doc.rust-lang.org/std/primitive.slice.html)
