//{"name":"B. Фото на память - 2 (round version)","group":"Codeforces - VK Cup 2015 - Round 1","url":"https://codeforces.com/contest/524/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n10 1\n20 2\n30 3\n","output":"180\n"},{"input":"3\n3 1\n2 2\n4 3\n","output":"21\n"},{"input":"1\n5 10\n","output":"50\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BFotoNaPamyat2RoundVersion"}}}

use std::cmp::{max, min};
use std::collections::BTreeSet;

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();

    let mut possible_h = BTreeSet::new();

    let mut f = vec![];

    for i in 0..n {
        let a = input.read_size();
        let b = input.read_size();
        f.push((min(a, b), max(a, b)));
        possible_h.insert(a);
        possible_h.insert(b);
    }

    let mut ans = 1_000_000_000;
    for h in possible_h {
        let mut w = 0;
        let mut bad = false;
        for i in 0..n {
            if f[i].0 > h {
                bad = true;
                break;
            }
            if f[i].1 <= h {
                w += f[i].0;
            } else {
                w += f[i].1;
            }
        }
        if !bad {
            ans.minim(h * w);
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
    // don't forget to set test_type = Single if you do it
    // tester::stress_test(run, tester::check);
}
//END MAIN
