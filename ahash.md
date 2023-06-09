


| function | main | ahash
| - | - | - |
| parse_default_env_file | 277.72 µs | 388.57 µs |
| parse_default_config_file | 614.40 µs | 1.4292 ms |
| eval default_env.nu  | 801.96 µs | 767.66 µs |
| eval default_config.nu  | 2.0325 ms | 1.9867 ms |


### References for comparison

Let's just run the parser_benchmarks for comparison initially

```rust
criterion_group!(benches, parser_benchmarks,);
criterion_main!(benches);
```
