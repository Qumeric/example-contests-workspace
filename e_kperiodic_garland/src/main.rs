//{"name":"E. K-periodic Garland","group":"Codeforces - Codeforces Round 642 (Div. 3)","url":"https://codeforces.com/problemset/problem/1353/E","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n9 2\n010001010\n9 3\n111100000\n7 4\n1111111\n10 3\n1001110101\n1 1\n1\n1 1\n0\n","output":"1\n2\n5\n4\n0\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EKPeriodicGarland"}}}

use std::cmp::min;

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

// TODO I think if we implement this solution will be full and correct
fn min_operations_to_contiguous(numbers: Vec<usize>) -> usize {
    if numbers.is_empty() {
        return 0;
    }
    let mut max_balance = 1;
    let mut balance = 0;
    for i in 0..(numbers.len()) {
        balance += 1;
        balance -= 
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let s = input
        .read_line()
        .into_iter()
        .map(|c| if c == b'0' { 0 } else { 1 })
        .collect_vec();

    let total_ones: usize = s.clone().into_iter().sum();

    let mut v = vec![vec![]; k];
    for i in 0..n {
        if s[i] == 1 {
            v[i % k].push(i / k);
        }
    }

    let mut ans = total_ones;

    for c in v {
        let other_ones = total_ones - c.len();

        ans = min(ans, other_ones + min_operations_to_contiguous(c.clone()));
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
