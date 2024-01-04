//{"name":"D. Print a 1337-string...","group":"Codeforces - Educational Codeforces Round 70 (Rated for Div. 2)","url":"https://codeforces.com/problemset/problem/1202/D","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n6\n1\n","output":"113337\n1337\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DPrintA1337String"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let mut n = input.read_size();

    if n == 1 {
        out.print_line(1337);
        return;
    }

    let mut thirds_middle = 0;

    for i in 2..100_000 {
        if i * (i + 1) / 2 > n {
            thirds_middle = i;
            n -= i * (i - 1) / 2;
            break;
        }
    }
    out.print(1);
    for i in 2..thirds_middle {
        out.print(3);
    }

    for _ in 0..n {
        out.print(1);
    }

    out.print(33);

    out.print_line(7);
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
