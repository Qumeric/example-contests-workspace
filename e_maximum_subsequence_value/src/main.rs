//{"name":"E. Maximum Subsequence Value","group":"Codeforces - Codeforces Round 648 (Div. 2)","url":"https://codeforces.com/contest/1365/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n2 1 3\n","output":"3\n"},{"input":"3\n3 1 4\n","output":"7\n"},{"input":"1\n1\n","output":"1\n"},{"input":"4\n7 7 1 1\n","output":"7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EMaximumSubsequenceValue"}}}

use std::cmp::max;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n: usize = input.read();

    let a: Vec<usize> = input.read_vec(n);

    let mut ans = 0;
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                ans = max(ans, a[i] | a[j] | a[k]);
            }
        }
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
}
//END MAIN