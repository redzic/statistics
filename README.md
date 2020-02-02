# Statistics

Statistical library written in Rust.

## Examples

Compute various statistical measures of a data set.

```rust
use statistics::*;

let dist = Normal::new(0.0, 1.0);
assert_eq!(dist.cdf(0.0), 0.5);
assert_eq!(dist.cdf(1.96), 0.9750021048517796);

let data = vec![0.0, 1.0, 2.5, 5.3, 0.2, -4.2];

println!("{}", data.mean());
println!("{}", data.median());
println!("{}", data.variance());
println!("{}", data.stdev());
println!("{}", data.min());
println!("{}", data.max());
```

Print an (right now only somewhat accurate) approximation of the inverse t-distribution table.

```rust
use statistics::*;

fn main() {
    let values = [0.9, 0.95, 0.975, 0.99, 0.995, 0.999, 0.9995];

    println!("Inverse t-distribution table");

    print!("df↓ / P→\t");
    for i in values.iter() {
        print!("{}\t\t", i);
    }

    const N: i32 = 120;

    println!();

    for i in 1..=N {
        print!(
            "{: >#width$}\t\t",
            i,
            width = (N as f64).log(10.0).floor() as usize + 1
        );
        let dist = T::new(i);
        for j in values.iter() {
            print!("{:.3}\t\t", dist.ppf(*j));
        }
        println!();
    }

    let dist = Normal::new(0.0, 1.0);
    print!(
        "{: >#width$}\t\t",
        "∞",
        width = (N as f64).log(10.0).floor() as usize + 1
    );

    for i in values.iter() {
        print!("{:.3}\t\t", dist.ppf(*i));
    }

    println!();
}
```
