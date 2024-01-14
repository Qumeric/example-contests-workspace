//{"name":"D. Yet Another Subarray Problem","group":"Codeforces - Educational Codeforces Round 69 (Rated for Div. 2)","url":"https://codeforces.com/problemset/problem/1197/D","interactive":false,"timeLimit":2000,"tests":[{"input":"7 3 10\n2 -4 15 -3 4 8 3\n","output":"7\n"},{"input":"5 2 1000\n-13 -4 -9 -20 -11\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DYetAnotherSubarrayProblem"}}}

use std::cmp::max;

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_long();
    let mut a = input.read_long_vec(n);

    if m == 1 {
        // special case
        for i in 0..n {
            a[i] -= k;
        }
        let mut max_sum = 0i64;
        let mut current_sum = 0i64;
        for &value in &a {
            current_sum = max(value, current_sum + value);
            max_sum = max(max_sum, current_sum);
        }
        out.print_line(max_sum);
        return;
    }

    let mut dp = Arr2d::new(n, m + 1, 0i64);
    dp[0][1] = a[0] - k;
    for i in 2..m {
        dp[0][i] = -2_000_000_000i64;
    }

    for i in 1..n {
        for rem in 0..m {
            let next_rem = (rem + 1) % m;
            if next_rem == 0 {
                dp[i][next_rem] = max(0, dp[i - 1][rem] + a[i]);
            } else if next_rem == 1 {
                dp[i][next_rem] = max(a[i] - k, dp[i - 1][rem] + a[i] - k);
            } else {
                dp[i][next_rem] = dp[i - 1][rem] + a[i];
            }
        }
    }
    let mut ans = 0;
    for i in 0..n {
        for rem in 0..m {
            ans = max(ans, dp[i][rem]);
        }
    }

    // for rem in 0..m {
    //     for i in 0..n {
    //         out.print(dp[i][rem]);
    //         out.print(" ");
    //     }
    //     out.print_line("");
    // }

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
