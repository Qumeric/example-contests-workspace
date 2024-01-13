//{"name":"B. Summation Game","group":"Codeforces - Codeforces Round 919 (Div. 2)","url":"https://codeforces.com/contest/1920/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"8\n1 1 1\n1\n4 1 1\n3 1 2 4\n6 6 3\n1 4 3 2 5 6\n6 6 1\n3 7 3 3 32 15\n8 5 3\n5 5 3 3 3 2 9 9\n10 6 4\n1 8 2 9 3 3 4 5 3 200\n2 2 1\n4 3\n2 1 2\n1 3\n","output":"0\n2\n0\n3\n-5\n-9\n0\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BSummationGame"}}}

use std::cmp::{max, min};

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let x = input.read_size();

    let mut a = input.read_long_vec(n);

    a.sort();

    let init_sum: i64 = a.clone().into_iter().sum();

    let mut alice_turns = k;

    a.reverse();

    let mut diff = 0;
    for i in 0..x {
        diff += -a[i] * 2;
    }

    let mut answers = vec![];
    answers.push(diff);
    for i in 0..k {
        diff += a[i];
        if i + x < n {
            diff -= a[i + x] * 2;
        }
        answers.push(diff);
    }

    // out.print_line(&answers);
    out.print_line(init_sum + answers.into_iter().max().unwrap());
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
