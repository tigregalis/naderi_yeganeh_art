#![feature(fn_traits)]
#![feature(unboxed_closures)]
#![feature(tuple_trait)]

pub mod memo;
pub use memo::*;

pub mod run;
pub use run::*;

pub mod sunflower_field;

pub mod utils;
pub use utils::*;

pub mod winit_app;
pub use winit_app::*;

pub trait Art {
    const FULL_M: usize;
    const FULL_N: usize;
    fn draw(m: f64, n: f64) -> (u8, u8, u8);
}
