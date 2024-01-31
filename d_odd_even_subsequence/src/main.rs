//{"name":"D. Odd-Even Subsequence","group":"Codeforces - Codeforces Round 651 (Div. 2)","url":"https://codeforces.com/problemset/problem/1370/D","interactive":false,"timeLimit":2000,"tests":[{"input":"4 2\n1 2 3 4\n","output":"1\n"},{"input":"4 3\n1 2 3 4\n","output":"2\n"},{"input":"5 3\n5 3 4 2 6\n","output":"2\n"},{"input":"6 4\n5 3 50 2 4 5\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DOddEvenSubsequence"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let a = input.read_long_vec(n);
    let mut b = a.clone();
    b.remove(0);

    let mut l = 0;
    let mut r = 1_000_000_000 + 1;

    let check = |m, a: &[i64], k| {
        let mut k = k as i64;
        let n = a.len();
        let mut is_odd = true;
        for i in 0..n {
            if is_odd {
                if a[i] <= m {
                    is_odd = !is_odd;
                    k -= 1;
                }
            } else {
                is_odd = !is_odd;
                k -= 1;
            }
        }
        k <= 0
    };

    while r - l > 1 {
        let m = (l + r) / 2;
        if check(m, &a, k) || check(m, &b, k - 1) {
            r = m;
        } else {
            l = m;
        }
    }
    out.print_line(r);
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
