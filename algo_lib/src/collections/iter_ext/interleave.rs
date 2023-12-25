enum NextFrom {
    First,
    Second,
}
/// An iterator that alternates between elements from two other iterators.
pub struct Interleave<T, I: Iterator<Item = T>, J: Iterator<Item = T>> {
    iter1: I,
    iter2: J,
    next_from: NextFrom,
}

impl<T, I: Iterator<Item = T>, J: Iterator<Item = T>> Iterator for Interleave<T, I, J> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_from {
            NextFrom::First => {
                self.next_from = NextFrom::Second;
                self.iter1.next()
            }
            NextFrom::Second => {
                self.next_from = NextFrom::First;
                self.iter2.next()
            }
        }
    }
}

/// Creates an iterator that alternates elements from two other iterators.
pub fn interleave<T, I: Iterator<Item = T>, J: Iterator<Item = T>>(
    iter1: I,
    iter2: J,
) -> Interleave<T, I, J> {
    Interleave {
        iter1,
        iter2,
        next_from: NextFrom::First,
    }
}

#[cfg(test)]
mod tests {
    use crate::collections::iter_ext::interleave::interleave;

    #[test]
    fn interleaved_example() {
        let a = vec![1, 3];
        let b = vec![2, 4];

        let mut interleaved = interleave(a.iter(), b.iter());

        assert_eq!(interleaved.next(), Some(&1));
        assert_eq!(interleaved.next(), Some(&2));
        assert_eq!(interleaved.next(), Some(&3));
        assert_eq!(interleaved.next(), Some(&4));
        assert!(interleaved.next().is_none());
    }
}
