
##### How to fire it up...

Compiles Devyn's branch and then issue this command

```rust
nurunn
```

After nushell comes up...

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
