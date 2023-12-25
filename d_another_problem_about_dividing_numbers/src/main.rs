//{"name":"D. Another Problem About Dividing Numbers","group":"Codeforces - Codeforces Round 725 (Div. 3)","url":"https://codeforces.com/problemset/problem/1538/D","interactive":false,"timeLimit":2000,"tests":[{"input":"8\n36 48 2\n36 48 3\n36 48 4\n2 8 1\n2 8 2\n1000000000 1000000000 1000000000\n1 2 1\n2 2 1\n","output":"YES\nYES\nYES\nYES\nYES\nNO\nYES\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DAnotherProblemAboutDividingNumbers"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::factorize;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let a: i64 = input.read();
    let b: i64 = input.read();
    let k: i64 = input.read();

    if k == 1 {
        out.print_line((a % b == 0 || b % a == 0) && a != b);
        return;
    }

    let fa = factorize(a);
    let fb = factorize(b);

    let divs = fa.len() + fb.len();
    out.print_line(divs >= k as usize);
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
