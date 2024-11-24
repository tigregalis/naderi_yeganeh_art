/// Computes e^n
#[inline(always)]
pub fn e(n: f64) -> f64 {
    n.exp()
}

/// Computes |n|
#[inline(always)]
pub fn abs(n: f64) -> f64 {
    n.abs()
}

#[inline(always)]
pub fn arccos(n: f64) -> f64 {
    n.acos()
}

#[inline(always)]
pub fn cos(n: f64) -> f64 {
    n.cos()
}

#[inline(always)]
pub fn sin(n: f64) -> f64 {
    n.sin()
}

#[inline(always)]
pub fn arctan(n: f64) -> f64 {
    n.atan()
}

/// https://calculus.subwiki.org/wiki/Cosine-cubed_function
#[inline(always)]
pub fn cos3(x: f64) -> f64 {
    let y = x.cos();
    y * y * y
}

/// https://calculus.subwiki.org/wiki/Cosine-squared_function
#[inline(always)]
pub fn cos2(x: f64) -> f64 {
    let y = x.cos();
    y * y
}

pub trait IntoUsize {
    fn into_usize(self) -> usize;
}

impl IntoUsize for usize {
    #[inline(always)]
    fn into_usize(self) -> usize {
        self
    }
}

impl IntoUsize for f64 {
    #[inline(always)]
    fn into_usize(self) -> usize {
        self as usize
    }
}

pub trait Powers {
    fn pow2(self) -> Self;
    fn powneg1(self) -> Self;
    fn pow10(self) -> Self;
}

impl Powers for f64 {
    /// Computes `self^2`.
    /// Theoretically faster than `self.powf(2.)` and `self.powi(2)`.
    #[inline(always)]
    fn pow2(self) -> Self {
        self * self
    }

    /// Computes `self^-1`.
    /// Theoretically faster than `self.powf(-1.)` and `self.powi(-1)`.
    #[inline(always)]
    fn powneg1(self) -> Self {
        1.0 / self
    }

    /// Computes `self^10`.
    /// Theoretically faster than `self.powf(10.)` and `self.powi(10)`.
    #[inline(always)]
    fn pow10(self) -> Self {
        self * self * self * self * self * self * self * self * self * self
    }
}

/// Computes inclusive range as f64
#[inline(always)]
pub fn range(start: impl IntoUsize, end: impl IntoUsize) -> impl Iterator<Item = f64> {
    let start = start.into_usize();
    let end = end.into_usize();
    (start..=end).map(|n| n as f64)
}

#[inline(always)]
pub fn sum(start: impl IntoUsize, end: impl IntoUsize, expression: impl Fn(f64) -> f64) -> f64 {
    range(start, end).map(expression).sum::<f64>()
}

#[inline(always)]
pub fn product(start: impl IntoUsize, end: impl IntoUsize, expression: impl Fn(f64) -> f64) -> f64 {
    range(start, end).map(expression).product::<f64>()
}

/// Softbuffer uses an ARGB representation
#[inline(always)]
pub fn softbuffer_color(rgb: (u8, u8, u8)) -> u32 {
    let (r, g, b) = rgb;
    u32::from_be_bytes([255, r, g, b])
}

pub fn xy_from_index(width: usize, index: usize) -> (usize, usize) {
    let x = index % width;
    let y = index / width;
    (x, y)
}
