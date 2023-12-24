//{"name":"C. Alternating Subsequence","group":"Codeforces - Codeforces Round 636 (Div. 3)","url":"https://codeforces.com/problemset/problem/1343/C","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n5\n1 2 3 -1 -2\n4\n-1 -2 -1 -3\n10\n-2 8 3 8 -4 -15 5 -2 -3 1\n6\n1 -1000000000 1 -1000000000 1 -1000000000\n","output":"2\n-1\n6\n-2999999997\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CAlternatingSubsequence"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n: usize = input.read();
    let a: Vec<i64> = input.read_vec(n);

    let mut max_sum = 0;
    let mut current_max = a[0];
    for &item in &a[1..] {
        if (current_max > 0 && item > 0) || (current_max < 0 && item < 0) {
            current_max = current_max.max(item);
        } else {
            max_sum += current_max;
            current_max = item;
        }
    }
    max_sum += current_max;
    out.print_line(max_sum);
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
