//{"name":"E. Counting Arrays","group":"Codeforces - Educational Codeforces Round 33 (Rated for Div. 2)","url":"https://codeforces.com/problemset/problem/893/E","interactive":false,"timeLimit":3000,"tests":[{"input":"2\n6 3\n4 2\n","output":"36\n6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ECountingArrays"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::factorize_map;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::mod_utils::Combinations;
use algo_lib::numbers::number_ext::Power;

type PreCalc = Combinations<ModInt7>;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, c: &PreCalc) {
    let x = input.read_size();
    let y = input.read_size();

    let primes = factorize_map(x as i64);
    let mut ans = ModInt7::new(1);
    for (_k, v) in primes {
        let cur = c.c(y + v - 1, v);
        ans *= cur;
    }
    let m = ModInt7::new(2).power(y - 1);
    out.print_line(ans * m);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = Combinations::<ModInt7>::new(1_000_100);

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
