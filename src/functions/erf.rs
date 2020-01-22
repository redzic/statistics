use std::cmp::Ordering;
use std::f64;

fn erf_0(x: f64) -> f64 {
    // taylor series approximation for erf function centered at x=0
    // accurate for 0 <= x <= 0.5
    x * f64::consts::FRAC_2_SQRT_PI
        - (2.0 * x.powi(3)) / (3.0 * f64::consts::PI.sqrt())
        + (x.powi(5) / (5.0 * f64::consts::PI.sqrt()))
        - (x.powi(7) / (21.0 * f64::consts::PI.sqrt()))
        + (x.powi(9) / (108.0 * f64::consts::PI.sqrt()))
        - (x.powi(11) / (660.0 * f64::consts::PI.sqrt()))
        + (x.powi(13) / (4680.0 * f64::consts::PI.sqrt()))
        - (x.powi(15) / (37800.0 * f64::consts::PI.sqrt()))
        + (x.powi(17) / (342720.0 * f64::consts::PI.sqrt()))
        - (x.powi(19) / (3447360.0 * f64::consts::PI.sqrt()))
}

pub fn erf_1(x: f64) -> f64 {
    // taylor series approximation for erf function centered at x=1
    // accurate for 0.5 < x <= 1.5
    0.4275932955291202 - 0.4151074974205947 * (x - 1.0).powi(2)
        + 0.1383691658068649 * (x - 1.0).powi(3)
        + 0.06918458290343245 * (x - 1.0).powi(4)
        - 0.06918458290343245 * (x - 1.0).powi(5)
        + 0.004612305526895497 * (x - 1.0).powi(6)
        + 0.01515471815979949 * (x - 1.0).powi(7)
        - 0.004777030724284622 * (x - 1.0).powi(8)
        - 0.001885188370119985 * (x - 1.0).powi(9)
        + 0.001226287580563485 * (x - 1.0).powi(10)
        + 8.552399137172747e-5 * (x - 1.0).powi(11)
        + 0.4151074974205947 * x
}

fn erf_2(x: f64) -> f64 {
    // taylor series approximation for erf function centered at x=1
    // accurate for x <= 2.5
    0.9539882943107686264479314
        - 0.1653358828327364308565515 * (0.5 * x - 1.0).powi(2)
        + 0.3857837266097183386652869 * (0.5 * x - 1.0).powi(3)
        - 0.5511196094424547695218384 * (0.5 * x - 1.0).powi(4)
        + 0.4188509031762656248365972 * (0.5 * x - 1.0).powi(5)
        + 0.02939304583693092104116472 * (0.5 * x - 1.0).powi(6)
        - 0.432497674457697838177138 * (0.5 * x - 1.0).powi(7)
        + 0.4073036351688999058561396 * (0.5 * x - 1.0).powi(8)
        - 0.02566059557192381995657237 * (0.5 * x - 1.0).powi(9)
        - 0.2691096641070119881991081 * (0.5 * x - 1.0).powi(10)
        + 0.2125121455430861281163805 * (0.5 * x - 1.0).powi(11)
        + 0.02066698535409205385706894 * x
}

fn erf_polynomial(x: f64) -> f64 {
    let t = 1.0 / (1.0 + 0.5 * x.abs());
    let tau = t
        * (-x.powi(2) - 1.265_512_23
            + 1.000_023_68 * t
            + 0.374_091_96 * t.powi(2)
            + 0.096_784_18 * t.powi(3)
            - 0.186_288_06 * t.powi(4)
            + 0.278_868_07 * t.powi(5)
            - 1.135_203_98 * t.powi(6)
            + 1.488_515_87 * t.powi(7)
            - 0.822_152_23 * t.powi(8)
            + 0.170_872_77 * t.powi(9))
        .exp();

    if x >= 0.0 { 1.0 - tau } else { tau - 1.0 }
}

pub fn erf(x: f64) -> f64 {
    if x >= 0.0 {
        if x <= 0.5 {
            return erf_0(x);
        } else if x <= 1.5 {
            return erf_1(x);
        } else if x <= 2.5 {
            return erf_2(x);
        }

        erf_polynomial(x)
    } else {
        -erf(-x)
    }
}

pub fn sign(x: f64) -> f64 {
    match x.partial_cmp(&0f64).unwrap() {
        Ordering::Equal => 0.0,
        Ordering::Greater => 1.0,
        Ordering::Less => -1.0,
    }
}

pub fn inv_erf(x: f64) -> f64 {
    // TODO switch to a = 0.147 (change other consts too that depend on A's value)
    const A: f64 = 0.140_012_288_686_666_6;
    const FRAC_2_PI_A: f64 = 4.54688_49794_48285;
    let ln_one_minus_x_squared = (1.0 - x.powi(2)).ln();
    let y = FRAC_2_PI_A + ln_one_minus_x_squared / 2.0;

    sign(x) * ((y.powi(2) - ln_one_minus_x_squared / A).sqrt() - y).sqrt()
}

// OLD CODE
// fn erf_polynomial(x: f64) -> f64 {
//     let t = 1.0 / (1.0 + 0.5 * x.abs());
//     let tau = t
//         * (-x.powi(2) - 1.265_512_23
//             + 1.000_023_68 * t
//             + 0.374_091_96 * t.powi(2)
//             + 0.096_784_18 * t.powi(3)
//             - 0.186_288_06 * t.powi(4)
//             + 0.278_868_07 * t.powi(5)
//             - 1.135_203_98 * t.powi(6)
//             + 1.488_515_87 * t.powi(7)
//             - 0.822_152_23 * t.powi(8)
//             + 0.170_872_77 * t.powi(9))
//         .exp();

//     if x >= 0.0 { 1.0 - tau } else { tau - 1.0 }
// }

// fn erf_taylor(x: f64, mx2: f64) -> f64 {
//     x * (f64::consts::FRAC_2_SQRT_PI
//         + mx2 * (0.376_126_389_031_837_54 + mx2 * 0.112_837_916_709_551_26))
// }

// pub fn erf(x: f64) -> f64 {
//     let mx2: f64;

//     if x >= 0.0 {
//         if x < 5e-3f64 {
//             mx2 = -x.powi(2);
//             return erf_taylor(x, mx2);
//         }
//         erf_polynomial(x)
//     } else {
//         if x < -5e-3f64 {
//             mx2 = -x.powi(2);
//             return erf_taylor(x, mx2);
//         }
//         erf_polynomial(x)
//     }
// }
