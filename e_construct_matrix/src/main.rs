//{"name":"E. Construct Matrix","group":"Codeforces - Codeforces Round 917 (Div. 2)","url":"https://codeforces.com/contest/1917/problem/E","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n4 0\n6 6\n6 5\n4 2\n6 36\n","output":"Yes\n0 0 0 0\n0 0 0 0\n0 0 0 0\n0 0 0 0\nYes\n1 0 0 0 0 0\n0 1 0 0 0 0\n0 0 1 0 0 0\n0 0 0 1 0 0\n0 0 0 0 1 0\n0 0 0 0 0 1\nNo\nNo\nYes\n1 1 1 1 1 1\n1 1 1 1 1 1\n1 1 1 1 1 1\n1 1 1 1 1 1\n1 1 1 1 1 1\n1 1 1 1 1 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EConstructMatrix"}}}

use std::collections::{BTreeMap, BTreeSet};

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn print_matrix(
    out: &mut Output,
    diag: usize,
    size1: usize,
    mul1: usize,
    size2: usize,
    mul2: usize,
    n: usize,
) {
    let mut matrix = vec![vec![0; n]; n];
    for i in 0..n {
        matrix[i][n - i - 1] = diag;
    }

    let mut count = 0;
    for len in 1..=size1 {
        for i in 0..len {
            if count >= size1 {
                break;
            }
            matrix[i][len - i - 1] = mul1;
            count += 1;
        }
    }
    count = 0;
    for len in 1..=size2 {
        for i in 0..len {
            if count >= size2 {
                break;
            }
            matrix[n - i - 1][n - (len - i)] = mul2;
            count += 1;
        }
    }

    // out.print_line((size1, mul1, size2, mul2));
    for line in matrix {
        out.print_line(line);
    }
}

// TODO: incorrect but maybe is close?
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n: usize = input.read(); // even; <= 1000
    let k: usize = input.read(); // <= n^2

    // only around 2.5e8 different inputs?

    if k % 2 == 1 || k < n {
        out.print_line("No");
        return;
    }

    let k = k / 2;
    let n = n / 2;
    let diags: Vec<_> = (0..=n).map(|e| (e * n, e)).collect();
    let mut diags_map = std::collections::BTreeMap::new();
    for (key, value) in diags {
        diags_map.insert(key, value);
    }

    // out.print_line("diags");
    // out.print_line("");

    for vert in 0..=n {
        for hor in 0..=n {
            let mut left: i64 = k as i64;
            left -= (vert * n * 2) as i64;
            left -= (hor * (n - vert) * 2) as i64;
            if left < 0 {
                continue;
            }
            if let Some(diag) = diags_map.get(&(left as usize)) {
                // out.print_line("ans");
                // out.print_line((vert, hor, diag));

                let mut m: Vec<Vec<usize>> = vec![vec![0; n]; n];

                for i in 0..vert {
                    for j in 0..n {
                        m[i][j] = 1;
                    }
                }
                for i in 0..hor {
                    for j in 0..n {
                        m[j][i] = 1;
                    }
                }

                // out.print_line("c");
                // for line in m.iter() {
                //     out.print_line(line);
                // }

                let mut big_m: Vec<Vec<usize>> = vec![vec![0; 2 * n]; 2 * n];
                for i in 0..n {
                    for j in 0..n {
                        big_m[i][j] = m[i][j];
                        big_m[i][2 * n - j - 1] = m[i][j];
                        big_m[2 * n - i - 1][j] = m[i][j];
                        big_m[2 * n - i - 1][2 * n - j - 1] = m[i][j];
                    }
                }

                for i in 0..(2 * n) {
                    big_m[i][i] += diag;
                }

                out.print_line("Yes");
                for line in big_m {
                    out.print_line(line);
                }

                return;
            }
        }
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, &mut output, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN

/*

1 1 1 1 1 1
0 2 0 2 0 0
0 2 0 2 0 0
0 2 0 2 0 0
0 2 0 2 0 0
1 1 1 1 1 1

*/
