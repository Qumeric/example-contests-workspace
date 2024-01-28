//{"name":"C. Doors Breaking and Repairing","group":"Codeforces - Codeforces Round 531 (Div. 3)","url":"https://codeforces.com/contest/1102/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"6 3 2\n2 3 1 3 4 2\n","output":"6\n"},{"input":"5 3 3\n1 2 4 2 3\n","output":"2\n"},{"input":"5 5 6\n1 2 6 10 3\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CDoorsBreakingAndRepairing"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let x = input.read_size();
    let y = input.read_size();

    let a = input.read_size_vec(n);

    if x > y {
        out.print_line(n);
        return;
    }
    let small = a.iter().filter(|&d| d <= &x).count();

    out.print_line((small + 1) / 2);
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
