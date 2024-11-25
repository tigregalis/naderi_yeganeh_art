//! https://x.com/naderi_yeganeh/status/1858455441782534161
//! Sunflower Field by Hamid Naderi Yeganeh

#![allow(non_snake_case, clippy::let_and_return)]

use crate::utils::*;
use crate::*;
use core::f64;
use std::f64::consts::{FRAC_1_PI, PI};

#[inline(always)]
pub fn draw(m: f64, n: f64) -> (u8, u8, u8) {
    let result = rgb(
        F(H0((m - HALF_M) / HALF_N, (HALF_N_PLUS_ONE - n) / HALF_N)),
        F(H1((m - HALF_M) / HALF_N, (HALF_N_PLUS_ONE - n) / HALF_N)),
        F(H2((m - HALF_M) / HALF_N, (HALF_N_PLUS_ONE - n) / HALF_N)),
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
pub fn H0(x: f64, y: f64) -> f64 {
    let v = 0.;
    let term0 = A0(x, y); // A is the flower field
    let term1 = U(60., x, y); // U is the sky/cloud z-depth
    let term20 = B0(x, y); // B is the clouds
    let term21num = 2. - v; // sky/cloud tint hue: (2 - 0)/40 = 2/40 red
    let term21den = 40.;
    let term22num = 3. * v.pow2() - 3. * v + 14.; // sky hue: (3*0 - 3*0 + 14)/20 = 14/20 = 70% red
    let term22den = 20.;
    let term23 = V(20., x, y);
    let term240 = -100. * y - 3. * (x - 1. / 2.).pow2() + 14.;
    let term24 = e(-e(term240));
    let term2 = term20 + term21num / term21den + term22num / term22den * term23 * term24;
    let result = term0 + term1 * term2;
    result
}

/// H(v,x,y) called with v = [0, 1, 2]
pub fn H1(x: f64, y: f64) -> f64 {
    let v = 1.;
    let term0 = A1(x, y); // A is the flower field
    let term1 = U(60., x, y); // U is the sky/cloud z-depth
    let term20 = B1(x, y); // B is the clouds
    let term21num = 2. - v; // sky/cloud tint hue: (2 - 1)/40 = 1/40 green
    let term21den = 40.;
    let term22num = 3. * v.pow2() - 3. * v + 14.; // sky hue: (3*1 - 3*1 + 14)/20 = 14/20 = 70% green
    let term22den = 20.;
    let term23 = V(20., x, y);
    let term240 = -100. * y - 3. * (x - 1. / 2.).pow2() + 14.;
    let term24 = e(-e(term240));
    let term2 = term20 + term21num / term21den + term22num / term22den * term23 * term24;
    let result = term0 + term1 * term2;
    result
}

/// H(v,x,y) called with v = [0, 1, 2]
pub fn H2(x: f64, y: f64) -> f64 {
    let v = 2.;
    let term0 = A2(x, y); // A is the flower field
    let term1 = U(60., x, y); // U is the sky/cloud z-depth
    let term20 = B2(x, y); // B is the clouds
    let term21num = 2. - v; // sky/cloud tint hue: (2 - 2)/40 = 0/40 blue
    let term21den = 40.;
    let term22num = 3. * v.pow2() - 3. * v + 14.; // sky hue: (3*4 - 3*2 + 14)/20 = 20/20 = 100% blue
    let term22den = 20.;
    let term23 = V(20., x, y);
    let term240 = -100. * y - 3. * (x - 1. / 2.).pow2() + 14.;
    let term24 = e(-e(term240));
    let term2 = term20 + term21num / term21den + term22num / term22den * term23 * term24;
    let result = term0 + term1 * term2;
    result
}

/// A(v,x,y) called with v = [0, 1, 2]
pub fn A0(x: f64, y: f64) -> f64 {
    let result = sum(1, 60, |s| {
        let term0 = U(s - 1., x, y);
        let term1 = W0(s, x, y);
        term0 * term1
    });
    result
}

/// A(v,x,y) called with v = [0, 1, 2]
pub fn A1(x: f64, y: f64) -> f64 {
    let result = sum(1, 60, |s| {
        let term0 = U(s - 1., x, y);
        let term1 = W1(s, x, y);
        term0 * term1
    });
    result
}

/// A(v,x,y) called with v = [0, 1, 2]
pub fn A2(x: f64, y: f64) -> f64 {
    let result = sum(1, 60, |s| {
        let term0 = U(s - 1., x, y);
        let term1 = W2(s, x, y);
        term0 * term1
    });
    result
}

memo! {
    /// V(s,x,y) called with s = 1..=20
    pub fn V(s: f64, x: f64, y: f64) -> f64 {
        let result = product_with_key("V", 0, s, x, y, |u, x, y| {
            let term0 = 1.;
            let term1 = 9. / 10.;
            let term20 = -100. * (u - 1. / 2.);
            let term2 = e(-e(term20));
            let term3 = R7( u, x, y);
            term0 - term1 * term2 * term3
        });
        result
    }
}

memo! {
    /// W(v,s,x,y) called with v = [0, 1, 2], s = 1..=60
    pub fn W0(s: f64, x: f64, y: f64) -> f64 {
        let v = 0.;
        let term00 = J0(s, x, y);
        let term01 = 1. - J3(s, x, y);
        let term02num = 19. - 9. * v;
        let term02den = 20.;
        let term03num0 = 5. + 6. * v - 2. * v.pow2();
        let term03num1 = K(s, x, y);
        let term03num = 12. + term03num0 * term03num1;
        let term03den = 20.;
        let term04 = J3(s, x, y);
        let term05 = 21. / 20. - 53. * v / 100.;
        let term06num0 = 6. + 6. * v - 2. * v.pow2();
        let term06num1 = K(s, x, y);
        let term06num = 13. + term06num0 * term06num1;
        let term06den = 20.;
        let term0 = term00 * term01 * term02num / term02den * term03num / term03den
            + term04 * term05 * term06num / term06den;
        let term10num = 2. - v;
        let term10den = 10.;
        let term11 = 7. / 10.;
        let term120 = K(s, x, y) - 37. / 100. + E(x, y) / 40.;
        let term12 = e(-e(-40. * term120));
        let term13 = 3. / 10.;
        let term140 = 3. - 20. * K(s, x, y);
        let term14 = e(-e(term140));
        let term1 = term10num / term10den + term11 * term12 + term13 * term14;
        let term20num = 14. - 7. * (v - 1.).pow2();
        let term20den = 100.;
        let term21num = 5. + 4. * P(s, x, y);
        let term21den = 4.;
        let term22num = 5. + E(x, y);
        let term22den = 5.;
        let term23 = 1. - J0(s, x, y);
        let term24 = 1. - J3(s, x, y);
        let term25 = C(s, x, y);
        let term2 = term20num / term20den * term21num / term21den * term22num / term22den
            * term23
            * term24
            * term25;
        let result = term0 * term1 + term2;
        result
    }
}

memo! {
    /// W(v,s,x,y) called with v = [0, 1, 2], s = 1..=60
    pub fn W1(s: f64, x: f64, y: f64) -> f64 {
        let v = 1.;
        let term00 = J0(s, x, y);
        let term01 = 1. - J3(s, x, y);
        let term02num = 19. - 9. * v;
        let term02den = 20.;
        let term03num0 = 5. + 6. * v - 2. * v.pow2();
        let term03num1 = K(s, x, y);
        let term03num = 12. + term03num0 * term03num1;
        let term03den = 20.;
        let term04 = J3(s, x, y);
        let term05 = 21. / 20. - 53. * v / 100.;
        let term06num0 = 6. + 6. * v - 2. * v.pow2();
        let term06num1 = K(s, x, y);
        let term06num = 13. + term06num0 * term06num1;
        let term06den = 20.;
        let term0 = term00 * term01 * term02num / term02den * term03num / term03den
            + term04 * term05 * term06num / term06den;
        let term10num = 2. - v;
        let term10den = 10.;
        let term11 = 7. / 10.;
        let term120 = K(s, x, y) - 37. / 100. + E(x, y) / 40.;
        let term12 = e(-e(-40. * term120));
        let term13 = 3. / 10.;
        let term140 = 3. - 20. * K(s, x, y);
        let term14 = e(-e(term140));
        let term1 = term10num / term10den + term11 * term12 + term13 * term14;
        let term20num = 14. - 7. * (v - 1.).pow2();
        let term20den = 100.;
        let term21num = 5. + 4. * P(s, x, y);
        let term21den = 4.;
        let term22num = 5. + E(x, y);
        let term22den = 5.;
        let term23 = 1. - J0(s, x, y);
        let term24 = 1. - J3(s, x, y);
        let term25 = C(s, x, y);
        let term2 = term20num / term20den * term21num / term21den * term22num / term22den
            * term23
            * term24
            * term25;
        let result = term0 * term1 + term2;
        result
    }
}

memo! {
    /// W(v,s,x,y) called with v = [0, 1, 2], s = 1..=60
    pub fn W2(s: f64, x: f64, y: f64) -> f64 {
        let v = 2.;
        let term00 = J0(s, x, y);
        let term01 = 1. - J3(s, x, y);
        let term02num = 19. - 9. * v;
        let term02den = 20.;
        let term03num0 = 5. + 6. * v - 2. * v.pow2();
        let term03num1 = K(s, x, y);
        let term03num = 12. + term03num0 * term03num1;
        let term03den = 20.;
        let term04 = J3(s, x, y);
        let term05 = 21. / 20. - 53. * v / 100.;
        let term06num0 = 6. + 6. * v - 2. * v.pow2();
        let term06num1 = K(s, x, y);
        let term06num = 13. + term06num0 * term06num1;
        let term06den = 20.;
        let term0 = term00 * term01 * term02num / term02den * term03num / term03den
            + term04 * term05 * term06num / term06den;
        let term10num = 2. - v;
        let term10den = 10.;
        let term11 = 7. / 10.;
        let term120 = K(s, x, y) - 37. / 100. + E(x, y) / 40.;
        let term12 = e(-e(-40. * term120));
        let term13 = 3. / 10.;
        let term140 = 3. - 20. * K(s, x, y);
        let term14 = e(-e(term140));
        let term1 = term10num / term10den + term11 * term12 + term13 * term14;
        let term20num = 14. - 7. * (v - 1.).pow2();
        let term20den = 100.;
        let term21num = 5. + 4. * P(s, x, y);
        let term21den = 4.;
        let term22num = 5. + E(x, y);
        let term22den = 5.;
        let term23 = 1. - J0(s, x, y);
        let term24 = 1. - J3(s, x, y);
        let term25 = C(s, x, y);
        let term2 = term20num / term20den * term21num / term21den * term22num / term22den
            * term23
            * term24
            * term25;
        let result = term0 * term1 + term2;
        result
    }
}

memo! {
    /// the arccos(cos(x)) normalises x within 0 to Pi, i.e.
    /// - arccos(cos(x = 0 to Pi)) = 0 to Pi
    /// - arccos(cos(x = Pi to 2Pi)) = Pi to 0
    /// - arccos(cos(x = 2Pi to 3Pi)) = 0 to Pi
    /// - arccos(cos(x = 3Pi to 4Pi)) = Pi to 0
    /// - ...
    pub fn K(s: f64, x: f64, y: f64) -> f64 {
        let term00 = 10. * P(s, x, y);
        let term0 = arccos(cos(term00)).pow2();
        let term10 = 10. * Q(s, x, y);
        let term1 = arccos(cos(term10)).pow2();
        let result = term0 + term1;
        result
    }
}

memo! {
    /// C(s,x,y) called with s = 0..=60
    pub fn C(s: f64, x: f64, y: f64) -> f64 {
        let term00 = -100.;
        let term01 = s - 1. / 2.;
        let term0 = e(-e(term00 * term01));
        let term10 = -100.;
        let term11 = s - 1. / 2.;
        let term1 = e(-e(term10 * term11));
        let term200 = 10. * Q(s, x, y);
        let term20 = 98. - 100. * cos3(term200);
        let term21 = 50. * P(s, x, y);
        let term22 = -50. * P(s, x, y) - 75.;
        let term2 = 1. - e(-e(term20) - e(term21) - e(term22));
        let term3 = 1. - N(s, x, y);
        let result = term0 - term1 * term2 * term3;
        result
    }
}

memo! {
    pub fn L(s: f64, x: f64, y: f64) -> f64 {
        let term0num0 = 10. * P(s, x, y);
        let term0num = HALF_M * arccos(cos(term0num0));
        let term0den00 = 10. * Q(s, x, y);
        let term0den0 = arccos(cos(term0den00));
        let term0den = 1. + HALF_M * abs(term0den0);
        let result = arctan(term0num / term0den);
        result
    }
}

memo! {
    pub fn Q(s: f64, x: f64, y: f64) -> f64 {
        let term0num = 103.0f64.powf(s);
        let term0den = 100.0f64.powf(s);
        let term1num = 5. + cos(8. * s);
        let term1den = 5.;
        let term20 = cos(3. * s) / 20.;
        let term21 = sin(2. * P(s, x, y));
        let term22 = 4. * cos(5. * s);
        let term2 = x + term20 * term21 + term22;
        let result = term0num / term0den * term1num / term1den * term2;
        result
    }
}

memo! {
    pub fn P(s: f64, x: f64, y: f64) -> f64 {
        let term0num = 103.0f64.powf(s);
        let term0den = 100.0f64.powf(s);
        let term100 = 2. * x - 1.;
        let term10 = 3. * term100.pow2() / 400.;
        let term11 = 1. / 5.;
        let term12 = 19.0f64.powf(s) / 20.0f64.powf(s);
        let term130 = 3. * x + 2. * s;
        let term13 = cos(term130) / 20.;
        let term1 = y + term10 - term11 + term12 + term13;
        let result = term0num / term0den * term1;
        result
    }
}

/// B(v,x,y) called with v = [0, 1, 2]
pub fn B0(x: f64, y: f64) -> f64 {
    let v = 0.;
    let result = sum(1, 20, |s| {
        let term0 = V(s - 1., x, y);
        let term1 = R7(s, x, y);
        let term2num0 = R3(s, x, y);
        let term2num = 15. - 7. * term2num0;
        let term2den = 10.;
        let term30num = cos(4. * s + v * s) + s;
        let term30den = 40.;
        let term31 = v / 5.;
        let term32num = cos(5. * x + 3. * y + 3. * s);
        let term32den = 10.;
        let term33num = cos(8. * s);
        let term33den = 5.;
        let term3 =
            term30num / term30den - term31 + y + term32num / term32den + term33num / term33den;
        term0 * term1 * term2num / term2den * term3
    });
    result
}

/// B(v,x,y) called with v = [0, 1, 2]
pub fn B1(x: f64, y: f64) -> f64 {
    let v = 1.;
    let result = sum(1, 20, |s| {
        let term0 = V(s - 1., x, y);
        let term1 = R7(s, x, y);
        let term2num0 = R3(s, x, y);
        let term2num = 15. - 7. * term2num0;
        let term2den = 10.;
        let term30num = cos(4. * s + v * s) + s;
        let term30den = 40.;
        let term31 = v / 5.;
        let term32num = cos(5. * x + 3. * y + 3. * s);
        let term32den = 10.;
        let term33num = cos(8. * s);
        let term33den = 5.;
        let term3 =
            term30num / term30den - term31 + y + term32num / term32den + term33num / term33den;
        term0 * term1 * term2num / term2den * term3
    });
    result
}

/// B(v,x,y) called with v = [0, 1, 2]
pub fn B2(x: f64, y: f64) -> f64 {
    let v = 2.;
    let result = sum(1, 20, |s| {
        let term0 = V(s - 1., x, y);
        let term1 = R7(s, x, y);
        let term2num0 = R3(s, x, y);
        let term2num = 15. - 7. * term2num0;
        let term2den = 10.;
        let term30num = cos(4. * s + v * s) + s;
        let term30den = 40.;
        let term31 = v / 5.;
        let term32num = cos(5. * x + 3. * y + 3. * s);
        let term32den = 10.;
        let term33num = cos(8. * s);
        let term33den = 5.;
        let term3 =
            term30num / term30den - term31 + y + term32num / term32den + term33num / term33den;
        term0 * term1 * term2num / term2den * term3
    });
    result
}

memo! {
    /// U(s,x,y) called with s = 0..=60
    pub fn U(s: f64, x: f64, y: f64) -> f64 {
        let result = product_with_key("U", 0, s, x, y, |u, x, y| {
            let term0 = 1. - J0(u, x, y);
            let term1 = 1. - J3(u, x, y);
            let term2 = 1. - C(u, x, y);
            term0 * term1 * term2
        });
        result
    }
}

memo! {
    pub fn E(x: f64, y: f64) -> f64 {
        let result = sum(1, 50, |s| {
            let term0 = 25.0f64.powf(s) / 26.0f64.powf(s);
            let term1 = T(s, x, y);
            term0 * term1
        });
        result
    }
}

memo! {
    /// R(v,s,x,y) called with v = [3, 7]
    pub fn R3(s: f64, x: f64, y: f64) -> f64 {
        let v = 3.;
        let term00 = x + cos(5. * s);
        let term010 = s / 40.;
        let term011num = cos(5. * x + 3. * y + 3. * s);
        let term011den = 10.;
        let term012num = cos(8. * s);
        let term012den = 5.;
        let term01 = y - 1. + term010 + term011num / term011den + term012num / term012den;
        let term02 = cos(6. * s) / 5.;
        let term03 = 3. * E(x, y) / 10.;
        let term0 = term00.pow2() + 20. * term01.pow2() - 2. + term02 + term03;
        let result = e(-e(v * term0));
        result
    }
}

memo! {
    /// R(v,s,x,y) called with v = [3, 7]
    pub fn R7(s: f64, x: f64, y: f64) -> f64 {
        let v = 7.;
        let term00 = x + cos(5. * s);
        let term010 = s / 40.;
        let term011num = cos(5. * x + 3. * y + 3. * s);
        let term011den = 10.;
        let term012num = cos(8. * s);
        let term012den = 5.;
        let term01 = y - 1. + term010 + term011num / term011den + term012num / term012den;
        let term02 = cos(6. * s) / 5.;
        let term03 = 3. * E(x, y) / 10.;
        let term0 = term00.pow2() + 20. * term01.pow2() - 2. + term02 + term03;
        let result = e(-e(v * term0));
        result
    }
}

memo! {
    /// J(v,s,x,y) called with J = [0, 3]
    pub fn J0(s: f64, x: f64, y: f64) -> f64 {
        let v = 0.;
        let term0 = -100.;
        let term1 = s - 1. / 2.;
        let term2 = 50.;
        let term3 = abs(10. * P(s, x, y)) - PI;
        let term4 = 90.;
        let term50 = K(s, x, y);
        let term51 = 8. / 5.;
        let term52 = 5. / 4.;
        let term530 = 8. + v / 2.;
        let term531 = L(s, x, y);
        let term532num0 = 5. * K(s, x, y) + 6. * s;
        let term532num = 9. * cos(term532num0);
        let term532den = 50.;
        let term533 = K(s, x, y) / 2.;
        let term5340 = 3. * L(s, x, y) + 4. * s;
        let term534 = cos(term5340);
        let term535 = 4. * s;
        let term536 = 5. * v / 3.;
        let term53 =
            term530 * term531 + term532num / term532den + term533 * term534 + term535 + term536;
        let term5 = term50 - term51 + term52 * cos2(term53);
        let result = e(-e(term0 * term1) - e(term2 * term3) - e(term4 * term5));
        result
    }
}

memo! {
    /// J(v,s,x,y) called with J = [0, 3]
    pub fn J3(s: f64, x: f64, y: f64) -> f64 {
        let v = 3.;
        let term0 = -100.;
        let term1 = s - 1. / 2.;
        let term2 = 50.;
        let term3 = abs(10. * P(s, x, y)) - PI;
        let term4 = 90.;
        let term50 = K(s, x, y);
        let term51 = 8. / 5.;
        let term52 = 5. / 4.;
        let term530 = 8. + v / 2.;
        let term531 = L(s, x, y);
        let term532num0 = 5. * K(s, x, y) + 6. * s;
        let term532num = 9. * cos(term532num0);
        let term532den = 50.;
        let term533 = K(s, x, y) / 2.;
        let term5340 = 3. * L(s, x, y) + 4. * s;
        let term534 = cos(term5340);
        let term535 = 4. * s;
        let term536 = 5. * v / 3.;
        let term53 =
            term530 * term531 + term532num / term532den + term533 * term534 + term535 + term536;
        let term5 = term50 - term51 + term52 * cos2(term53);
        let result = e(-e(term0 * term1) - e(term2 * term3) - e(term4 * term5));
        result
    }
}

memo! {
    pub fn N(s: f64, x: f64, y: f64) -> f64 {
        let term00 = -400.;
        let term010 = 20. * P(s, x, y);
        let term011num0 = 30. * x + 24. * y + 7. * s;
        let term011num = cos(term011num0);
        let term011den = 2.;
        let term012 = 2.;
        let term0130 = 10. * Q(s, x, y);
        let term013 = arccos(cos(term0130));
        let term01 = term010 + term011num / term011den - term012 * term013;
        let term02 = 160.;
        let term03 = 40. * E(x, y);
        let term040 = 10. * Q(s, x, y);
        let term04 = arccos(cos(term040));
        let term050 = arccos(cos(10. * Q(s, x, y)));
        let term05 = 1. - FRAC_1_PI * term050;
        let term0 = term00 * cos(term01) - term02 - term03 + HALF_N * term04 + HALF_N * term05.pow10();
        let term10 = 200.;
        let term110 = P(s, x, y);
        let term111 = 1. / 10.;
        let term1120 = 10. * Q(s, x, y);
        let term112 = arccos(cos(term1120));
        let term113 = 2. / 5.;
        let term11 = abs(term110 - term111 * term112 + term113);
        let term12 = 40.;
        let term1 = term10 * term11 - term12;
        let result = e(-e(term0) - e(term1));
        result
    }
}

memo! {
    pub fn T(s: f64, x: f64, y: f64) -> f64 {
        let term00 = 10.0f64.powneg1() * 23.0f64.powf(s) * 20.0f64.powf(-s);
        let term01 = 1. + cos(10. * s);
        let term020 = cos(7. * s) * x;
        let term021 = sin(7. * s) * y;
        let term022 = 2. * cos(17. * s);
        let term02 = term020 + term021 + term022;
        let term03 = 4.;
        let term040 = 10.0f64.powneg1() * 23.0f64.powf(s) * 20.0f64.powf(-s);
        let term0410 = cos(9. * s) * x;
        let term0411 = sin(9. * s) * y;
        let term041 = term0410 + term0411;
        let term04 = cos(term040 * term041);
        let term05 = 2. * cos(5. * s);
        let term0 = cos(term00 * term01 * term02 + term03 * term04 + term05);
        let term10 = 10.0f64.powneg1() * 23.0f64.powf(s) * 20.0f64.powf(-s);
        let term11 = 1. + cos(10. * s);
        let term120 = cos(7. * s) * y;
        let term121 = sin(7. * s) * x;
        let term122 = 2. * cos(15. * s);
        let term12 = term120 - term121 + term122;
        let term13 = 4.;
        let term140 = 10.0f64.powneg1() * 23.0f64.powf(s) * 20.0f64.powf(-s);
        let term1410 = cos(8. * s) * x;
        let term1411 = sin(8. * s) * y;
        let term141 = term1410 + term1411;
        let term14 = cos(term140 * term141);
        let term15 = 2. * cos(7. * s);
        let term1 = cos(term10 * term11 * term12 + term13 * term14 + term15);
        let result = term0 * term1;
        result
    }
}
