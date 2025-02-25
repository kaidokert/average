#![cfg_attr(feature = "nightly",
   feature(generic_const_exprs))]

#![allow(clippy::float_cmp)]

mod histogram;
#[cfg(feature = "nightly")]
mod histogram_const;
#[cfg(any(feature = "std", feature = "libm"))]
mod kurtosis;
mod macros;
mod max;
mod mean;
mod min;
mod moments;
#[cfg(feature = "std")]
mod proptest;
#[cfg(any(feature = "std", feature = "libm"))]
mod quantile;
#[cfg(any(feature = "std", feature = "libm"))]
mod random;
#[cfg(any(feature = "std", feature = "libm"))]
mod skewness;
#[cfg(feature = "std")]
mod streaming_stats;
mod weighted_mean;
