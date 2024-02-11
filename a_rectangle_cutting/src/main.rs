//{"name":"A. Rectangle Cutting","group":"Codeforces - Codeforces Round 924 (Div. 2)","url":"https://codeforces.com/contest/1928/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n1 1\n2 1\n2 6\n3 2\n2 2\n2 4\n6 3\n","output":"No\nNo\nYes\nYes\nYes\nYes\nNo\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ARectangleCutting"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let mut a = input.read_size();
    let mut b = input.read_size();
    if a % 2 == 1 && b % 2 == 1 {
        out.print_line("No");
        return;
    }

    if a < b {
        std::mem::swap(&mut a, &mut b);
    }
    if a % 2 == 0 {
        if a / 2 != b {
            out.print_line("Yes");
            return;
        }
    }
    if b % 2 == 0 {
        out.print_line("Yes");
    } else {
        out.print_line("No");
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
