//{"name":"B. Math Show","group":"Codeforces - Educational Codeforces Round 28","url":"https://codeforces.com/contest/846/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"3 4 11\n1 2 3 4\n","output":"6\n"},{"input":"5 5 10\n1 2 4 8 16\n","output":"7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BMathShow"}}}

use std::cmp::max;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let m = input.read_long();

    let mut t = input.read_long_vec(k);
    t.sort();
    let full_t: i64 = t.iter().sum();

    let mut ans = 0;
    for completed in 0..=n {
        let mut cur_points = completed * (k + 1);
        let mut m_left = m - (completed as i64) * full_t;
        if m_left < 0 {
            break;
        }
        let mut ptr = 0;
        while m_left > 0 && ptr < k {
            for _ in 0..(n - completed) {
                m_left -= t[ptr];
                if m_left >= 0 {
                    cur_points += 1;
                } else {
                    break;
                }
            }
            ptr += 1;
        }
        ans = max(ans, cur_points);
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
