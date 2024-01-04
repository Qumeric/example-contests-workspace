//{"name":"F. Number of Subsequences","group":"Codeforces - Codeforces Round 674 (Div. 3)","url":"https://codeforces.com/problemset/problem/1426/F","interactive":false,"timeLimit":1000,"tests":[{"input":"6\nac?b?c\n","output":"24\n"},{"input":"7\n???????\n","output":"2835\n"},{"input":"9\ncccbbbaaa\n","output":"0\n"},{"input":"5\na???c\n","output":"46\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FNumberOfSubsequences"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::mod_int::{ModInt7, ModInt9};
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let s = input.read_str();

    let mut dp = vec![
        (
            ModInt7::default(),
            ModInt7::default(),
            ModInt7::default(),
            ModInt7::new(1)
        );
        n + 1
    ];

    for i in 0..n {
        let c = s[i];
        match c {
            b'a' => {
                dp[i + 1].0 = dp[i].0 + dp[i].3;
                dp[i + 1].1 = dp[i].1;
                dp[i + 1].2 = dp[i].2;
                dp[i + 1].3 = dp[i].3;
            }
            b'b' => {
                dp[i + 1].0 = dp[i].0;
                dp[i + 1].1 = dp[i].0 + dp[i].1;
                dp[i + 1].2 = dp[i].2;
                dp[i + 1].3 = dp[i].3;
            }
            b'c' => {
                dp[i + 1].0 = dp[i].0;
                dp[i + 1].1 = dp[i].1;
                dp[i + 1].2 = dp[i].1 + dp[i].2;
                dp[i + 1].3 = dp[i].3;
            }
            // Sum all?
            _ => {
                dp[i + 1].0 = dp[i].0 * 3.into() + dp[i].3;
                dp[i + 1].1 = dp[i].1 * 3.into() + dp[i].0;
                dp[i + 1].2 = dp[i].2 * 3.into() + dp[i].1;
                dp[i + 1].3 = dp[i].3 * 3.into();
            }
        }
    }

    out.print_line(dp[n].2);
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
