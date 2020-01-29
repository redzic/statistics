# Statistics

Statistical library written in Rust.

## Examples

```rust
use statistics::*;

let dist = Normal::new(0.0, 1.0);
assert_eq!(dist.cdf(0.0), 0.5);
assert_eq!(dist.cdf(1.96), 0.5);

let data = vec![0.0, 1.0, 2.5, 5.3, 0.2, -4.2];

println!("{}", data.mean());
println!("{}", data.median());
println!("{}", data.variance());
println!("{}", data.stdev());
println!("{}", data.min());
println!("{}", data.max());
```
