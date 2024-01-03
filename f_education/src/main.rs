//{"name":"F. Education","group":"Codeforces - Codeforces Round 713 (Div. 3)","url":"https://codeforces.com/problemset/problem/1512/F","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n4 15\n1 3 10 11\n1 2 7\n4 100\n1 5 10 50\n3 14 12\n2 1000000000\n1 1\n1\n","output":"6\n13\n1000000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FEducation"}}}

use std::cmp::{max, min};

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn div_ceil(a: i64, b: i64) -> i64 {
    (a + b - 1) / b
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let c = input.read_long();
    let a = input.read_long_vec(n);
    let b = input.read_long_vec(n - 1);

    let mut ans = i64::MAX;

    let mut days_spent = 0;
    let mut rem = 0;

    for i in 0..n {
        ans = min(ans, days_spent + div_ceil(c - rem, a[i]));

        if i < n - 1 {
            let new_days = div_ceil(max(b[i] - rem, 0), a[i]);
            rem += new_days * a[i] - b[i];
            days_spent += new_days;
        }
        days_spent += 1;
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
