pub trait ArgsTraits: Clone + Copy + PartialEq {}
impl<T> ArgsTraits for T where T: Clone + Copy + PartialEq {}

/// Keeps exactly one entry only.
/// Resets on any change in arguments.
pub struct MemoOnceFunc<FArgs, FOutput, FFunc> {
    args: Option<FArgs>,
    output: Option<FOutput>,
    f: FFunc,
}

impl<FArgs, FOutput, FFunc> MemoOnceFunc<FArgs, FOutput, FFunc> {
    pub const fn new(f: FFunc) -> Self {
        Self {
            args: None,
            output: None,
            f,
        }
    }
}

impl<FArgs, FOutput, FFunc> FnOnce<FArgs> for MemoOnceFunc<FArgs, FOutput, FFunc>
where
    FArgs: std::marker::Tuple + ArgsTraits,
    FOutput: Clone,
    FFunc: FnOnce<FArgs, Output = FOutput>,
{
    type Output = FOutput;

    extern "rust-call" fn call_once(self, args: FArgs) -> Self::Output {
        self.f.call_once(args)
    }
}

impl<FArgs, FOutput, FFunc> FnMut<FArgs> for MemoOnceFunc<FArgs, FOutput, FFunc>
where
    FArgs: std::marker::Tuple + ArgsTraits,
    FOutput: Clone,
    FFunc: FnMut<FArgs, Output = FOutput>,
{
    extern "rust-call" fn call_mut(&mut self, args: FArgs) -> Self::Output {
        match self {
            MemoOnceFunc {
                args: Some(self_args),
                output: Some(output),
                ..
            } if *self_args == args => output.clone(),
            _ => {
                let output = self.call_mut(args);
                self.args = Some(args);
                self.output = Some(output.clone());
                output
            }
        }
    }
}

/// Keeps exactly one entry only.
/// Resets on any change in arguments.
#[macro_export]
macro_rules! memo {
    ( $(#[$attr:meta])* $vis:vis fn $name:ident ( $($arg:ident : $argty:ty),* ) -> $outty:ty { $($body:tt)* } ) => {
        #[allow(non_snake_case)]
        $(#[$attr])* $vis fn $name ( $($arg:$argty),* ) -> $outty {
            use std::cell::Cell;
            use $crate::memo::MemoOnceFunc;

            fn inner ( $($arg:$argty),* ) -> $outty { $($body)* }

            thread_local! {
                pub static INNER: Cell<Option<MemoOnceFunc<($($argty),*), $outty, fn($($argty),*) -> $outty>>> = Cell::new(Some(MemoOnceFunc::new(inner)));
            }

            with_local_cell(&INNER, |f| {
                let f = f.as_mut().unwrap();
                f($($arg),*)
            })
        }
    };
}
