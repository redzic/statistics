use ::statistics::*;

fn main() {
    let n = Normal::new(0.0, 1.0);

    println!("{:?}", n.cdf(0.0));
}
