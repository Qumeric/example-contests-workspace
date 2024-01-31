use crate::numbers::num_traits::algebra::{
    IntegerMultiplicationMonoid, IntegerSemiRingWithSub, One, SemiRingWithSub, Zero,
};
use crate::numbers::num_traits::wideable::Wideable;
use std::collections::BTreeMap;
use std::mem::swap;

pub fn extended_gcd<T: IntegerSemiRingWithSub + Wideable + Copy>(a: T, b: T) -> (T, T::W, T::W)
where
    T::W: Copy + SemiRingWithSub,
{
    if a == T::zero() {
        (b, T::W::zero(), T::W::one())
    } else {
        let (d, y, mut x) = extended_gcd(b % a, a);
        x -= T::W::from(b / a) * y;
        (d, x, y)
    }
}

pub fn gcd<T: Copy + Zero + IntegerMultiplicationMonoid>(mut a: T, mut b: T) -> T {
    while b != T::zero() {
        a %= b;
        swap(&mut a, &mut b);
    }
    a
}

pub fn lcm<T: Copy + Zero + IntegerMultiplicationMonoid>(a: T, b: T) -> T {
    (a / gcd(a, b)) * b
}

/// Computes the subarray gcds for each position in the array.
///
/// For each index `i` in the array `a`, this function calculates a mapping
/// from each possible gcd value to the largest length `len` such that
/// `gcd(a[i - len], ..., a[i])` equals to that gcd value.
///
/// # Arguments
///
/// * `a` - A vector of elements for which the subarray gcds are to be computed.
///
/// # Returns
///
/// A vector of `BTreeMap`s where each map corresponds to an index in `a` and
/// contains the gcd values and their associated maximum lengths.
pub fn subsegment_gcd<T>(a: Vec<T>) -> Vec<BTreeMap<T, usize>>
where
    T: Copy + SemiRingWithSub + IntegerMultiplicationMonoid + Ord,
{
    let n = a.len();
    let mut sub_gcd: Vec<BTreeMap<T, usize>> = vec![BTreeMap::new(); n];
    sub_gcd[0].insert(a[0], 0);
    for i in 1..n {
        sub_gcd[i].insert(a[i], 0);
        // TODO: avoid clone here
        let prev_gcd = sub_gcd[i - 1].clone();
        for (&key, &value) in prev_gcd.iter() {
            let new_gcd = gcd(key, a[i]);
            sub_gcd[i]
                .entry(new_gcd)
                .and_modify(|e| *e = usize::max(*e, value + 1))
                .or_insert(value + 1);
        }
    }
    sub_gcd
}
