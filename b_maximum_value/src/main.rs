//{"name":"B. Maximum Value","group":"Codeforces - Codeforces Round 276 (Div. 1)","url":"https://codeforces.com/problemset/problem/484/B","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n3 4 5\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BMaximumValue"}}}

use std::cmp::max;

use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let mut a = input.read_u64_vec(n);

    a.sort_unstable();
    a.dedup();

    let n = a.len();

    let mut ans = 0;
    if n < 30_000 {
        for i in 0..n {
            for j in i + 1..n {
                ans = max(ans, a[j] % a[i]);
            }
        }
        out.print_line(ans);
        return;
    }
    let mut i = n - 1;

    while i > 0 {
        for j in 2..10 {
            let need_larger = a[i] / j;
            let pos = a.upper_bound(&need_larger);
            if pos < n {
                ans = max(ans, a[i] % a[pos]);
            }
        }

        i -= 1;
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
