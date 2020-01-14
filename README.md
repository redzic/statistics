# Statistics

Statistical library written in rust.

```rust
let dist = Normal::new(0.0, 1.0);
dist.cdf(0.0);

let data = vec![0.0, 1.0, 2.5, 5.3, 0.2, -4.2];

println!("{}", mean(&data));
println!("{}", median(&data));
println!("{}", variance(&data));
println!("{}", stdev(&data));
```
