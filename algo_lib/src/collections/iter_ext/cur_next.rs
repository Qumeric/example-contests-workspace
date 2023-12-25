use crate::zip;
use std::iter::once;

pub fn cur_next(n: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..n).zip((1..n).chain(once(0)))
}

pub fn prev_cur_next(n: usize) -> impl Iterator<Item = (usize, usize, usize)> {
    zip!(once(n - 1).chain(0..n - 1), 0..n, (1..n).chain(once(0)))
}

/// This macro creates an iterator that iterates over multiple other
/// iterators simultaneously, yielding a tuple where each element
/// corresponds to the current item of one of the iterators.
///
/// # Examples
///
/// ```
/// use algo_lib::collections::iter_ext::zip;
///
/// let a = [1, 2, 3];
/// let b = [4, 5, 6];
///
/// let mut zipped = zip!(a.iter(), b.iter());
///
/// assert_eq!(zipped.next(), Some((&1, &4)));
/// assert_eq!(zipped.next(), Some((&2, &5)));
/// assert_eq!(zipped.next(), Some((&3, &6)));
/// assert!(zipped.next().is_none());
/// ```
///
/// # Syntax
///
/// `zip!` can take any number of comma-separated iterators and will
/// return an iterator that yields tuples of their items. The syntax is
/// as follows:
///
/// - `zip!(a)` where `a` is an iterator.
/// - `zip!(a, b)` where `a` and `b` are iterators.
/// - `zip!(a, b, ...)` where `a`, `b`, etc. are iterators.
#[macro_export]
macro_rules! zip {
    ( @closure $p:pat => $tup:expr ) => {
        |$p| $tup
    };

    ( @closure $p:pat => ( $($tup:tt)* ) , $_iter:expr $( , $tail:expr )* ) => {
        zip!(@closure ($p, b) => ( $($tup)*, b ) $( , $tail )*)
    };

    ($first:expr $(,)*) => {
        std::iter::IntoIterator::into_iter($first)
    };

    // binary
    ($first:expr, $second:expr $(,)*) => {
        zip!($first).zip($second)
    };

    // n-ary where n > 2
    ( $first:expr $( , $rest:expr )* $(,)* ) => {
        zip!($first)
            $(
                .zip($rest)
            )*
            .map(
                zip!(@closure a => (a) $( , $rest )*)
            )
    };
}
