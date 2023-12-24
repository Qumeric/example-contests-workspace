//{"name":"A. Hit the Lottery","group":"Codeforces - Codeforces Round 492 (Div. 2) [Thanks, uDebug!]","url":"https://codeforces.com/problemset/problem/996/A","interactive":false,"timeLimit":1000,"tests":[{"input":"125\n","output":"3\n"},{"input":"43\n","output":"5\n"},{"input":"1000000000\n","output":"10000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AHitTheLottery"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let mut a: i64 = input.read();

    let mut ans = 0;

    let bills = vec![1, 5, 10, 20, 100];

    for bill in bills.into_iter().rev() {
        while a >= bill {
            a -= bill;
            ans += 1;
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
