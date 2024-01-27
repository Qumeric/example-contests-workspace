//{"name":"B. A Balanced Problemset?","group":"Codeforces - Codeforces Round 921 (Div. 2)","url":"https://codeforces.com/contest/1925/problem/B","interactive":false,"timeLimit":1500,"tests":[{"input":"3\n10 3\n5 5\n420 69\n","output":"2\n1\n6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BABalancedProblemset"}}}

use std::cmp::max;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let x = input.read_size();
    let n = input.read_size();

    // balance -- gcd of all
    // find max balance
    // sum 0_n = x

    if n == 1 {
        out.print_line(x);
        return;
    }
    // gcd (sum_div) = x. So gcd divides x

    let mut i = 1;
    let mut ans = 1;
    while i * i <= x {
        if x % i == 0 {
            let ii = x / i;
            if ii >= n {
                ans = max(ans, i);
            }
            if i >= n {
                ans = max(ans, ii);
            }
        }
        i += 1;
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
    // don't forget to set test_type = Single if you do it
    // tester::stress_test(run, tester::check);
}
//END MAIN
