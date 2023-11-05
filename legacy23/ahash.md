

### Saturday June 10, 2023

Apple M1 Pro with 16G of memory

| function | main | ahash
| - | - | - |
| parse_default_env_file | 411.49 µs | 401.16 µs |
| parse_default_config_file | 1.4912 ms | 1.4415 ms |
| eval default_env.nu  | 823.05 µs | 770.71 µs |
| eval default_config.nu  | 2.0569 ms | 1.9858 ms |
| eval default_env.nu #2 | 812.52 µs | 767.01 µs |
| eval default_config.nu #2 | 2.0333 ms | 1.9896 ms |

### main

parse_default_env_file  time:   [411.49 µs

parse_default_config_file
                        time:   [1.4912 ms

eval default_env.nu     time:   [823.05 µs

eval default_config.nu  time:   [2.0569 ms

eval default_env.nu #2  time:   [812.52 µs

eval default_config.nu #2
                        time:   [2.0333 ms

### ahash
parse_default_env_file  time:   [401.16 µs

parse_default_config_file
                        time:   [1.4415 ms

eval default_env.nu     time:   [770.71 µs

eval default_config.nu  time:   [1.9858 ms

eval default_env.nu #2  time:   [767.01 µs

eval default_config.nu #2
                        time:   [1.9896 ms


### Friday June 9, 2023

| function | main | ahash
| - | - | - |
| parse_default_env_file | 277.72 µs | 388.57 µs |
| parse_default_config_file | 614.40 µs | 1.4292 ms |
| eval default_env.nu  | 801.96 µs | 767.66 µs |
| eval default_config.nu  | 2.0325 ms | 1.9867 ms |


### References for comparison

Let's just run the parser_benchmarks for comparison initially and then we can branch out if we want to other areas...

```rust
criterion_group!(benches, parser_benchmarks, eval_benchmarks,);
criterion_main!(benches);
```

### Details

* a microsecond is one millionth of a second
* a millisecond is 1000 microseconds
