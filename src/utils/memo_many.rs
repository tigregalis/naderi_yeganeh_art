pub trait PrefixTraits: Clone + Copy + PartialEq {}
impl<T> PrefixTraits for T where T: Clone + Copy + PartialEq {}

pub trait SplitArgs: Clone + Copy {
    type Prefix: PrefixTraits;
    fn split(&self) -> (Self::Prefix, f64, f64);
}

impl SplitArgs for (f64, f64) {
    type Prefix = ();

    fn split(&self) -> (Self::Prefix, f64, f64) {
        let (x, y) = *self;
        ((), x, y)
    }
}

impl<T0: PrefixTraits> SplitArgs for (T0, f64, f64) {
    type Prefix = (T0,);

    fn split(&self) -> (Self::Prefix, f64, f64) {
        let (a, x, y) = *self;
        ((a,), x, y)
    }
}

impl<T0: PrefixTraits, T1: PrefixTraits> SplitArgs for (T0, T1, f64, f64) {
    type Prefix = (T0, T1);

    fn split(&self) -> (Self::Prefix, f64, f64) {
        let (a, b, x, y) = *self;
        ((a, b), x, y)
    }
}

/// Specialised for functions suffixed with `x: f64` and `y: f64` parameters.
/// Keeps an entry for every prefix argument.
/// Resets on new `x` and `y` coordinates.
pub struct MemoManyFunc<FArgs, FOutput, FFunc>
where
    FArgs: SplitArgs,
{
    map: Vec<(<FArgs as SplitArgs>::Prefix, FOutput)>,
    x: Option<f64>,
    y: Option<f64>,
    f: FFunc,
}

impl<FArgs, FOutput, FFunc> MemoManyFunc<FArgs, FOutput, FFunc>
where
    FArgs: SplitArgs,
{
    pub fn new(f: FFunc) -> Self {
        Self {
            map: Vec::with_capacity(256),
            x: None,
            y: None,
            f,
        }
    }

    fn reset_if_new_position(&mut self, x: f64, y: f64) {
        if self.x != Some(x) || self.y != Some(y) {
            self.map.clear();
            self.x = Some(x);
            self.y = Some(y);
        }
    }
}

impl<FArgs, FOutput, FFunc> FnOnce<FArgs> for MemoManyFunc<FArgs, FOutput, FFunc>
where
    FArgs: std::marker::Tuple + SplitArgs,
    FOutput: Clone,
    FFunc: FnOnce<FArgs, Output = FOutput>,
{
    type Output = FOutput;

    extern "rust-call" fn call_once(self, args: FArgs) -> Self::Output {
        self.f.call_once(args)
    }
}

impl<FArgs, FOutput, FFunc> FnMut<FArgs> for MemoManyFunc<FArgs, FOutput, FFunc>
where
    FArgs: std::marker::Tuple + SplitArgs,
    FOutput: Clone,
    FFunc: FnMut<FArgs, Output = FOutput>,
{
    extern "rust-call" fn call_mut(&mut self, args: FArgs) -> Self::Output {
        let (prefix, x, y) = args.split();
        self.reset_if_new_position(x, y);
        match self
            .map
            .iter()
            .find(|(that_prefix, _)| that_prefix == &prefix)
        {
            Some((_, output)) => output.clone(),
            None => {
                let output = self.f.call_mut(args);
                self.map.push((prefix, output.clone()));
                output
            }
        }
    }
}

/// Specialised for functions suffixed with `x: f64` and `y: f64` parameters.
/// Keeps an entry for every prefix argument.
/// Resets on new `x` and `y` coordinates.
#[macro_export]
macro_rules! memo_many {
    ( $(#[$attr:meta])* $vis:vis fn $name:ident ( $($arg:ident : $argty:ty),* ) -> $outty:ty { $($body:tt)* } ) => {
        #[allow(non_snake_case)]
        $(#[$attr])* $vis fn $name ( $($arg:$argty),* ) -> $outty {
            use std::cell::Cell;
            use $crate::memo_many::MemoManyFunc;

            fn inner ( $($arg:$argty),* ) -> $outty { $($body)* }

            thread_local! {
                pub static INNER: Cell<Option<MemoManyFunc<($($argty),*), $outty, fn($($argty),*) -> $outty>>> = Cell::new(Some(MemoManyFunc::new(inner)));
            }

            with_local_cell(&INNER, |f| {
                let f = f.as_mut().expect("function should exist; the thread may have crashed");
                f($($arg),*)
            })
        }
    };
}
