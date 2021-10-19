
This document is mainly about nushell tables.

We start out with a summary of the subdirectory **filters** in the crate nu-command.

### These commands work on rows

  * first
  * last
  * nth
  * range
  * reverse
  * roll
  * shuffle

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

**In the context of follow_cell_path** Table is a List of vals where each val is a Row which is a Record
