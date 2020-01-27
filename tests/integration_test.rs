use statistics::*;

#[test]
fn basic_summary_stats_test() {
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
    // TODO add better tests than this
    assert_eq!(0f64.erf(), 0.0);

    let dist2 = Normal::new(1.0, 0.5);
    assert_eq!(dist + dist2, Normal::new(1.0, 1.118033988749895));
}

#[test]
fn binom_dist_test() {
    let dist = Binomial::new(200, 0.02);
    assert_eq!(dist.pmf(0), 0.0175879466057215);
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

#[test]
fn binom_coeff() {
    for i in 1..150 {
        assert_eq!(i.choose(1), i);
    }
    assert_eq!(5.choose(2), 10);
    assert_eq!(5.choose(3), 10);
    assert_eq!(14.choose(6), 3003);
    assert_eq!(87.choose(2), 3741);
    assert_eq!(17.choose(8), 24310);
    assert_eq!(53.choose(4), 292825);
}
