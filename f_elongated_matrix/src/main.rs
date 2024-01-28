//{"name":"F. Elongated Matrix","group":"Codeforces - Codeforces Round 531 (Div. 3)","url":"https://codeforces.com/contest/1102/problem/F","interactive":false,"timeLimit":4000,"tests":[{"input":"4 2\n9 9\n10 8\n5 3\n4 3\n","output":"5\n"},{"input":"2 4\n1 2 3 4\n10 3 7 3\n","output":"0\n"},{"input":"6 1\n3\n6\n2\n5\n1\n4\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FElongatedMatrix"}}}

use std::cmp::min;

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::md_arr::arr3d::Arr3d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size(); // <= 16
    let m = input.read_size(); // <= 10_000

    let mut matrix = Arr2d::new(n, m, 0);

    for i in 0..n {
        for j in 0..m {
            matrix[i][j] = input.read_long();
        }
    }

    if n == 1 {
        let mut ans = i64::MAX;
        for j in 1..m {
            ans = min(ans, (matrix[0][j] - matrix[0][j - 1]).abs());
        }
        out.print_line(ans);
        return;
    }
    // if m == 1 {
    //     let mut ans = i64::MAX;
    //     let mut vec = vec![];
    //     for i in 0..n {
    //         vec.push(matrix[i][0]);
    //     }
    //     vec.sort();
    //     for i in 1..n {
    //         ans = min(ans, vec[]
    //     }
    //     out.print_line(ans);
    //     return;
    // }

    let mut ks = Arr2d::new(n, n, 0);

    // mask, first, last
    let mut dp = Arr3d::new(1 << n, n, n, 0);

    for a in 0..n {
        for b in 0..n {
            if a == b {
                continue;
            }
            let mut d = u64::MAX;
            for j in 0..m {
                d = min(d, (matrix[a][j] - matrix[b][j]).abs() as u64);
            }
            ks[a][b] = d;
            let mask = (1 << a) | (1 << b);
            dp[(mask as usize, a, b)] = d;
        }
    }

    let mut next_ks = Arr2d::new(n, n, u64::MAX);

    // HERE "a" IS THE LAST ROW, AND "b" IS THE FIRST
    for a in 0..n {
        for b in 0..n {
            if a == b {
                continue;
            }
            let mut d = u64::MAX;
            for j in 0..m - 1 {
                d = min(d, (matrix[a][j] - matrix[b][j + 1]).abs() as u64);
            }
            next_ks[a][b] = d;
        }
    }

    for mask in 0..(1 << n) {
        for first in 0..n {
            for old in 0..n {
                let from_val = dp[(mask, first, old)];
                if mask & (1 << first) == 0 {
                    continue;
                }
                if mask & (1 << old) == 0 {
                    continue;
                }
                // if (mask) == ((1 << 0) | (1 << 2)) {
                //     out.print_line("CHECKING FROM_VAL");
                //     out.print_line(from_val);
                // }
                if from_val == 0 {
                    continue;
                }
                for new in 0..n {
                    if mask & (1 << new) > 0 {
                        // alrdeay had this
                        continue;
                    }
                    if new == old || new == first {
                        continue;
                    }
                    // if new == 1 && first == 0 {
                    //     out.print_line("FIRST OLD NEW");
                    //     out.print_line((first, old, new));
                    //     out.print_line("KS FROM_VAL");
                    //     out.print_line((ks[old][new], from_val));
                    //     out.print_line("OLD KS");
                    //     out.print_line(ks[first][old]);
                    //     out.print_line("MASK");
                    //     out.print_line(mask);
                    // }
                    dp[(mask | (1 << new), first, new)].maxim(min(ks[old][new], from_val));
                }
            }
        }
    }
    let final_mask = (1 << n) - 1;
    let mut ans = 0;
    for a in 0..n {
        for b in 0..n {
            if a == b {
                continue;
            }
            // out.print_line(("FOR START", a, "END", b));
            // out.print_line((
            //     "DP",
            //     dp[(final_mask, a, b)],
            //     "KS",
            //     ks[a][b],
            //     "NEXT_KS (rot)",
            //     next_ks[b][a],
            // ));
            ans.maxim(min(dp[(final_mask, a, b)], next_ks[b][a]));
        }
    }

    out.print_line(ans);

    // 2 ^ 16 * 16 (last) * 16 (first) * 16 (just in case) IS OK
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
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
    // don't forget to set test_type = Single if you do it
    // tester::stress_test(run, tester::check);
}
//END MAIN
