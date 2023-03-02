
[discord](https://discord.com/channels/601130461678272522/683070703716925568/1080647423577301044)

```rust
~/Source/vm_experiment〉timeit { for x in 0..100000000 {} }
1min 44sec 728ms 140µs 41ns

~/Source/vm_experiment〉timeit { ./vm_benchmark }
465ms 324µs 125ns

~/Source/vm_experiment〉timeit { ./cranelift_benchmark }
57ms 128µs 416ns
```

### Compared to Python

```rust
timeit { python3 loop.py }
466ms 914µs 541ns
```

```rust
And luajit:
> timeit { luajit loop.lua }
38ms 968µs 916ns

And node:
> timeit { node loop.js }
101ms 675µs 125ns
```