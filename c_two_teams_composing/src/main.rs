//{"name":"C. Two Teams Composing","group":"Codeforces - Codeforces Round 634 (Div. 3)","url":"https://codeforces.com/contest/1335/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n7\n4 2 4 1 4 3 4\n5\n2 1 5 4 3\n1\n1\n4\n1 1 1 3\n","output":"3\n1\n0\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CTwoTeamsComposing"}}}

use std::cmp::min;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);
    let mut occurrences = std::collections::HashMap::new();
    for &item in &a {
        *occurrences.entry(item).or_insert(0) += 1;
    }
    let max_occurrences = *occurrences.values().max().unwrap_or(&0);

    let mut distinct = a.clone();
    distinct.sort();
    distinct.dedup();

    if max_occurrences as usize > distinct.len() {
        out.print_line(distinct.len())
    } else {
        out.print_line(min(distinct.len() - 1, max_occurrences));
    }

    // first team unique, second same
    // size equal
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
