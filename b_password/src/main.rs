//{"name":"B. Password","group":"Codeforces - Codeforces Beta Round 93 (Div. 1 Only)","url":"https://codeforces.com/problemset/problem/126/B","interactive":false,"timeLimit":2000,"tests":[{"input":"fixprefixsuffix\n","output":"fix\n"},{"input":"abcdabc\n","output":"Just a legend\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BPassword"}}}

use std::cmp::max;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::strings::{z_function, KmpMatcher};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let s = input.read_line();
    let n = s.len();

    let c: Vec<char> = s.chars().collect();

    let zf = z_function(&c);

    let mut mids: Vec<usize> = vec![0; s.len()];
    let mut ends: Vec<usize> = vec![0; s.len()];

    if n < 3 {
        out.print_line("Just a legend");
        return;
    }

    // out.print_line(&zf);

    let mut max_mid = 0;

    for i in 1..n {
        if zf[i] > 0 {
            if i + zf[i] < n {
                max_mid = max(max_mid, zf[i]);
            } else {
                ends[zf[i]] = i;
                max_mid = max(max_mid, zf[i] - 1);
            }
        }
    }

    // out.print_line(&mids);
    // out.print_line(&ends);

    for i in (1..n).rev() {
        if ends[i] > 0 {
            if max_mid >= i {
                out.print_line(c.into_iter().take(i).collect::<String>());
                return;
            }
        }
    }
    out.print_line("Just a legend")
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
