## log4rs - asynchronous log files rotation benchmark

```
cargo run --release -- [count_of_threads]
```

To rotate log files in a separate thread enable feature `async_rotation`:

```
cargo run --release --features=async_rotation -- [count_of_threads]
```
