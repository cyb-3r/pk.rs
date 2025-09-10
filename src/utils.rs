//! Utilities for various things

/// Checks if a given value is inside of a given range
pub fn in_range<T>(val: &T, min: T, max: T) -> bool
where
    T: PartialOrd,
{
    *val >= min && *val < max
}
