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
