//{"name":"D. Pair of Topics","group":"Codeforces - Codeforces Round 627 (Div. 3)","url":"https://codeforces.com/problemset/problem/1324/D","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n4 8 2 6 2\n4 5 4 1 3\n","output":"7\n"},{"input":"4\n1 3 2 4\n1 3 2 4\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DPairOfTopics"}}}

use std::cmp::max;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n: usize = input.read();
    let a: Vec<i64> = input.read_vec(n);
    let b: Vec<i64> = input.read_vec(n);

    let mut c: Vec<_> = a
        .into_iter()
        .zip(b.into_iter())
        .map(|(a, b)| a - b)
        .collect();

    c.sort();

    let mut count = 0;

    for (i, e) in c.iter().enumerate() {
        let mut left = i;
        let mut right = c.len();
        while left < right {
            let mid = left + (right - left) / 2;
            if c[mid] + e > 0 {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        let pos = max(left, i + 1);

        count += c.len() - pos;
    }

    out.print_line(count);
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
