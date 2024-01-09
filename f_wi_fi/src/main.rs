//{"name":"F. Wi-Fi","group":"Codeforces - Codeforces Round 587 (Div. 3)","url":"https://codeforces.com/problemset/problem/1216/F","interactive":false,"timeLimit":2000,"tests":[{"input":"5 2\n00100\n","output":"3\n"},{"input":"6 1\n000000\n","output":"21\n"},{"input":"4 1\n0011\n","output":"4\n"},{"input":"12 6\n000010000100\n","output":"15\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FWiFi"}}}

use std::cmp::{max, min};

use algo_lib::collections::specs::ArqSpec;
use algo_lib::collections::static_arq::StaticArq;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

pub enum MinGet {}
impl ArqSpec for MinGet {
    type S = i64;
    type F = i64;
    fn op(&a: &Self::S, &b: &Self::S) -> Self::S {
        min(a, b)
    }
    fn identity() -> Self::S {
        i64::MAX
    }
    fn compose(&f: &Self::F, &g: &Self::F) -> Self::F {
        min(f, g)
    }
    fn apply(&f: &Self::F, &a: &Self::S, size: i64) -> Self::S {
        min(a, f)
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let k = input.read_size();

    let s = input.read_str();

    let mut x = vec![i64::MAX; n + 1];
    x[0] = 0;
    let mut tree = StaticArq::<MinGet>::new(&x);

    for i in 0..n {
        if s[i] == b'1' {
            // can set router
            let fst = max(0, i as i64 - k as i64) as usize;
            let end = min(n, i + k + 1);
            let cv = tree.query_point(fst);
            let nv = cv + i as i64 + 1;
            // out.print_line(("for", i, "upd inc", fst + 1, end, nv));
            tree.update(fst + 1, end, &nv);
        } else {
            let cv = tree.query_point(i);
            let nv = cv + i as i64 + 1;
            tree.update_point(i + 1, &nv);
        }
    }
    // for i in 0..=n {
    //     out.print_line(tree.query_point(i));
    // }
    let ans = tree.query_point(n);
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
    // don't forget to set test_type = Single if you do it
    // tester::stress_test(run, tester::check);
}
//END MAIN
