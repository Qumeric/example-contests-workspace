//! Number-theoretic utilities for contests problems.

use std::collections::HashMap;

pub fn generate_primes(limit: i64) -> Vec<bool> {
    if limit <= 0 || limit > 10i64.pow(8) {
        panic!("Incorrect limit {limit}")
    }

    let limit = limit as usize;
    let mut is_prime = vec![true; limit + 1];

    for i in 2..=limit {
        if !is_prime[i] {
            continue;
        }

        let mut j = i * 2;
        while j <= limit {
            is_prime[j] = false;
            j += i;
        }
    }

    is_prime
}

/// Fast iterative version of Euclid's GCD algorithm
pub fn fast_gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    a.abs()
}

fn pollard_rho(n: i64) -> i64 {
    for a in 1..n {
        let f = |x| pos_mod(mod_mul(x, x, n) + a, n);
        let mut x = 2;
        let mut y = 2;
        loop {
            x = f(x);
            y = f(f(y));
            let div = fast_gcd(x - y, n);
            if div == n {
                break;
            } else if div > 1 {
                return div;
            }
        }
    }
    panic!("No divisor found!");
}

// TODO: deduplicate modular arithmetic code with num::Field
fn pos_mod(n: i64, m: i64) -> i64 {
    if n < 0 {
        n + m
    } else {
        n
    }
}
fn mod_mul(a: i64, b: i64, m: i64) -> i64 {
    pos_mod((a as i128 * b as i128 % m as i128) as i64, m)
}
fn mod_exp(mut base: i64, mut exp: u64, m: i64) -> i64 {
    assert!(m >= 1);
    let mut ans = 1 % m;
    base %= m;
    while exp > 0 {
        if exp % 2 == 1 {
            ans = mod_mul(ans, base, m);
        }
        base = mod_mul(base, base, m);
        exp /= 2;
    }
    pos_mod(ans, m)
}

fn is_strong_probable_prime(n: i64, exp: u64, r: i64, a: i64) -> bool {
    let mut x = mod_exp(a, exp, n);
    if x == 1 || x == n - 1 {
        return true;
    }
    for _ in 1..r {
        x = mod_mul(x, x, n);
        if x == n - 1 {
            return true;
        }
    }
    false
}

/// Assuming x >= 0, returns whether x is prime
pub fn is_prime(n: i64) -> bool {
    const BASES: [i64; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    assert!(n >= 0);
    match n {
        0 | 1 => false,
        2 | 3 => true,
        _ if n % 2 == 0 => false,
        _ => {
            let r = (n - 1).trailing_zeros() as i64;
            let exp = (n - 1) as u64 >> r;
            BASES
                .iter()
                .all(|&base| base > n - 2 || is_strong_probable_prime(n, exp, r, base))
        }
    }
}

/// Assuming x >= 1, finds the prime factorization of n
/// TODO: pollard_rho needs randomization to ensure correctness in contest settings!
/// Example: factorize(12) => [2, 2, 3]
pub fn factorize(n: i64) -> Vec<i64> {
    assert!(n >= 1);
    let r = n.trailing_zeros() as usize;
    let mut factors = vec![2; r];
    let mut stack = match n >> r {
        1 => vec![],
        x => vec![x],
    };
    while let Some(top) = stack.pop() {
        if is_prime(top) {
            factors.push(top);
        } else {
            let div = pollard_rho(top);
            stack.push(div);
            stack.push(top / div);
        }
    }
    factors.sort_unstable();
    factors
}

/// Example: factorize_map(12) => {2: 2, 3: 1}
pub fn factorize_map(n: i64) -> HashMap<i64, usize> {
    let f = factorize(n);
    let mut factor_counts = HashMap::new();
    for &factor in &f {
        *factor_counts.entry(factor).or_insert(0) += 1;
    }
    factor_counts
}
