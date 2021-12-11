//! truncate-integer: integer truncation for Rust
//!
//! There are several ways one might want to do integer truncation in Rust:
//!
//! - Unchecked: truncation may result in a changed value. You only get
//! the low-order N bits.
//! - Checked: if truncation would result in a changed value, return `None`,
//! otherwise `Some(value)`.
//! - Panicking: if truncation would result in a changed value, 'panic!'
//! This is equivalent to checked truncation with `.unwrap()`, but with a nicer
//! panic message.
//! - Saturating: if truncation would result in a changed value, return
//! the maximum value that would fit in the target type.
//!
//! It's possible to get all of these in Rust without importing additional
//! crates or writing much code, for example:
//!
//! ```rust
//! let x = 257u16;
//!
//! let unchecked = x as u8;
//! assert_eq!(unchecked, 1u8);
//!
//! let checked = u8::try_from(x);
//! assert!(checked.is_err());
//!
//! // This would panic
//! // let value = x.try_from().unwrap();
//!
//! let saturating = u8::try_from(x).unwrap_or(u8::MAX);
//! assert_eq!(saturating, 255);
//! ```
//!
//! If those are good enough for you, then you don't need this crate.
//! However, if you would prefer to call a function to communicate your
//! intent, then you might find this crate useful.
//!
//! It provides a trait that implements each of the truncation forms listed above:
//!
//! - [`TruncateUnchecked`] performs unchecked truncation.
//! - [`TryTruncate`] performs checked truncation.
//! - [`Chop`] performs panicking truncation.
//! - [`Shrink`] performs saturating truncation.
//!
//! It's sometimes desirable to invert this logic, e.g. in trait bounds,
//! so there is an inverse of each of the above:
//!
//! - [`TruncateFromUnchecked`]
//! - [`TryTruncateFrom`]
//! - [`ChopFrom`]
//! - [`ShrinkFrom`]
//!
//! All of the truncations are implemented for both signed and unsigned
//! integers (including signed-to-unsigned and vice versa), except
//! `TruncateFromUnchecked`, because it's not immediately clear what the
//! correct output would be when then input is outside the output bounds.

pub trait TryTruncate<T> {
    /// Try to truncate an integer to fit into a smaller type.
    ///
    /// If the value fits into the target type, return `Ok(value)`
    /// Otherwise, return `None`.
    fn try_truncate(self) -> Option<T>;
}

pub trait TryTruncateFrom<T>: Sized {
    /// Try to truncate an integer to fit into a smaller type.
    ///
    /// If the value fits into the `Self` type, return `Ok(value)`
    /// Otherwise, return `None`.
    fn try_truncate_from(value: T) -> Option<Self>;
}

impl<Source, Dest> TryTruncateFrom<Source> for Dest
where
    Source: TryTruncate<Dest>,
{
    fn try_truncate_from(x: Source) -> Option<Self> {
        x.try_truncate()
    }
}

pub trait Chop<T> {
    /// Perform panicking truncation
    ///
    /// If the value fits into the target type, return that value.
    /// Otherwise, panic.
    fn chop(self) -> T;
}

pub trait ChopFrom<T> {
    /// Perform panicking truncation
    ///
    /// If the value fits into the `Self` type, return that value.
    /// Otherwise, panic.
    fn chop_from(value: T) -> Self;
}

impl<Source, Dest> ChopFrom<Source> for Dest
where
    Source: Chop<Dest>,
{
    fn chop_from(x: Source) -> Self {
        x.chop()
    }
}

pub trait TruncateUnchecked<T> {
    /// Perform unchecked bitwise truncation
    ///
    /// If the value fits into the target type, return that value.
    /// Otherwise, return the low-order bits that do fit.
    ///
    /// This has the same result as using `as` to truncate (e.g. `foo as u8`).
    fn truncate_unchecked(self) -> T;
}

pub trait TruncateFromUnchecked<T> {
    /// Perform unchecked bitwise truncation
    ///
    /// If the value fits into the `Self` type, return that value.
    /// Otherwise, return the low-order bits that do fit.
    ///
    /// This has the same result as using `as` to truncate (e.g. `foo as u8`).
    fn truncate_from_unchecked(value: T) -> Self;
}

impl<Source, Dest> TruncateFromUnchecked<Source> for Dest
where
    Source: TruncateUnchecked<Dest>,
{
    fn truncate_from_unchecked(x: Source) -> Self {
        x.truncate_unchecked()
    }
}

/// Perform saturating truncation.
pub trait Shrink<T> {
    /// Perform saturating truncation.
    ///
    /// If the value fits into the target type, return that value.
    /// Otherwise, return the closest value that does fit.
    fn shrink(self) -> T;
}

/// Perform saturating truncation.
pub trait ShrinkFrom<T> {
    /// Perform saturating truncation.
    ///
    /// If the value fits into the `Self` type, return that value.
    /// Otherwise, return the closest value that does fit.
    fn shrink_from(value: T) -> Self;
}

impl<Source, Dest> ShrinkFrom<Source> for Dest
where
    Source: Shrink<Dest>,
{
    fn shrink_from(x: Source) -> Self {
        x.shrink()
    }
}

