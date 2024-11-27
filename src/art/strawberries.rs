//! https://x.com/naderi_yeganeh/status/1858455441782534161
//! Sunflower Field by Hamid Naderi Yeganeh

#![allow(non_snake_case, clippy::let_and_return)]

use crate::utils::*;
use crate::*;
use core::f64;
use std::f64::consts::PI;

pub const FULL_M: usize = 2000;
pub const FULL_N: usize = 1200;
const HALF_M_INT: usize = FULL_M / 2;
const HALF_M: f64 = HALF_M_INT as f64;
const HALF_N: f64 = (FULL_N / 2) as f64;
const HALF_N_PLUS_ONE: f64 = HALF_N + 1.;

pub struct Artwork;

impl Art for Artwork {
    const FULL_M: usize = FULL_M;

    const FULL_N: usize = FULL_N;

    fn draw(m: f64, n: f64) -> (u8, u8, u8) {
        draw(m, n)
    }
}

#[inline(always)]
pub fn draw(m: f64, n: f64) -> (u8, u8, u8) {
    let result = rgb(
        F(H(0, (m - HALF_M) / HALF_N, (HALF_N_PLUS_ONE - n) / HALF_N)),
        F(H(1, (m - HALF_M) / HALF_N, (HALF_N_PLUS_ONE - n) / HALF_N)),
        F(H(2, (m - HALF_M) / HALF_N, (HALF_N_PLUS_ONE - n) / HALF_N)),
    );

    result
}

#[inline(always)]
pub fn rgb(r: f64, g: f64, b: f64) -> (u8, u8, u8) {
    let result = (r.round() as u8, g.round() as u8, b.round() as u8);
    result
}

pub fn F(x: f64) -> f64 {
    let term0 = 255. * e(-e(-HALF_M * x));
    let term1 = abs(x).powf(e(-e(HALF_M * (x - 1.))));
    let result = term0 * term1;
    result
}

/// H(v,x,y) called with v = [0, 1, 2]
pub fn H(v: usize, x: f64, y: f64) -> f64 {
    let v_ = v as f64;
    let result = sum(1, 30, |s| {
        let s = s as usize;
        let term0 = product_with_key("H", 0, s - 1, x, y, |r, x, y| {
            let r_ = r;
            let r = r as usize;

            let term010 = 1. - A(HALF_M_INT, r, x, y);
            let term0110 = e(-e(-HALF_M * (r_ - 1. / 2.)));
            let term0111 = U(r, x, y);
            let term011 = 1. - term0110 * term0111;
            let term0120 = 5. / 4.;
            let term0121 = A(4, r, x, y);
            let term012 = 1. - term0120 * term0121;
            let term01 = term010 * term011 * term012;
            term01
        });
        let term10000 = v_ - 1.;
        let term1000 = 3. * term10000.pow2();
        let term1001 = W(x, y);
        let term100 = 5. - term1000 + term1001;
        let term10 = term100 / 10.;
        let term110 = 71. / 10. - 10. * P(s, x, y);
        let term11 = e(-e(-abs(term110)));
        let term12 = U(s, x, y);
        let term13 = A(HALF_M_INT, s, x, y);
        let term14 = 1. - U(s, x, y);
        let term15 = L(v, s, x, y);
        let term1 = term10 * term11 * term12 + term13 * term14 * term15;
        term0 * term1
    });
    result
}

memo_many! {
/// L(v,s,x,y) called with v = [0, 1, 2], s = 0..=30
    pub fn L(v: usize, s: usize, x: f64, y: f64) -> f64 {
        let v_ = v as f64;
        let s_ = s as f64;

        // debug_store_value(v_);
        let term01 = 1. / 10.;
        let term02 = 1. / 40.;
        let term03 = cos(20. * arccos(R(0, s, x, y)));
        let term04 = cos(25. * P(s, x, y));
        let term0 = term01 - term02 * term03 * term04;
        let term10 = 4. * v_.pow2() - 13. * v_ + 11.;
        let term11 = cos(7. * s_ + v_ * s_);
        let term120 = C(20, s, x, y) - 1. / 2.;
        let term12 = 20. * e(-e(-70. * term120));
        let term130 = C(10, s, x, y) - 1. / 2.;
        let term13 = 20. * e(-e(-10. * term130));
        let term1 = term10 + term11 + term12 + term13;
        let term2 = A(4, s, x, y);
        let term3 = A(HALF_M_INT, s, x, y);
        let term4 = B(s, x, y);
        let result = term0 * term1 * term2 * term3 + term4;
        result
    }
}

