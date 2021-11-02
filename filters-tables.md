
This document is mainly about nushell tables.

We start out with a summary of the subdirectory **filters** in the crate nu-command.

### These commands work on rows

  * first
  * last
  * nth [possible candidate after you finish reverse and shuffle]
  * range
  * reverse
  * roll
  * shuffle [ref](https://stackoverflow.com/questions/26033976/how-do-i-create-a-vec-from-a-range-and-shuffle-it)

### These commands work on columns

  * select
  * wrap
  * update

### These commands work on both

  * drop

### These commands work in tandem

  * keep
  * skip
  * append
  * prepend

### Notes

In the context of **follow_cell_path** Table is a List of vals where each val is a Row which is a Record
