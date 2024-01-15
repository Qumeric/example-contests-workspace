//{"name":"E2. String Coloring (hard version)","group":"Codeforces - Codeforces Round 617 (Div. 3)","url":"https://codeforces.com/problemset/problem/1296/E2","interactive":false,"timeLimit":1000,"tests":[{"input":"9\nabacbecfd\n","output":"2\n1 1 2 1 2 1 2 1 2\n"},{"input":"8\naaabbcbb\n","output":"2\n1 2 1 2 1 2 1 1\n"},{"input":"7\nabcdedc\n","output":"3\n1 1 1 1 1 2 3\n"},{"input":"5\nabcde\n","output":"1\n1 1 1 1 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"E2StringColoringHardVersion"}}}

use std::collections::BTreeSet;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    // for each i we need to color it in the minimum color such that all larger letters before don't have it
    // so we need to save colors for each letter

    let n = input.read_size();
    let s = input.read_str();
    let mut colors = vec![0; n];
    let mut has = vec![BTreeSet::<i32>::default(); 26];

    for i in 0..n {
        let l = s[i] - b'a';

        for c in 1..30 {
            let mut had = false;
            for j in (l + 1)..26 {
                if has[j as usize].contains(&c) {
                    had = true;
                    break;
                }
            }
            if !had {
                colors[i] = c;
                break;
            }
        }
        has[l as usize].insert(colors[i]);
    }
    out.print_line(colors.iter().max().unwrap());
    out.print_line(colors);
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
