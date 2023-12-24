//{"name":"A. Least Product","group":"Codeforces - Codeforces Round 917 (Div. 2)","url":"https://codeforces.com/contest/1917/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n1\n155\n4\n2 8 -1 3\n4\n-1 0 -2 -5\n4\n-15 -75 -25 -30\n","output":"1\n1 0\n0\n0\n1\n3 0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ALeastProduct"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n: usize = input.read();
    let a: Vec<i64> = input.read_vec(n);

    let negs: Vec<_> = a.clone().into_iter().filter(|e| *e < 0).collect();
    let zeros: Vec<_> = a.clone().into_iter().filter(|e| *e == 0).collect();

    if !zeros.is_empty() || negs.len() % 2 == 1 {
        out.print_line(0);
        return;
    }

    out.print_line(1);
    out.print_line("1 0");
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
