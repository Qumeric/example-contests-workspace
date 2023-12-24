//{"name":"C. K-th Not Divisible by n","group":"Codeforces - Codeforces Round 640 (Div. 4)","url":"https://codeforces.com/problemset/problem/1352/C","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n3 7\n4 12\n2 1000000000\n7 97\n1000000000 1000000000\n2 1\n","output":"10\n15\n1999999999\n113\n1000000001\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CKThNotDivisibleByN"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n: i64 = input.read();
    let k: i64 = input.read();

    let mut l: i64 = 1;
    let mut r: i64 = 2_000_000_000 + 1;

    while l + 1 < r {
        let mid = (l + r) / 2;
        if mid - mid / n <= k {
            l = mid;
        } else {
            r = mid;
        }
    }
    if (l - 1) - (l - 1) / n == k {
        out.print_line(l-1);
    } else {
        out.print_line(l);
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
