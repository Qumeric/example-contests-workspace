//{"name":"A. Divide it!","group":"Codeforces - Codeforces Round 565 (Div. 3)","url":"https://codeforces.com/contest/1176/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n1\n10\n25\n30\n14\n27\n1000000000000000000\n","output":"0\n4\n6\n6\n-1\n6\n72\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ADivideIt"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let mut n = input.read_long();
    let mut steps = 0;
    while n > 1 {
        if n % 2 == 0 {
            n = n / 2;
        } else if n % 3 == 0 {
            n = 2 * n / 3;
        } else if n % 5 == 0 {
            n = 4 * n / 5;
        } else {
            out.print_line(-1);
            return;
        }
        steps += 1;
    }
    out.print_line(steps);
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
