//{"name":"F2. Pictures with Kittens (hard version)","group":"Codeforces - Codeforces Round 521 (Div. 3)","url":"https://codeforces.com/contest/1077/problem/F2","interactive":false,"timeLimit":2500,"tests":[{"input":"5 2 3\n5 1 3 10 1\n","output":"18\n"},{"input":"6 1 5\n10 30 30 70 10 10\n","output":"-1\n"},{"input":"4 3 1\n1 100 1 1\n","output":"100\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"F2PicturesWithKittensHardVersion"}}}

use std::cmp::{max, min};

use algo_lib::collections::specs::AssignMax;
use algo_lib::collections::static_arq::StaticArq;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

// TODO: correct but to slow, should be n^2
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let x = input.read_size();

    let a = input.read_long_vec(n);

    let mut segs = vec![];
    for i in 0..=n {
        segs.push(StaticArq::<AssignMax>::new(&vec![i64::MIN; n + 1]));
    }

    for i in 0..k {
        segs[1].update_point(n - 1 - i, &a[n - 1 - i]);
    }

    for i in (0..n).rev() {
        for kittens in 1..x {
            let r = i + k;
            let end = segs[kittens].query(i + 1, min(r, n));
            let val = a[i] + end;
            segs[kittens + 1].update_point(i, &val);
            // out.print_line(("at", i, "kittens", kittens + 1, "have", val));
        }
    }

    let mut ans = -1;
    for i in 0..k {
        ans = max(ans, segs[x].query_point(i));
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
    // don't forget to set test_type = Single if you do it
    // tester::stress_test(run, tester::check);
}
//END MAIN
