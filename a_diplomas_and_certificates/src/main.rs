//{"name":"A. Diplomas and Certificates","group":"Codeforces - Educational Codeforces Round 24","url":"https://codeforces.com/contest/818/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"18 2\n","output":"3 6 9\n"},{"input":"9 10\n","output":"0 0 9\n"},{"input":"1000000000000 5\n","output":"83333333333 416666666665 500000000002\n"},{"input":"1000000000000 499999999999\n","output":"1 499999999999 500000000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ADiplomasAndCertificates"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let k = input.read_size();

    let div = (k + 1);
    let pw = n / 2;
    let w = pw / div;
    out.print_line((w, w * k, n - w * div));
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
