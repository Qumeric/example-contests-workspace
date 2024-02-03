//{"name":"C. Robot Breakout","group":"Codeforces - Codeforces Round 575 (Div. 3)","url":"https://codeforces.com/contest/1196/problem/C","interactive":false,"timeLimit":3000,"tests":[{"input":"4\n2\n-1 -2 0 0 0 0\n-1 -2 0 0 0 0\n3\n1 5 1 1 1 1\n2 5 0 1 0 1\n3 5 1 0 0 0\n2\n1337 1337 0 1 1 1\n1336 1337 1 1 0 1\n1\n3 5 1 1 1 1\n","output":"1 -1 -2\n1 2 5\n0\n1 -100000 -100000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CRobotBreakout"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let q = input.read_size();

    let mut min_x = -100_000;
    let mut min_y = -100_000;
    let mut max_x = 100_000;
    let mut max_y = 100_000;
    for i in 0..q {
        let x = input.read_long();
        let y = input.read_long();
        let f1 = input.read_long() == 1;
        let f2 = input.read_long() == 1;
        let f3 = input.read_long() == 1;
        let f4 = input.read_long() == 1;

        if !f1 {
            min_x.maxim(x);
        }
        if !f2 {
            max_y.minim(y);
        }
        if !f3 {
            max_x.minim(x);
        }
        if !f4 {
            min_y.maxim(y);
        }
    }
    if min_x <= max_x && min_y <= max_y {
        out.print_line((1, min_x, min_y));
    } else {
        out.print_line(0);
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
    // don't forget to set test_type = Single if you do it
    // tester::stress_test(run, tester::check);
}
//END MAIN
