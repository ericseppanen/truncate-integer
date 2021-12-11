<!-- cargo-sync-readme start -->

truncate-integer: integer truncation for Rust

There are several ways one might want to do integer truncation in Rust:

- Unchecked: truncation may result in a changed value. You only get
the low-order N bits.
- Checked: if truncation would result in a changed value, return `None`,
otherwise `Some(value)`.
- Panicking: if truncation would result in a changed value, 'panic!'
This is equivalent to checked truncation with `.unwrap()`, but with a nicer
panic message.
- Saturating: if truncation would result in a changed value, return
the maximum value that would fit in the target type.

It's possible to get all of these in Rust without importing additional
crates or writing much code, for example:

```rust
let x = 257u16;

let unchecked = x as u8;
assert_eq!(unchecked, 1u8);

let checked = u8::try_from(x);
assert!(checked.is_err());

// This would panic
// let value = x.try_from().unwrap();

let saturating = u8::try_from(x).unwrap_or(u8::MAX);
assert_eq!(saturating, 255);
```

If those are good enough for you, then you don't need this crate.
However, if you would prefer to call a function to communicate your
intent, then you might find this crate useful.

It provides a trait that implements each of the truncation forms listed above:

- `TruncateUnchecked` performs unchecked truncation.
- `TryTruncate` performs checked truncation.
- `Chop` performs panicking truncation.
- `Shrink` performs saturating truncation.

It's sometimes desirable to invert this logic, e.g. in trait bounds,
so there is an inverse of each of the above:

- `TruncateFromUnchecked`
- `TryTruncateFrom`
- `ChopFrom`
- `ShrinkFrom`

All of the truncations are implemented for both signed and unsigned
integers (including signed-to-unsigned and vice versa), except
`TruncateFromUnchecked`, because it's not immediately clear what the
correct output would be when then input is outside the output bounds.

<!-- cargo-sync-readme end -->
