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
fn normal_dist_test() {
    let dist = Normal::new(0.0, 1.0);
    assert_eq!(dist.cdf(0.0), 0.5);
    assert_eq!(erf(0.0), 0.0);

    let dist2 = Normal::new(1.0, 0.5);
    assert_eq!(dist + dist2, Normal::new(1.0, 1.118033988749895));
}

#[test]
#[should_panic]
fn invalid_normal_dist() {
    Normal::new(0.0, 0.0);
}

#[test]
fn min_max_test() {
    let data = [
        0.0, 0.0, 2.0, -0.2, 5.0, -0.264, 61.9, -54.6, 72.9, 3.0, -0.2,
    ];

    assert_eq!(data.min(), -54.6);
    assert_eq!(data.max(), 72.9);
}
