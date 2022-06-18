
Add this one line to Cargo.toml

```rust
mini-redis = "0.4.1"
```

And add this code to the version command and everything works.

```rust
use mini_redis::blocking_client;

let mut client = blocking_client::connect("localhost:6379").unwrap();

client.set("foo", "bar".into()).unwrap();

// Getting the value immediately works
let val = client.get("foo").unwrap().unwrap();
assert_eq!(val, "bar");

cols.push("features".to_string());
vals.push(Value::String {
    val: features_enabled().join(", "),
    span: call.head,
});
```
