//{"name":"C. Square Subsets","group":"Codeforces - Codeforces Round 448 (Div. 2)","url":"https://codeforces.com/problemset/problem/895/C","interactive":false,"timeLimit":4000,"tests":[{"input":"4\n1 1 1 1\n","output":"15\n"},{"input":"4\n2 2 2 2\n","output":"7\n"},{"input":"5\n1 2 4 5 8\n","output":"7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CSquareSubsets"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::mod_int::ModInt;
use algo_lib::numbers::number_ext::Power;
use algo_lib::value;
// use algo_lib::numbers::mod_int::ModInt7;

type PreCalc = ();

fn c2(x: i64) -> i64 {
    x * (x - 1) / 2
}

value!(Val7: i64 = 1_000_000_007);
pub type ModInt7 = ModInt<i64, Val7>;
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);
    // let n = 100_000;
    // let a = (1..=n).collect_vec();

    let primes = vec![
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67,
    ];

    const LIM: usize = 19;

    let mut masks = vec![0; 1 << LIM];

    for i in 0..n {
        let mut cur = a[i];
        let mut mask = 0;
        for j in 0..LIM {
            while cur % primes[j] == 0 {
                cur /= primes[j];
                mask ^= 1 << j;
            }
        }
        masks[mask] += 1;
    }
    let mut dp = vec![ModInt7::new(0); 1 << LIM];
    dp[0] = ModInt7::new(1);

    // out.print_line(&masks);
    // out.print_line(&dp);

    for i in 0..(1 << LIM) {
        if masks[i] == 0 {
            continue;
        }

        let zeros = ModInt7::new(2).power(masks[i] - 1);
        let ones = zeros.clone();

        let mut new_dp = vec![ModInt7::new(0); 1 << LIM];
        for j in 0..(1 << LIM) {
            new_dp[j] = dp[j] * zeros + dp[j ^ i] * ones;
        }
        dp = new_dp;
        // out.print_line(&dp);
    }
    out.print_line(dp[0] - ModInt7::new(1));

    // let mut ans = 0;
    // let mut ans2 = 0;
    // for i in 0..(1 << 18) {
    //     if masks[i] > 0 {
    //         ans += 1;
    //         ans2 += masks[i];
    //     }
    // }
    // out.print_line(ans);
    // out.print_line(ans2);
    // out.print_line(ans * ans / 1_000_000);
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
