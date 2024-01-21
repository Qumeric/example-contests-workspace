#[cfg(test)]
mod tests {
    use crate::collections::iter_ext::interleave::interleave;
    use crate::collections::iter_ext::iters::Iters;
    use crate::collections::{dsu::DSU, specs::AssignMin, static_arq::StaticArq};

    #[test]
    fn dsu() {
        let mut dsu = DSU::new(10);
        assert_eq!(dsu.set_count(), 10);

        assert!(dsu.join(2, 3));
        assert_eq!(dsu.set_count(), 9);
        assert_eq!(dsu.get(2), dsu.get(3));

        assert!(!dsu.join(2, 3));
        assert_eq!(dsu.set_count(), 9);

        assert!(dsu.join(3, 4));
        assert_eq!(dsu.set_count(), 8);
        assert_eq!(dsu.get(2), dsu.get(4));

        dsu.clear();
        assert_eq!(dsu.set_count(), 10);
    }

    #[test]
    fn rmq() {
        let vec: Vec<i64> = vec![1, 3, 2, 5];
        let mut rmq_tree = StaticArq::<AssignMin>::new(&vec);

        assert_eq!(rmq_tree.query(0, 1), 1);
        assert_eq!(rmq_tree.query(0, 2), 1);
        assert_eq!(rmq_tree.query(1, 2), 2);
        assert_eq!(rmq_tree.query(2, 3), 2);
        rmq_tree.update(0, 3, &1);
        assert_eq!(rmq_tree.query(0, 3), 1);
    }

    #[test]
    fn interleaved() {
        let a = vec![1, 3];
        let b = vec![2, 4];

        let mut interleaved = interleave(a.iter(), b.iter());

        assert_eq!(interleaved.next(), Some(&1));
        assert_eq!(interleaved.next(), Some(&2));
        assert_eq!(interleaved.next(), Some(&3));
        assert_eq!(interleaved.next(), Some(&4));
        assert!(interleaved.next().is_none());
    }

    #[test]
    fn iters() {
        let vec = vec![1, 2, 3];
        let mut iter = vec.enumerate();

        assert_eq!(iter.next(), Some((0, &1)));
        assert_eq!(iter.next(), Some((1, &2)));
        assert_eq!(iter.next(), Some((2, &3)));
        assert!(iter.next().is_none());
    }

    #[test]
    fn iters_map() {
        let vec = vec![1, 2, 3];
        let mut iter = vec.map(|x| x * 2);

        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), Some(6));
        assert!(iter.next().is_none());
    }
}
