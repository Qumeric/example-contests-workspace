//{"name":"E. Obtain a Permutation","group":"Codeforces - Codeforces Round 615 (Div. 3)","url":"https://codeforces.com/problemset/problem/1294/E","interactive":false,"timeLimit":2000,"tests":[{"input":"3 3\n3 2 1\n1 2 3\n4 5 6\n","output":"6\n"},{"input":"4 3\n1 2 3\n4 5 6\n7 8 9\n10 11 12\n","output":"0\n"},{"input":"3 4\n1 6 3 4\n5 10 7 8\n9 2 11 12\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EObtainAPermutation"}}}

use std::collections::BTreeMap;

use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();

    let mut arr = Arr2d::new(n, m, 0);
    for i in 0..n {
        for j in 0..m {
            arr[i][j] = input.read_size();
        }
    }

    let mut ans = 0;
    for j in 0..m {
        let mut s = vec![0; n];
        for i in 0..n {
            if arr[i][j] <= j {
                continue;
            }
            let val = arr[i][j] - j - 1;
            if val % m == 0 && val / m < n {
                let want_pos = val / m;
                let shifts = if want_pos <= i {
                    i - want_pos
                } else {
                    n - (want_pos - i)
                };
                s[shifts] += 1;
            }
        }
        let col_ans = s.enumerate().map(|(i, &val)| n - val + i).min().unwrap();
        // out.print_line(("col", j, col_ans));
        ans += col_ans;
    }
    out.print_line(ans);
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
