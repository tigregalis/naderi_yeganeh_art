#[derive(Debug)]
pub enum Item {
    /// Name of function `F` in `F(v,x)`, e.g. `Item::Start("F")`
    Start(&'static str),
    /// Name of argument and its value `v` in `F(v,x)` where` `v: usize`, e.g. `Item::ArgUsize("v", 0)`
    ArgUsize(&'static str, usize),
    /// Name of argument and its value `x` in `F(v,x)` where` `x: f64`, e.g. `Item::ArgF64("x", 0.5)`
    ArgF64(&'static str, f64),
    FinishArg,
    FinishU8U8U8(u8, u8, u8),
    /// Result `this` in `let this = F(v, x);`, e.g. `Item::Finish(0.9)`
    FinishF64(f64),
}

impl From<(&'static str, f64)> for Item {
    fn from((name, value): (&'static str, f64)) -> Self {
        Self::ArgF64(name, value)
    }
}

impl From<(&'static str, usize)> for Item {
    fn from((name, value): (&'static str, usize)) -> Self {
        Self::ArgUsize(name, value)
    }
}

impl From<(u8, u8, u8)> for Item {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self::FinishU8U8U8(r, g, b)
    }
}

impl From<f64> for Item {
    fn from(v: f64) -> Self {
        Self::FinishF64(v)
    }
}

use std::cell::Cell;

use super::with_local_cell;

thread_local! {
    pub static STACK: Cell<Vec<Item>> = Cell::new(Vec::with_capacity(400_000));
    pub static SHOULD_TRACK: Cell<bool> = const { Cell::new(false) };
}

#[macro_export]
macro_rules! track {
    ( $(#[$attr:meta])* $vis:vis fn $name:ident ( $($arg:ident : $argty:ty),* ) -> $outty:ty { $($body:tt)* } ) => {
        #[allow(non_snake_case)]
        $(#[$attr])* $vis fn $name ( $($arg:$argty),* ) -> $outty {

            use $crate::utils::track::{push_stack, should_track, Item};

            fn inner ( $($arg:$argty),* ) -> $outty { $($body)* }

            if should_track() {
                // Push function name
                push_stack(Item::Start(stringify!($name)));
                // Push arguments
                $( push_stack((stringify!($arg), $arg).into()); )*
                push_stack(Item::FinishArg);
                // Call function
                let output = inner ( $($arg),* );
                // Push output (finish)
                push_stack(output.into());
                output
            } else {
                inner ( $($arg),* )
            }
        }
    };
}

pub fn push_stack(i: Item) {
    with_local_cell(&STACK, |stack| {
        stack.push(i);
    });
}

pub fn with_stack<O>(f: impl FnOnce(&mut Vec<Item>) -> O) -> O {
    with_local_cell(&STACK, f)
}

pub fn set_should_track(track: bool) {
    with_local_cell(&SHOULD_TRACK, |should_track| *should_track = track);
}

pub fn should_track() -> bool {
    with_local_cell(&SHOULD_TRACK, |should_track| *should_track)
}
