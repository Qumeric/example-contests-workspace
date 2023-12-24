//{"name":"A. Nastia and Nearly Good Numbers","group":"Codeforces - Codeforces Round 720 (Div. 2)","url":"https://codeforces.com/problemset/problem/1521/A","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n5 3\n13 2\n7 11\n","output":"YES\n10 50 60\nYES\n169 39 208\nYES\n28 154 182\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ANastiaAndNearlyGoodNumbers"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let a: usize = input.read();
    let b: usize = input.read();

    if b == 1 {
        out.print_line("NO");
    } else if b == 2 {
        out.print_line("YES");
        let x = a;
        let y = a * 5;
        let z = a * 6;
        out.print_line((x, y, z));
    } else {
        out.print_line("YES");
        let x = a;
        let y = a * (b - 1);
        let z = a * b;
        out.print_line((x, y, z));
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