memo_many! {
    /// C(v,s,x,y) called with v = [10, 20], s = 0..=30
    pub fn C(v: usize, s: usize, x: f64, y: f64) -> f64 {
        let v_ = v as f64;
        let term0xx0 = 10. * arccos(R(0, s, x, y));
        let term0xx1 = 25. / 2. * P(s, x, y);
        let term0xx2 = 7. / 10.;
        let term0xx3 = W(x, y) / 5.;
        let term000 = cos(term0xx0) * cos(term0xx1) - term0xx2 - term0xx3;
        let term00 = v_ * term000;
        let term010 = cos(term0xx0) * cos(term0xx1) + term0xx2 + term0xx3;
        let term01 = -v_ * term010;
        let term020 = sin(term0xx0) * sin(term0xx1) - term0xx2 - term0xx3;
        let term02 = v_ * term020;
        let term030 = sin(term0xx0) * sin(term0xx1) + term0xx2 + term0xx3;
        let term03 = -v_ * term030;
        let term0 = e(-e(term00) - e(term01) - e(term02) - e(term03));
        let term1000 = Q(s, x, y);
        let term1001 = P(s, x, y) - 1. / 4.;
        let term1002 = 21. / 50.;
        let term1003 = W(x, y) / 5.;
        let term100 = term1000.pow2() + term1001.pow2() - term1002 + term1003;
        let term10 = 3. / 2. * term100;
        let term1 = e(-e(term10));
        let result = term0 * term1;
        result
    }
}

memo_many! {
    /// B(s,x,y) called with s = 0..=30
    pub fn B(s: usize, x: f64, y: f64) -> f64 {
        let term000 = cos(20. * arccos(R(0, s, x, y)));
        let term001 = cos(25. * P(s, x, y));
        let term002 = 47. / 50.;
        let term00 = term000 * term001 - term002;
        let term0 = -70. * term00;
        let result = e(-e(term0));
        result
    }
}

memo_many! {
    /// A(v,s,x,y) called with v = [4, 1000], s = 0..=30
    pub fn A(v: usize, s: usize, x: f64, y: f64) -> f64 {
        let v_ = v as f64;
        let s_ = s as f64;
        let term0 = s_ - 1. / 2.;
        let term10 = 5. / 4.;
        let term11 = 1. - P(s, x, y);
        let term12 = Q(s, x, y);
        let term13 = P(s, x, y);
        let term14 = 11. / 20.;
        let term150 = v_ - 100.;
        let term151 = 10. * PI;
        let term15 = arctan(100. * (term150)) / term151;
        let term1 = term10 * term11 * term12.pow2() + term13.pow2() - term14 + term15;
        let term20 = Q(s, x, y);
        let term21 = P(s, x, y);
        let term2 = term20.pow2() + term21.pow2() - 1.;
        let result = e(-e(-HALF_M * term0) - e(v_ * term1) - e(v_ * term2));
        result
    }
}

memo_many! {
    /// U(s,x,y) called with s = 0..=30
    pub fn U(s: usize, x: f64, y: f64) -> f64 {
        let term0 = 1. - M(s, x, y);
        let term1 = 1. - N(s, x, y);
        let result = 1. - term0 * term1;
        result
    }
}

memo_many! {
    /// M(s,x,y) called with s = 0..=30
    pub fn M(s: usize, x: f64, y: f64) -> f64 {
        let s_ = s as f64;
        let term00 = P(s, x, y);
        let term01 = 57. / 100.;
        let term020 = cos(7. * Q(s, x, y) + 2. * s_);
        let term02 = 3. / 20. + term020 / 10.;
        let term030 = 10. + 3. * cos(14. * s_);
        let term031 = arccos(R(0, s, x, y));
        let term032 = 3. / 10.;
        let term033 = cos(45. * x + 47. * y + cos(17. * x));
        let term034 = 2. * cos(5. * s_);
        let term03 = cos(term030 * term031 + term032 * term033 + term034);
        let term0 = term00 - term01 - term02 * term03;
        let term10 = P(s, x, y);
        let term11 = 18. / 25.;
        let term12 = 3. / 2. * Q(s, x, y);
        let term1 = term10 - term11 - term12.pow8();
        let result = e(-e(-100. * term0) - e(HALF_M * term1));

        result
    }
}

