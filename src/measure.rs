pub mod measure {
    use rayon::prelude::*;

    pub fn mean(data: &[f64]) -> f64 {
        data.iter().sum::<f64>() / data.len() as f64
    }

    pub fn median(data: &[f64]) -> f64 {
        let n = data.len();
        let mut _data = data.to_owned();

        _data.par_sort_unstable_by(|x, y| x.partial_cmp(&y).unwrap());

        match n % 2 {
            0 => (_data[n / 2 - 1] + _data[n / 2]) / 2.0,
            _ => _data[n / 2],
        }
    }

    pub fn variance(data: &[f64]) -> f64 {
        let mean = mean(data);

        data.iter().map(|i| (i - mean).powi(2)).sum::<f64>() / (data.len() as f64 - 1.0)
    }

    pub fn stdev(data: &[f64]) -> f64 {
        variance(data).sqrt()
    }

    fn _erf_approx(z: f64) -> f64 {
        const A_1: f64 = 0.070_523_078_4;
        const A_2: f64 = 0.042_282_012_3;
        const A_3: f64 = 0.009_270_527_2;
        const A_4: f64 = 0.000_152_014_3;
        const A_5: f64 = 0.000_276_567_2;
        const A_6: f64 = 0.000_043_063_8;

        1.0 - (1.0
            / (1.0
                + A_1 * z
                + A_2 * z.powi(2)
                + A_3 * z.powi(3)
                + A_4 * z.powi(4)
                + A_5 * z.powi(5)
                + A_6 * z.powi(6))
            .powi(16))
    }

    pub fn erf(z: f64) -> f64 {
        // TODO look for better approximation?
        if z > 0.0 {
            _erf_approx(z)
        } else {
            -_erf_approx(-z)
        }
    }

    // FIXME - refactor this into a method pertaining to dist::Normal struct
    pub fn normal_cdf(x: f64, mean: f64, stdev: f64) -> f64 {
        0.5 * (1.0 + erf((x - mean) / (stdev * 1.414213562373095)))
    }

    // TODO implement summary statistics
    // TODO implement the following:
    // IQR, skewness, correlation coefficient, etc
}
