#![feature(fn_traits)]
#![feature(unboxed_closures)]
#![feature(tuple_trait)]
#![feature(sort_floats)]

pub mod art;

pub mod run;
pub use run::*;

pub mod utils;
pub use utils::memo::*;
pub use utils::*;

pub trait Art {
    const FULL_M: usize;
    const FULL_N: usize;
    fn draw(m: f64, n: f64) -> (u8, u8, u8);
}
