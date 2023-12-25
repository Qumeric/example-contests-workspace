//{"name":"A. Simple Design","group":"Codeforces - Codeforces Round 904 (Div. 2)","url":"https://codeforces.com/contest/1884/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n1 5\n10 8\n37 9\n777 3\n1235 10\n1 10\n","output":"5\n17\n45\n777\n1243\n19\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ASimpleDesign"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn is_beautiful(mut x: i64, k: i64) -> bool {
    let mut s = 0;
    while x > 0 {
        s += x % 10;
        x /= 10;
    }
    s % k == 0
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let x: i64 = input.read();
    let k: i64 = input.read();

    for i in x..(x + 100000) {
        if is_beautiful(i, k) {
            out.print_line(i);
            return;
        }
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
}
//END MAIN
