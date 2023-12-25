use std::iter::{Chain, Enumerate, Rev, Skip, Zip};

pub trait Iters<'a>: 'a
where
    &'a Self: IntoIterator,
{
    fn enumerate(&'a self) -> Enumerate<<&'a Self as IntoIterator>::IntoIter>;
    fn rev(&'a self) -> Rev<<&'a Self as IntoIterator>::IntoIter>
    where
        <&'a Self as IntoIterator>::IntoIter: DoubleEndedIterator;
    fn skip(&'a self, n: usize) -> Skip<<&'a Self as IntoIterator>::IntoIter>;
    fn chain<V: 'a>(
        &'a self,
        chained: &'a V,
    ) -> Chain<<&'a Self as IntoIterator>::IntoIter, <&'a V as IntoIterator>::IntoIter>
    where
        &'a V: IntoIterator<Item = <&'a Self as IntoIterator>::Item>;
    fn zip<V: 'a>(
        &'a self,
        other: &'a V,
    ) -> Zip<<&'a Self as IntoIterator>::IntoIter, <&'a V as IntoIterator>::IntoIter>
    where
        &'a V: IntoIterator<Item = <&'a Self as IntoIterator>::Item>;
}

/// Syntax sugar to avoid typing `into_iter()`.
impl<'a, U: 'a> Iters<'a> for U
where
    &'a U: IntoIterator,
{
    fn enumerate(&'a self) -> Enumerate<<&'a Self as IntoIterator>::IntoIter> {
        self.into_iter().enumerate()
    }

    fn rev(&'a self) -> Rev<<&'a Self as IntoIterator>::IntoIter>
    where
        <&'a Self as IntoIterator>::IntoIter: DoubleEndedIterator,
    {
        self.into_iter().rev()
    }

    fn skip(&'a self, n: usize) -> Skip<<&'a Self as IntoIterator>::IntoIter> {
        self.into_iter().skip(n)
    }

    fn chain<V: 'a>(
        &'a self,
        chained: &'a V,
    ) -> Chain<<&'a Self as IntoIterator>::IntoIter, <&'a V as IntoIterator>::IntoIter>
    where
        &'a V: IntoIterator<Item = <&'a Self as IntoIterator>::Item>,
    {
        self.into_iter().chain(chained)
    }

    fn zip<V: 'a>(
        &'a self,
        other: &'a V,
    ) -> Zip<<&'a Self as IntoIterator>::IntoIter, <&'a V as IntoIterator>::IntoIter>
    where
        &'a V: IntoIterator<Item = <&'a Self as IntoIterator>::Item>,
    {
        self.into_iter().zip(other)
    }
}

#[cfg(test)]
mod tests {
    use crate::collections::iter_ext::iters::Iters;

    #[test]
    fn iters_example() {
        let vec = vec![1, 2, 3];
        let mut iter = vec.enumerate();

        assert_eq!(iter.next(), Some((0, &1)));
        assert_eq!(iter.next(), Some((1, &2)));
        assert_eq!(iter.next(), Some((2, &3)));
        assert!(iter.next().is_none());
    }
}
