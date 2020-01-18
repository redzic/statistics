use crate::functions::functions::*;
use crate::statistics::traits::*;

#[derive(Debug)]
pub struct Normal {
    mean: f64,
    stdev: f64,
}

impl Normal {
    // TODO create traits to avoid having to rewrite each method thing
    // TODO allow addition, subtraction, multiplication, and division of Normal
    // distributions
    pub fn new(mean: f64, stdev: f64) -> Self {
        Normal { mean, stdev }
    }

    pub fn from(data: &[f64]) -> Self {
        // TODO change functions so it doesn't keep recalculating the mean
        // aka provide optional mean parameter for the mean that you've already
        // calculated

        Normal {
            mean: data.mean(),
            stdev: data.stdev(),
        }
    }
}

impl Mean for Normal {
    fn mean(&self) -> f64 {
        self.mean
    }
}

impl Median for Normal {
    fn median(&self) -> f64 {
        self.mean
    }
}

impl Mode for Normal {
    fn mode(&self) -> f64 {
        self.mean
    }
}

impl StdDev for Normal {
    fn stdev(&self) -> f64 {
        self.stdev
    }
}

impl Variance for Normal {
    fn variance(&self) -> f64 {
        self.stdev.powi(2)
    }
}

impl Cdf for Normal {
    fn cdf(&self, x: f64) -> f64 {
        0.5 * (1.0 + erf((x - self.mean) / (self.stdev * std::f64::consts::SQRT_2)))
    }
}

impl InvCdf for Normal {
    fn inv_cdf(&self, p: f64) -> f64 {
        // TODO check p to see if it is valid
        let q = p - 0.5;

        let num: f64;
        let den: f64;

        if q.abs() <= 0.425 {
            let r = 0.180625 - q.powi(2);

            num = (((((((2.50908_09287_30122_6727e+3 * r
                + 3.34305_75583_58812_8105e+4)
                * r
                + 6.72657_70927_00870_0853e+4)
                * r
                + 4.59219_53931_54987_1457e+4)
                * r
                + 1.37316_93765_50946_1125e+4)
                * r
                + 1.97159_09503_06551_4427e+3)
                * r
                + 1.33141_66789_17843_7745e+2)
                * r
                + 3.38713_28727_96366_6080e+0)
                * q;

            den = ((((((5.22649_52788_52854_5610e+3 * r
                + 2.87290_85735_72194_2674e+4)
                * r
                + 3.93078_95800_09271_0610e+4)
                * r
                + 2.12137_94301_58659_5867e+4)
                * r
                + 5.39419_60214_24751_1077e+3)
                * r
                + 6.87187_00749_20579_0830e+2)
                * r
                + 4.23133_30701_60091_1252e+1)
                * r
                + 1.0;

            let x = num / den;
            return self.mean + (x * self.stdev);
        }

        let mut r = if q <= 0.0 { p } else { 1.0 - p };
        r = (-r.ln()).sqrt();

        if r <= 5.0 {
            r -= 1.6;

            num = ((((((7.74545_01427_83414_07640e-4 * r
                + 2.27238_44989_26918_45833e-2)
                * r
                + 2.41780_72517_74506_11770e-1)
                * r
                + 1.27045_82524_52368_38258e+0)
                * r
                + 3.64784_83247_63204_60504e+0)
                * r
                + 5.76949_72214_60691_40550e+0)
                * r
                + 4.63033_78461_56545_29590e+0)
                * r
                + 1.42343_71107_49683_57734e+0;

            den = ((((((1.05075_00716_44416_84324e-9 * r
                + 5.47593_80849_95344_94600e-4)
                * r
                + 1.51986_66563_61645_71966e-2)
                * r
                + 1.48103_97642_74800_74590e-1)
                * r
                + 6.89767_33498_51000_04550e-1)
                * r
                + 1.67638_48301_83803_84940e+0)
                * r
                + 2.05319_16266_37758_82187e+0)
                * r
                + 1.0;
        } else {
            r -= 5.0;

            num = ((((((2.01033_43992_92288_13265e-7 * r
                + 2.71155_55687_43487_57815e-5)
                * r
                + 1.24266_09473_88078_43860e-3)
                * r
                + 2.65321_89526_57612_30930e-2)
                * r
                + 2.96560_57182_85048_91230e-1)
                * r
                + 1.78482_65399_17291_33580e+0)
                * r
                + 5.46378_49111_64114_36990e+0)
                * r
                + 6.65790_46435_01103_77720e+0;

            den = ((((((2.04426_31033_89939_78564e-15 * r
                + 1.42151_17583_16445_88870e-7)
                * r
                + 1.84631_83175_10054_68180e-5)
                * r
                + 7.86869_13114_56132_59100e-4)
                * r
                + 1.48753_61290_85061_48525e-2)
                * r
                + 1.36929_88092_27358_05310e-1)
                * r
                + 5.99832_20655_58879_37690e-1)
                * r
                + 1.0;
        }

        let mut x = num / den;
        if q < 0.0 {
            x = -x;
        }

        self.mean + (x * self.stdev)
    }
}
