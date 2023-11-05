
## Note 1 on engine-q

```rust
[4] | each {$it + 10}
each list
[14]

4 | each {$it + 10}
each default
14

4 | each {$it + 10} | each {$it + 10}
each default
each default
24

[4] | each {$it + 10} | each {$it + 10}
each list
each stream
[24]

[4] | each {$it + 10} | each {$it + 10} | each {$it + 10}
each list
each stream
each stream
[34]
```
