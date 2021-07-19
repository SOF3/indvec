#![no_std]

#[doc(hidden)]
pub extern crate alloc;

#[doc(hidden)]
#[macro_export]
macro_rules! seq_ident {
    ($($ident:ident),*) => {
        {
            let mut counter = 0usize;
            $(
                let $ident = counter;
                counter += 1usize;
            )*
            [$($ident),*]
        }
    }
}

/// A [`Vec`][alloc::vec::Vec] literal that assigns item indices.
///
/// ```
/// use indvec::indvec;
///
/// indvec![
///     a = "foo",
///     b = "bar",
///     _c = "qux",
///     d = "corge",
/// ];
///
/// assert_eq!(a, 0usize);
/// assert_eq!(b, 1usize);
/// assert_eq!(d, 3usize);
/// ```
#[macro_export]
macro_rules! indvec {
    ($($ident:ident = $expr:expr),* $(,)?) => {
        #[allow(unused_assignments)]
        let [$($ident),*] = $crate::seq_ident!($($ident),*);
        $crate::alloc::vec![$($expr),*]
    }
}
