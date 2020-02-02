use crate::functions::gamma::Gamma;

/* Copyright 2014–2019 The special Developers
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the “Software”), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

pub trait Beta {
    fn beta_inc(self, p: Self, q: Self) -> Self;
}

pub fn beta(x: f64, y: f64) -> f64 {
    // TODO add check for negative integers
    (x.gamma() * y.gamma()) / (x + y).gamma()
}

// TODO bring over implementation of beta over here
// That crate isn't maintained anyway and is too small
// of a dependency to matter
pub fn beta_inc(a: f64, b: f64, x: f64) -> f64 {
    x.beta_inc(a, b)
}

/// Computes the regularized incomplete beta function
impl Beta for f64 {
    // Copied from `special` crate
    fn beta_inc(self, mut p: Self, mut q: Self) -> Self {
        // Algorithm AS 63
        // http://www.jstor.org/stable/2346797
        //
        // The function uses the method discussed by Soper (1921). If p is not
        // less than (p + q)x and the integral part of q + (1 - x)(p + q) is a
        // positive integer, say s, reductions are made up to s times “by parts”
        // using the recurrence relation
        //
        //                 Γ(p + q)
        // I(x, p, q) = ------------- x^p (1 - x)^(q - 1) + I(x, p + 1, q - 1)
        //              Γ(p + 1) Γ(q)
        //
        // and then reductions are continued by “raising p” with the recurrence
        // relation
        //
        //                             Γ(p + q)
        // I(x, p + s, q - s) = --------------------- x^(p + s) (1 - x)^(q - s)
        //                      Γ(p + s + 1) Γ(q - s)
        //
        //                    + I(x, p + s + 1, q - s)
        //
        // If s is not a positive integer, reductions are made only by “raising
        // p.” The process of reduction is terminated when the relative
        // contribution to the integral is not greater than the value of ACU. If
        // p is less than (p + q)x, I(1 - x, q, p) is first calculated by the
        // above procedure and then I(x, p, q) is obtained from the relation
        //
        // I(x, p, q) = 1 - I(1 - x, p, q).
        //
        // Soper (1921) demonstrated that the expansion of I(x, p, q) by “parts”
        // and “raising p” method as described above converges more rapidly than
        // any other series expansions.

        const ACU: f64 = 0.1e-14;

        let x = self;
        // debug_assert!(x >= 0.0 && x <= 1.0 && p > 0.0 && q > 0.0);

        if x == 0.0 {
            return 0.0;
        }
        if x == 1.0 {
            return 1.0;
        }

        let mut psq = p + q;

        let pbase;
        let qbase;

        let mut temp;

        let flip = p < psq * x;
        if flip {
            pbase = 1.0 - x;
            qbase = x;
            temp = q;
            q = p;
            p = temp;
        } else {
            pbase = x;
            qbase = 1.0 - x;
        }

        let mut term = 1.0;
        let mut ai = 1.0;

        let mut rx;
        let mut ns = (q + qbase * psq) as isize;
        if ns == 0 {
            rx = pbase;
        } else {
            rx = pbase / qbase;
        }

        let mut a = 1.0;
        temp = q - ai;

        loop {
            term = term * temp * rx / (p + ai);

            a += term;

            temp = if term < 0.0 { -term } else { term };
            if temp <= ACU && temp <= ACU * a {
                break;
            }

            ai += 1.0;
            ns -= 1;

            if 0 < ns {
                temp = q - ai;
            } else if ns == 0 {
                temp = q - ai;
                rx = pbase;
            } else {
                temp = psq;
                psq += 1.0;
            }
        }

        // Remark AS R19 and Algorithm AS 109
        // http://www.jstor.org/stable/2346887
        a = a * (p * pbase.ln() + (q - 1.0) * qbase.ln() - beta(p, q).ln()).exp()
            / p;

        if flip { 1.0 - a } else { a }
    }
}
