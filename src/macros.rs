/// Assert that two numbers are almost equal to each other.
///
/// On panic, this macro will print the values of the expressions with their
/// debug representations.
#[macro_export]
macro_rules! assert_almost_eq {
    ($a:expr, $b:expr, $prec:expr) => (
        let diff = ($a - $b).abs();
        if diff > $prec {
            panic!(format!(
                "assertion failed: `abs(left - right) = {:.1e} < {:e}`, \
                 (left: `{}`, right: `{}`)",
                diff, $prec, $a, $b));
        }
    );
}

/// Concatenate several iterative estimators into one.
///
/// `$name` is the name of the new type. `$statistic` is the name of a statistic
/// and must exist as a method of the corresponding type `$estimator`.
/// `$estimator` must have an `add` method for adding new observations to the
/// sample (taking an `f64` as an argument). It must also implement `Default`.
///
/// For moments, only an estimator for the highest moment should be used and
/// reused for the lower moments. This is currently not supported by this macro
/// and has to be done by hand.
///
///
/// # Example
///
/// ```
/// # extern crate core;
/// # #[macro_use] extern crate average;
/// # fn main() {
/// use average::{Min, Max};
///
/// concatenate!(MinMax, min, Min, max, Max);
///
/// let s: MinMax = (1..6).map(Into::into).collect();
///
/// assert_eq!(s.min(), 1.0);
/// assert_eq!(s.max(), 5.0);
/// # }
/// ```
///
/// The generated code looks roughly like this:
///
/// ```
/// # use average::{Min, Max};
/// #
/// struct MinMax {
///     min: Min,
///     max: Max,
/// }
///
/// impl MinMax {
///     pub fn new() -> MinMax {
///         MinMax { min: Min::default(), max: Max::default() }
///     }
///
///     pub fn add(&mut self, x: f64) {
///         self.min.add(x);
///         self.max.add(x);
///     }
///
///     pub fn min(&self) -> f64 {
///         self.min.min()
///     }
///
///     pub fn max(&self) -> f64 {
///         self.max.max()
///     }
/// }
/// ```
#[macro_export]
macro_rules! concatenate {
    ( $name:ident, $($statistic:ident, $estimator:ident),* ) => {
        struct $name {
        $(
            $statistic: $estimator,
        )*
        }

        impl $name {
            pub fn new() -> $name {
                $name {
                $(
                    $statistic: ::core::default::Default::default(),
                )*
                }
            }

            pub fn add(&mut self, x: f64) {
                $(
                    self.$statistic.add(x);
                )*
            }

            $(
                pub fn $statistic(&self) -> f64 {
                    self.$statistic.$statistic()
                }
            )*
        }

        impl Default for $name {
            fn default() -> $name {
                $name::new()
            }
        }

        impl ::core::iter::FromIterator<f64> for $name {
            fn from_iter<T>(iter: T) -> $name
                where T: IntoIterator<Item=f64>
            {
                let mut e = $name::new();
                for i in iter {
                    e.add(i);
                }
                e
            }
        }
    };
}
