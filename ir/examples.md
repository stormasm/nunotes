```rust
{
  if ([] | is-empty) {
    if ((random int 0..2) < 1) {
      print "foo"
    }
  } else {
    print "bar"
  }
}
```

```rust
view ir --json {
  if ([] | is-empty) {
    if ((random int 0..2) < 1) {
      print "foo"
    }
  } else {
    print "bar"
  }
} | explore ir
```
