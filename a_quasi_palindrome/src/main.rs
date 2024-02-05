//{"name":"A. Quasi-palindrome","group":"Codeforces - Educational Codeforces Round 29","url":"https://codeforces.com/contest/863/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"131\n","output":"YES\n"},{"input":"320\n","output":"NO\n"},{"input":"2010200\n","output":"YES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AQuasiPalindrome"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn check(x: &Vec<u8>) -> bool {
    let n = x.len();
    for i in 0..n {
        if x[i] != x[n - 1 - i] {
            return false;
        }
    }
    true
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let mut n = input.read_str().into_iter().collect_vec();
    n.reverse();

    for i in 0..20 {
        if check(&n) {
            out.print_line(true);
            return;
        }
        n.push(b'0');
    }
    out.print_line(false);
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
