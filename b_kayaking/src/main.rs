//{"name":"B. Kayaking","group":"Codeforces - Educational Codeforces Round 29","url":"https://codeforces.com/contest/863/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n1 2 3 4\n","output":"1\n"},{"input":"4\n1 3 4 6 3 4 100 200\n","output":"5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BKayaking"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let mut w = input.read_size_vec(n * 2);

    w.sort();

    let mut ans = 1_000_000;
    for i in 0..2 * n {
        for j in (i + 1)..2 * n {
            let mut wc = w.clone();
            wc.remove(j);
            wc.remove(i);

            let mut cur = 0;
            for k in 0..(n - 1) {
                cur += wc[2 * k + 1] - wc[2 * k];
            }
            ans.minim(cur);
        }
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
