/// Single-pattern matching with the Knuth-Morris-Pratt algorithm
pub struct KmpMatcher<'a, C: Eq> {
    /// The string pattern to search for.
    pub pattern: &'a [C],
    /// KMP match failure automaton: fail[i] is the length of the longest
    /// string that's both a proper prefix and a proper suffix of pattern[0..=i].
    pub prefix_function: Vec<usize>,
}

impl<'a, C: Eq> KmpMatcher<'a, C> {
    /// Precomputes the automaton that allows linear-time string matching.
    ///
    /// # Example
    ///
    /// ```
    /// use contest_algorithms::string_proc::Matcher;
    /// let byte_string: &[u8] = b"hello";
    /// let utf8_string: &str = "hello";
    /// let vec_char: Vec<char> = utf8_string.chars().collect();
    ///
    /// let match_from_byte_literal = Matcher::new(byte_string);
    /// let match_from_utf8 = Matcher::new(utf8_string.as_bytes());
    /// let match_from_chars = Matcher::new(&vec_char);
    ///
    /// let vec_int = vec![4, -3, 1];
    /// let match_from_ints = Matcher::new(&vec_int);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if pattern is empty.
    pub fn new(pattern: &'a [C]) -> Self {
        let mut fail = Vec::with_capacity(pattern.len());
        fail.push(0);
        let mut len = 0;
        for ch in &pattern[1..] {
            while len > 0 && pattern[len] != *ch {
                len = fail[len - 1];
            }
            if pattern[len] == *ch {
                len += 1;
            }
            fail.push(len);
        }
        Self {
            pattern,
            prefix_function: fail,
        }
    }

    /// KMP algorithm, sets @return[i] = length of longest prefix of pattern
    /// matching a suffix of text[0..=i].
    pub fn kmp_match(&self, text: impl IntoIterator<Item = C>) -> Vec<usize> {
        let mut len = 0;
        text.into_iter()
            .map(|ch| {
                if len == self.pattern.len() {
                    len = self.prefix_function[len - 1];
                }
                while len > 0 && self.pattern[len] != ch {
                    len = self.prefix_function[len - 1];
                }
                if self.pattern[len] == ch {
                    len += 1;
                }
                len
            })
            .collect()
    }
}

/// Calculates the Z-function for a pattern.
/// For a string s, Z-function produces an array where the i-th element is
/// the length of the longest substring starting from s[i] which is also a prefix of s.
///
/// # Examples
/// "aaaaa"   - [0, 4, 3, 2, 1]
/// "aaabaab" - [0, 2, 1, 0, 2, 1, 0]
/// "abacaba" - [0, 0, 1, 0, 3, 0 1]
pub fn z_function<'a, C: Eq>(pattern: &'a [C]) -> Vec<usize> {
    let n = pattern.len();
    let mut z = vec![0; n];
    let (mut l, mut r) = (0, 0);
    for i in 1..n {
        if i < r {
            z[i] = usize::min(r - i, z[i - l]);
        }
        while i + z[i] < n && pattern[z[i]] == pattern[i + z[i]] {
            z[i] += 1;
        }
        if i + z[i] > r {
            l = i;
            r = i + z[i];
        }
    }
    z
}

#[test]
fn z_function_matches_examples() {
    let test_cases = vec![
        ("aaaaa", vec![0, 4, 3, 2, 1]),
        ("aaabaab", vec![0, 2, 1, 0, 2, 1, 0]),
        ("abacaba", vec![0, 0, 1, 0, 3, 0, 1]),
    ];

    for (s, t) in test_cases.into_iter() {
        assert_eq!(z_function(s.as_bytes()), t);
    }
}
