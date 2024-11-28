use std::{
    cell::Cell,
    collections::{HashMap, HashSet},
    thread::LocalKey,
};

pub mod memo_many;
pub mod memo_once;
pub mod winit_app;

#[inline(always)]
pub fn sqrt(n: f64) -> f64 {
    n.sqrt()
}

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
pub fn tan(n: f64) -> f64 {
    n.tan()
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

pub trait Number {
    fn into_usize(self) -> usize;
    fn into_f64(self) -> f64;
}

impl Number for usize {
    #[inline(always)]
    fn into_usize(self) -> usize {
        self
    }

    #[inline(always)]
    fn into_f64(self) -> f64 {
        self as f64
    }
}

impl Number for f64 {
    #[inline(always)]
    fn into_usize(self) -> usize {
        self as usize
    }

    #[inline(always)]
    fn into_f64(self) -> f64 {
        self
    }
}

pub trait Powers {
    fn powneg1(self) -> Self;
    fn pow2(self) -> Self;
    fn pow8(self) -> Self;
    fn pow10(self) -> Self;
}

impl Powers for f64 {
    /// Computes `self^-1`.
    /// Theoretically faster than `self.powf(-1.)` and `self.powi(-1)`.
    #[inline(always)]
    fn powneg1(self) -> Self {
        1.0 / self
    }

    /// Computes `self^2`.
    /// Theoretically faster than `self.powf(2.)` and `self.powi(2)`.
    #[inline(always)]
    fn pow2(self) -> Self {
        self * self
    }

    /// Computes `self^8`.
    /// Theoretically faster than `self.powf(8.)` and `self.powi(8)`.
    #[inline(always)]
    fn pow8(self) -> Self {
        self * self * self * self * self * self * self * self
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
pub fn range(start: impl Number, end: impl Number) -> impl Iterator<Item = f64> {
    let start = start.into_usize();
    let end = end.into_usize();
    (start..=end).map(|n| n as f64)
}

#[inline(always)]
pub fn sum_with_key(
    key: &'static str,
    start: impl Number,
    end: impl Number,
    x: f64,
    y: f64,
    expression: impl Fn(f64, f64, f64) -> f64,
) -> f64 {
    thread_local! {
        static ARGS: Cell<Option<(f64, f64)>> = Default::default();
        static MAP: Cell<HashMap<&'static str, Vec<f64>>> = Default::default();
    }
    let start = start.into_usize();
    let end = end.into_usize();
    // TODO:
    // fetch or create cached value, invalidating if x and y are different
    // invalidate cache if x and y are different
    with_local_cell(&ARGS, |args| {
        if *args != Some((x, y)) {
            *args = Some((x, y));
            with_local_cell(&MAP, |map| map.clear());
        }
    });
    // fetch or compute
    let sum = with_local_cell(&MAP, move |map| {
        let v = map
            .entry(key)
            .and_modify(|v| {
                for s in v.len()..=end {
                    v.push(expression(s as f64, x, y));
                }
            })
            .or_insert_with(|| range(0, end).map(|s| expression(s, x, y)).collect());

        v[start..end].iter().sum::<f64>()
    });
    sum
}

#[inline(always)]
pub fn sum(start: impl Number, end: impl Number, expression: impl Fn(usize) -> f64) -> f64 {
    (start.into_usize()..=end.into_usize())
        .map(expression)
        .sum::<f64>()
}

#[inline(always)]
pub fn product_with_key(
    key: &'static str,
    start: impl Number,
    end: impl Number,
    x: f64,
    y: f64,
    expression: impl Fn(usize, f64, f64) -> f64,
) -> f64 {
    thread_local! {
        static ARGS: Cell<Option<(f64, f64)>> = Default::default();
        static MAP: Cell<HashMap<&'static str, Vec<f64>>> = Default::default();
    }
    let start = start.into_usize();
    let end = end.into_usize();
    // invalidate cache if x and y are different
    with_local_cell(&ARGS, |args| {
        if *args != Some((x, y)) {
            *args = Some((x, y));
            with_local_cell(&MAP, |map| map.clear());
        }
    });
    // fetch or compute
    let product = with_local_cell(&MAP, move |map| {
        let v = map
            .entry(key)
            .and_modify(|v| {
                for s in v.len()..=end {
                    v.push(expression(s, x, y));
                }
            })
            .or_insert_with(|| (0..=end).map(|s| expression(s, x, y)).collect());

        v[start..=end].iter().product::<f64>()
    });
    product
}

#[inline(always)]
pub fn product(start: impl Number, end: impl Number, expression: impl Fn(f64) -> f64) -> f64 {
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

pub fn with_local_cell<T: Default, O>(
    cell: &'static LocalKey<Cell<T>>,
    f: impl FnOnce(&mut T) -> O,
) -> O {
    let mut value = cell.take();
    let output = f(&mut value);
    cell.set(value);
    output
}

thread_local! {
    static DEBUG_CONTEXT: Cell<(bool, HashSet<u64>)> = Default::default();
}

pub fn debug_store_value(v: f64) {
    with_local_cell(&DEBUG_CONTEXT, |(_, set)| {
        set.insert(v.to_bits());
    });
}

pub fn debug_print_stored_values() {
    with_local_cell(&DEBUG_CONTEXT, |(logged, set)| {
        if !*logged {
            let mut values = set
                .iter()
                .map(|value| f64::from_bits(*value))
                .collect::<Vec<_>>();
            values.sort_floats();
            dbg!(values);
            *logged = true;
        }
    });
}
