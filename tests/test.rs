use truncate_integer::{Chop, TryTruncate, TryTruncateFrom, Shrink, TruncateUnchecked};

#[test]
#[should_panic]
fn test_chop_panic() {
    let _x: u8 = 257u16.chop();
}

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
