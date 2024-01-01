//{"name":"E. Fish","group":"Codeforces - Codeforces Beta Round 16 (Div. 2 Only)","url":"https://codeforces.com/contest/16/problem/E","interactive":false,"timeLimit":3000,"tests":[{"input":"2\n0 0.5\n0.5 0\n","output":"0.500000 0.500000\n"},{"input":"5\n0 1 1 1 1\n0 0 0.5 0.5 0.5\n0 0.5 0 0.5 0.5\n0 0.5 0.5 0 0.5\n0 0.5 0.5 0.5 0\n","output":"1.000000 0.000000 0.000000 0.000000 0.000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EFish"}}}

use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dRead};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::number_ext::Power;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();

    let f: Arr2d<f64> = input.read_table(n, n);

    // alive fish, eaten fishes (includes alive fish)
    let mut dp: Arr2d<f64> = Arr2d::<f64>::new(n, 2.power(n), 0f64);

    for i in 0..n {
        dp[i][1 << i] = 1f64;
    }
    for eaten_fish in 1..n {
        for alive_fish in 0..n {
            // TODO need to generate bitmasks with 1 bit etc.
        }
    }
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