macro_rules! make_truncate {
    ($Source: ty, $Dest:ty) => {
        impl TryTruncate<$Dest> for $Source {
            #[track_caller]
            #[inline]
            fn try_truncate(self) -> Option<$Dest> {
                use ::core::convert::TryFrom;
                <$Dest>::try_from(self).ok()
            }
        }

        impl Chop<$Dest> for $Source {
            #[track_caller]
            #[inline]
            fn chop(self) -> $Dest {
                use ::core::convert::TryFrom;

                match <$Dest>::try_from(self) {
                    Ok(val) => val,
                    Err(_) => panic!("chop overflow"),
                }
            }
        }

        impl Shrink<$Dest> for $Source {
            #[track_caller]
            #[inline]
            fn shrink(self) -> $Dest {
                use ::core::convert::TryFrom;

                match <$Dest>::try_from(self) {
                    Ok(val) => val,
                    Err(_) => {
                        if self < (<$Dest>::MIN) as $Source {
                            <$Dest>::MIN
                        } else {
                            <$Dest>::MAX
                        }
                    }
                }
            }
        }

    };
}

macro_rules! make_truncate_all {
    ($Source: ty, $Dest:ty) => {
        // FIXME: don't implement this for negative numbers!
        impl TruncateUnchecked<$Dest> for $Source {
            #[track_caller]
            #[inline]
            fn truncate_unchecked(self) -> $Dest {
                self as $Dest
            }
        }

        make_truncate!($Source, $Dest);
    }
}

make_truncate_all!(usize, u8);
make_truncate_all!(usize, u16);
make_truncate_all!(usize, u32);

make_truncate_all!(u128, u8);
make_truncate_all!(u128, u16);
make_truncate_all!(u128, u32);
make_truncate_all!(u128, u64);
make_truncate_all!(u64, u8);
make_truncate_all!(u64, u16);
make_truncate_all!(u64, u32);
make_truncate_all!(u32, u8);
make_truncate_all!(u32, u16);
make_truncate_all!(u16, u8);

make_truncate_all!(u128, i8);
make_truncate_all!(u128, i16);
make_truncate_all!(u128, i32);
make_truncate_all!(u128, i64);
make_truncate_all!(u64, i8);
make_truncate_all!(u64, i16);
make_truncate_all!(u64, i32);
make_truncate_all!(u32, i8);
make_truncate_all!(u32, i16);
make_truncate_all!(u16, i8);

make_truncate!(i128, i64);
make_truncate!(i128, i32);
make_truncate!(i128, i16);
make_truncate!(i128, i8);
make_truncate!(i64, i8);
make_truncate!(i64, i16);
make_truncate!(i64, i32);
make_truncate!(i32, i8);
make_truncate!(i32, i16);
make_truncate!(i16, i8);

make_truncate!(i128, u64);
make_truncate!(i128, u32);
make_truncate!(i128, u16);
make_truncate!(i128, u8);
make_truncate!(i64, u8);
make_truncate!(i64, u16);
make_truncate!(i64, u32);
make_truncate!(i32, u8);
make_truncate!(i32, u16);
make_truncate!(i16, u8);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chop() {
        let x: u8 = 0u16.chop();
        assert_eq!(x, 0u8);
        let x: u8 = 0u32.chop();
        assert_eq!(x, 0u8);
        let x: u8 = 0u64.chop();
        assert_eq!(x, 0u8);
        let x: u8 = 0u128.chop();
        assert_eq!(x, 0u8);

        let x: i8 = 0i16.chop();
        assert_eq!(x, 0i8);
        let x: i8 = 0i32.chop();
        assert_eq!(x, 0i8);
        let x: i8 = 0i64.chop();
        assert_eq!(x, 0i8);
        let x: i8 = 0i128.chop();
        assert_eq!(x, 0i8);

        let x: u8 = 0i16.chop();
        assert_eq!(x, 0u8);
        let x: u8 = 0i32.chop();
        assert_eq!(x, 0u8);
        let x: u8 = 0i64.chop();
        assert_eq!(x, 0u8);
        let x: u8 = 0i128.chop();
        assert_eq!(x, 0u8);

        let x: i8 = 0u16.chop();
        assert_eq!(x, 0i8);
        let x: i8 = 0u32.chop();
        assert_eq!(x, 0i8);
        let x: i8 = 0u64.chop();
        assert_eq!(x, 0i8);
        let x: i8 = 0u128.chop();
        assert_eq!(x, 0i8);
    }

    #[test]
    fn test_try_truncate() {
        let x: Option<u8> = 257u16.try_truncate();
        assert!(x.is_none());
        let x: Option<u8> = (-1i16).try_truncate();
        assert!(x.is_none());

        let x = u8::try_truncate_from(257u16);
        assert!(x.is_none());
    }

    #[test]
    fn test_shrink() {
        let x: u8 = 257u16.shrink();
        assert_eq!(x, 255u8);
        let x: u8 = (-1i16).shrink();
        assert_eq!(x, 0u8);
    }

    #[test]
    fn test_truncate_unchecked() {
        let x: u8 = 257u16.truncate_unchecked();
        assert_eq!(x, 1u8);
    }
}
