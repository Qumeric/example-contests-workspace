//{"name":"D. Christmas Trees","group":"Codeforces - Codeforces Round 611 (Div. 3)","url":"https://codeforces.com/contest/1283/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"2 6\n1 5\n","output":"8\n-1 2 6 4 0 3\n"},{"input":"3 5\n0 3 1\n","output":"7\n5 -2 4 -1 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DChristmasTrees"}}}

use std::cmp::min;

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

// not quite correct may only be able to put worst person at K dist but doesn't mean that we need
// to fill greedy up to K
// maybe can be fixed by greedy fill up to k-1 and then fixing? Idk seems tricky
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let mut m = input.read_size();
    let mut x = input.read_long_vec(n);
    x.sort();

    let mut spaces = vec![];
    spaces.push((1_000_000i64, x[0], -1));
    spaces.push((1_000_000i64, x[n - 1], 1));

    for i in 1..n {
        // between prev and this
        let dist = x[i] - x[i - 1] - 1;

        let a = (dist + 1) / 2;
        let b = dist / 2;
        spaces.push((a, x[i - 1], 1));
        spaces.push((b, x[i], -1));
    }

    let mut ans = 0;
    let mut vals = vec![];

    spaces.sort();
    spaces.reverse();
    while spaces.last().unwrap().0 == 0 {
        spaces.pop();
    }

    // for s in &spaces {
    //     out.print_line(s);
    // }

    let mut cur = 1;
    while m > 0 {
        for i in 0..spaces.len() {
            if m == 0 {
                break;
            }
            ans += cur;
            spaces[i].0 -= 1;
            m -= 1;
            vals.push(spaces[i].1 + spaces[i].2 * cur);
        }
        while spaces.last().unwrap().0 == 0 {
            spaces.pop();
        }
        cur += 1;
    }
    out.print_line(ans);
    out.print_line(vals);
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
