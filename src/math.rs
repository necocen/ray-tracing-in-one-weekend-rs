#![cfg(feature = "approx-trigonometry")]

use std::f64::consts::{FRAC_PI_2, PI};

pub fn atan2(y: f64, x: f64) -> f64 {
    if x == 0.0 {
        if y == 0.0 {
            return 0.0;
        } else if y > 0.0 {
            return FRAC_PI_2;
        } else {
            return -FRAC_PI_2;
        }
    } else if y == 0.0 {
        if x > 0.0 {
            return 0.0;
        } else {
            return PI * y.signum();
        }
    }
    (((PI * y * (0.596227 + y / x)) / (x * ((2.0 * y * (1.192454 + y / x)) / x + 2.0))
        - (PI * y * (0.596227 - y / x)) / (x * ((2.0 * y * (y / x - 1.192454)) / x + 2.0)))
        * (1.0 + (x * y.abs()) / (x.abs() * y)))
        / 2.0
        + (PI * (1.0 - x.abs() / x) * y.abs()) / (2.0 * y)
        + (PI * y * (0.596227 - y / x)) / (x * ((2.0 * y * (y / x - 1.192454)) / x + 2.0))
}

pub fn acos(x: f64) -> f64 {
    atan2((1.0 - x * x).sqrt(), x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atan2() {
        for i in -1800..1800 {
            let theta = PI * i as f64 / 180.0 / 10.0;
            let tan = theta.tan();
            for x in -10..10 {
                let x = x as f64;
                let y = tan * x;
                let expected = y.atan2(x);
                let actual = atan2(y, x);
                assert!(
                    (expected - actual).abs() < 0.01,
                    "atan({y},{x}): {expected} {actual}"
                );
            }
        }
    }

    #[test]
    fn test_acos() {
        for i in -1800..1800 {
            let theta = PI * i as f64 / 180.0 / 10.0;
            let cos = theta.cos();
            let expected = cos.acos();
            let actual = acos(cos);
            assert!(
                (expected - actual).abs() < 0.01,
                "acos({cos}): {expected} {actual}"
            );
        }
    }
}
