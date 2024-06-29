
##### How to fire it up...

Compile Devyn's *ir* branch and then issue this command

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

```rust
if ([] | is-empty) {
  let x = 1;
  $x
}
```
