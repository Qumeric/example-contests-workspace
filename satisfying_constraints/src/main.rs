//{"name":"Satisfying Constraints","group":"Codeforces","url":"https://m2.codeforces.com/contest/1920/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n4\n1 3\n2 10\n3 1\n3 5\n2\n1 5\n2 4\n10\n3 6\n3 7\n1 2\n1 7\n3 100\n3 44\n2 100\n2 98\n1 3\n3 99\n6\n1 5\n2 10\n1 9\n2 2\n3 2\n3 9\n5\n1 1\n2 2\n3 1\n3 2\n3 3\n6\n1 10000\n2 900000000\n3 500000000\n1 100000000\n3 10000\n3 900000001\n","output":"7\n0\n90\n0\n0\n800000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"SatisfyingConstraints"}}}

use std::char::MAX;
use std::cmp::{max, min};

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let mut mmin = 0;
    let mut mmax = usize::MAX;
    let mut neq = vec![];
    for i in 0..n {
        let t = input.read_size();
        let x = input.read_size();

        if t == 1 {
            mmin = max(mmin, x);
        }
        if t == 2 {
            mmax = min(mmax, x);
        }
        if t == 3 {
            neq.push(x);
        }
    }
    neq.sort();
    if mmax < mmin {
        out.print_line(0);
        return;
    }
    let mut ans = mmax - mmin + 1;

    for x in neq {
        if x >= mmin && x <= mmax {
            ans -= 1;
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
    // don't forget to set test_type = Single if you do it
    // tester::stress_test(run, tester::check);
}
//END MAIN
