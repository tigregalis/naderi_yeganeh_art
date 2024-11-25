#[derive(Clone, Copy)]
pub struct Func<FArgs, FOutput, FFunc> {
    args: Option<FArgs>,
    output: Option<FOutput>,
    f: FFunc,
}

impl<FArgs, FOutput, FFunc> Func<FArgs, FOutput, FFunc> {
    pub const fn new(f: FFunc) -> Self {
        Self {
            args: None,
            output: None,
            f,
        }
    }
}

impl<FArgs, FOutput, FFunc> FnOnce<FArgs> for Func<FArgs, FOutput, FFunc>
where
    FArgs: std::marker::Tuple + PartialEq + Clone,
    FOutput: Clone,
    FFunc: FnOnce<FArgs, Output = FOutput>,
    Self: Clone,
{
    type Output = FOutput;

    extern "rust-call" fn call_once(self, args: FArgs) -> Self::Output {
        self.f.call_once(args)
    }
}

impl<FArgs, FOutput, FFunc> FnMut<FArgs> for Func<FArgs, FOutput, FFunc>
where
    FArgs: std::marker::Tuple + PartialEq + Clone,
    FOutput: Clone,
    FFunc: FnOnce<FArgs, Output = FOutput>,
    Self: Clone,
{
    extern "rust-call" fn call_mut(&mut self, args: FArgs) -> Self::Output {
        match self {
            Func {
                args: Some(self_args),
                output: Some(output),
                ..
            } if *self_args == args => output.clone(),
            _ => {
                let output = self.clone().call_once(args.clone());
                self.args = Some(args);
                self.output = Some(output.clone());
                output
            }
        }
    }
}

#[macro_export]
macro_rules! memo {
    ( $(#[$attr:meta])* $vis:vis fn $name:ident ( $($arg:ident : $argty:ty),* ) -> $outty:ty { $($body:tt)* } ) => {
        #[allow(non_snake_case)]
        $(#[$attr])* $vis fn $name ( $($arg:$argty),* ) -> $outty {
            use std::cell::Cell;
            use $crate::memo::Func;

            fn inner ( $($arg:$argty),* ) -> $outty { $($body)* }

            thread_local! {
                pub static INNER: Cell<Option<Func<($($argty),*), $outty, fn($($argty),*) -> $outty>>> = Cell::new(Some(Func::new(inner)));
            }

            let mut f = INNER.take().unwrap();
            let result = f($($arg),*);
            INNER.set(Some(f));
            result
        }
    };
}
