//{"name":"F. Consecutive Subsequence","group":"Codeforces - Codeforces Round 479 (Div. 3)","url":"https://codeforces.com/problemset/problem/977/F","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n3 3 4 7 5 6 8\n","output":"4\n2 3 5 6\n"},{"input":"6\n1 3 5 2 4 6\n","output":"2\n1 4\n"},{"input":"4\n10 9 8 7\n","output":"1\n1\n"},{"input":"9\n6 7 8 3 4 5 9 10 11\n","output":"6\n1 2 3 7 8 9\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FConsecutiveSubsequence"}}}

use std::cmp::max;
use std::collections::BTreeMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

// TODO: not finished. Need to iterate over consequitive keys and find pos with binsearch
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n: usize = input.read();
    let a: Vec<i64> = input.read_vec(n);

    let mut m: BTreeMap<i64, Vec<usize>> = Default::default();

    for i in 0..n {
        m.entry(a[i]).and_modify(|x| x.push(i)).or_insert(vec![i]);
    }

    m = m
        .into_iter()
        .map(|(k, mut v)| {
            v.sort();
            (k, v)
        })
        .collect();

    for k in m.keys().clone() {}
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
