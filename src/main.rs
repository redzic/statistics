use statistics::*;

fn main() {
    let values = [0.9, 0.95, 0.975, 0.99, 0.995, 0.999, 0.9995];

    println!("Inverse t-distribution table");

    print!("df↓ / P→\t");
    for i in values.iter() {
        print!("{}\t\t", i);
    }

    const N: i32 = 341;

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
