//! Number-theoretic utilities for contests problems.
pub fn generate_primes(limit: i64) -> Vec<bool> {
    if limit <= 0 || limit > 10i64.pow(8) {
        panic!("Incorrect limit {limit}")
    }

    let limit = limit as usize;
    let mut is_prime = vec![true; limit+1];

    for i in 2..=limit {
        if !is_prime[i] {
            continue;
        }

        let mut j = i*2;
        while j <= limit {
            is_prime[j] = false;
            j += i;
        }
    }

    is_prime
}
