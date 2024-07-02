
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

---

```rust
/// Example 01
view ir --json {
  let x = true;
  if ($x) {
    print "foo"
  }
}
```

---

```rust
view ir { for x in [1 2 3] { print $x }}
```

---

#### string-append

```rust
view ir { let name = "Alice"; $"greetings, ($name)" }
```

---

#### record-insert

```rust
view ir { [{A: A0}] | get 0.A }
```

---

#### view ast as json

```rust
ast '{1 + 2}' --json | get block | save -f look.json
```

```rust
ast 'help commands | where category == "debug"' --json | get block
```
