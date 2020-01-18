use ::statistics::*;

#[test]
fn stats_test() {
    let data = vec![2.0, 0.2, 5.03];
    assert_eq!(data.median(), 2.0);
    assert_eq!(data.mean(), 2.41);
    assert_eq!(data.variance(), 5.9583);
    assert_eq!(data.geometric_mean(), 1.262435869042509);
}

#[test]
fn distribution_test() {
    let dist = Normal::new(0.0, 1.0);
    assert_eq!(dist.cdf(0.0), 0.5);
    assert_eq!(erf(0.0), 0.0);
}

#[test]
#[should_panic]
fn invalid_distribution() {
    Normal::new(0.0, 0.0);
}
