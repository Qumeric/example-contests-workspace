//{"name":"A. Curriculum Vitae","group":"Codeforces - Educational Codeforces Round 28","url":"https://codeforces.com/contest/846/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n1 1 0 1\n","output":"3\n"},{"input":"6\n0 1 0 0 1 0\n","output":"4\n"},{"input":"1\n0\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ACurriculumVitae"}}}

use std::cmp::max;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let s = input.read_int_vec(n);

    let mut ans = 0;
    for i in 0..n {
        let mut zeros = 0;
        let mut ones = 0;
        for j in 0..=i {
            if s[j] == 0 {
                zeros += 1;
            }
        }
        for j in i..n {
            if s[j] == 1 {
                ones += 1;
            }
        }
        ans = max(ans, zeros + ones);
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