memo_many! {
    /// N(s,x,y) called with s = 0..=30
    pub fn N(s: usize, x: f64, y: f64) -> f64 {
        let s_ = s as f64;
        let term00 = P(s, x, y);
        let term01 = 37. / 50.;
        let term020 = cos(8. * Q(s, x, y) + 5. * s_);
        let term02 = 3. / 20. + term020 / 10.;
        let term030 = 10. + 3. * cos(16. * s_);
        let term031 = arccos(R(1, s, x, y));
        let term032 = 3. / 10.;
        let term033 = cos(38. * x - 47. * y + cos(19. * x));
        let term034 = 2. * cos(4. * s_);
        let term03 = cos(term030 * term031 + term032 * term033 + term034);
        let term0 = term00 - term01 - term02 * term03;
        let term10 = P(s, x, y);
        let term11 = 71. / 100.;
        let term12 = 3. / 2. * Q(s, x, y);
        let term1 = term10 - term11 - term12.pow8();
        let result = e(-e(100. * term0) - e(-HALF_M * term1));
        result
    }
}

memo_many! {
    /// R(t,s,x,y) called with t = [0, 1], s = 0..=30
    pub fn R(t: usize, s: usize, x: f64, y: f64) -> f64 {
        let term0 = E(t, s, x, y);
        let term100 = E(t, s, x, y);
        let term10 = abs(term100) - 1.;
        let term1 = e(-e(HALF_M * term10));
        let result = term0 * term1;
        result
    }
}

memo_many! {
    /// E(t,s,x,y) called with t = [0, 1], s = 0..=30
    pub fn E(t: usize, s: usize, x: f64, y: f64) -> f64 {
        let t_ = t as f64;
        let term0 = HALF_M / sqrt(20.);
        let term1 = Q(s, x, y);
        let term200 = 1. - 2. * t_;
        let term201 = P(s, x, y);
        let term20 = 20. - 20. * term200 * term201 - 27. * t_;
        let term2 = sqrt(5. * abs(term20));
        let term30000 = 1. - 2. * t_;
        let term30001 = P(s, x, y);
        let term3000 = 20. * term30000 * term30001 + 27. * t_;
        let term300 = 200. - term3000.pow2();
        let term30 = abs(4. * term300);
        let term3 = 1. + 50. * sqrt(term30);
        let result = term0 * term1 * term2 * term3.powneg1();
        result
    }
}

memo_many! {
    /// P(s,x,y) called with s = 0..=30
    pub fn P(s: usize, x: f64, y: f64) -> f64 {
        let s_ = s as f64;
        let term00 = 2. * sin(5. * s_) * x;
        let term01 = 2. * cos(5. * s_) * y;
        let term02 = 3. * cos(5. * s_);
        let term030 = 14. * x - 19. * y + 5. * s_;
        let term03 = 3. * cos(term030) / 200.;
        let term0 = term00 - term01 + term02 + term03;
        let result = arctan(tan(term0));
        result
    }
}

memo_many! {
    /// Q(s,x,y) called with s = 0..=30
    pub fn Q(s: usize, x: f64, y: f64) -> f64 {
        let s_ = s as f64;
        let term001 = cos(5. * s_) * x;
        let term002 = sin(5. * s_) * y;
        let term003 = 2. * cos(4. * s_);
        let term00 = term001 + term002 + term003;
        let term010 = 18. * x + 15. * y + 4. * s_;
        let term01 = 3. * cos(term010) / 200.;
        let term0 = 2. * term00 + term01;
        let result = arctan(tan(term0));
        result
    }
}

memo_many! {
    pub fn W(x: f64, y: f64) -> f64 {
        let result = sum(1, 40, |s| {
            let term000 = 28.0f64.powf(s) * 25.0f64.powf(-s);
            let term001 = cos(2. * s) * x + sin(2. * s) * y;
            let term002 = 2. * sin(5. * s);
            let term00 = term000 * term001 + term002;
            let term010 = 28.0f64.powf(s) * 25.0f64.powf(-s);
            let term011 = cos(2. * s) * y - sin(2. * s) * x;
            let term012 = 2. * sin(6. * s);
            let term01 = term010 * term011 + term012;
            let term02 = 97. / 100.;
            let term0 = cos2(term00) * cos2(term01) - term02;
            let result = e(-e(-3. * term0));
            result
        });
        result
    }
}
