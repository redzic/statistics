use assert_approx_eq::assert_approx_eq;
use statistics::distributions::*;
use statistics::functions::*;
use statistics::stats::*;

// The exact floating point numbers are derived externally from SymPy,
// Which is then rounded at 16 decimal places
// Basically, though it may seem like the tests are just asserting the
// value they would return anyway, they actually aren't

#[test]
fn summary_stats() {
    let data = vec![2.0, 0.2, 5.03];
    assert_eq!(data.median(), 2.0);
    assert_eq!(data.mean(), 2.41);
    assert_eq!(data.variance(), 5.9583);
    assert_eq!(data.geometric_mean(), 1.262435869042509);
    assert_eq!(
        vec![1.5, 2.5, 2.5, 2.75, 3.25, 4.75].stdev(),
        1.0810874155219827
    );
    assert_eq!(
        vec![2.75, 1.75, 1.25, 0.25, 0.5, 1.25, 3.5].variance(),
        1.3720238095238095
    );
    assert_eq!(vec![1.0, 4.0, 4.0].harmonic_mean(), 2.0);
}

#[test]
fn normal_dist() {
    // TODO change where appropriate to assert_approx_eq and their
    // EXACT answer
    let dist = Normal::new(0.0, 1.0);
    assert_eq!(dist.cdf(0.0), 0.5);
    assert_eq!(dist.cdf(1.96), 0.9750021048517796);
    assert_approx_eq!(dist.ppf(0.9750021048517796), 1.96);
    assert_eq!(dist.ppf(0.5), 0.0);

    assert_eq!(0.0.erf(), 0.0);
    assert_eq!(1.25.erf(), 0.9229001282564582);

    assert_eq!(
        dist + Normal::new(1.0, 0.5),
        Normal::new(1.0, 1.118033988749895)
    );

    // TODO implement almost_eq macro, this test really
    // should pass.

    let temp_c = Normal::new(5.0, 2.5);
    assert_eq!(temp_c * (9.0 / 5.0) + 32.0, Normal::new(41.0, 4.5));
}

#[test]
fn t_dist() {
    assert_approx_eq!(T::new(1).pdf(1.0), 0.15915494309189534);
    assert_approx_eq!(T::new(2).pdf(1.0), 0.192450089729875);
    assert_eq!(beta_inc(1.0, 3.0, 0.02), 0.058808);
    let dist = T::new(1);
    assert_eq!(dist.cdf(0.0), 0.5);
    assert_approx_eq!(T::new(15).cdf(1.25), 0.88477470);

    let dist = T::new(1);
    for i in -300..300 {
        let i = i as f64 * 0.01;
        assert_approx_eq!(dist.ppf(dist.cdf(i)), i);
    }
}

#[test]
fn gamma_dist() {
    let dist = Gamma::new(5.0, 2.0);

    assert_eq!(dist.pdf(0.0), 0.0);
    // TODO add more tests
}

#[test]
fn exp_dist() {
    let dist = Exponential::new(1.0);
    assert_eq!(dist.mean(), 1.0);
    assert_eq!(dist.pdf(1.0), 0.3678794411714423215955);
}

#[test]
fn poisson_dist() {
    let dist = Poisson::new(2.0);
    assert_eq!(dist.pmf(4), 0.0902235221577418);
    assert_eq!(dist.cdf(4), 0.8571234604985472);
}

#[test]
fn binom_dist() {
    assert_eq!(Binomial::new(200, 0.02).pmf(0), 0.0175879466057215);
    assert_eq!(Binomial::new(50, 0.5).cdf(14), 0.0004681114554259125);
}

#[test]
#[should_panic]
fn invalid_normal() {
    Normal::new(0.0, 0.0);
}

#[test]
fn min_max() {
    let data = [
        0.0, 0.0, 2.0, -0.2, 5.0, -0.264, 61.9, -54.6, 72.9, 3.0, -0.2,
    ];

    assert_eq!(data.min(), -54.6);
    assert_eq!(data.max(), 72.9);
}

#[test]
fn binom_coeff() {
    assert_eq!(5.choose(2), 10);
    assert_eq!(5.choose(3), 10);
    assert_eq!(14.choose(6), 3003);
    assert_eq!(87.choose(2), 3741);
    assert_eq!(17.choose(8), 24310);
    assert_eq!(53.choose(4), 292825);
}
