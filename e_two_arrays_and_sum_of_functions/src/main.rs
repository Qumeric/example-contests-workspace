//{"name":"E. Two Arrays and Sum of Functions","group":"Codeforces - Codeforces Round 560 (Div. 3)","url":"https://codeforces.com/contest/1165/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n1 8 7 2 4\n9 7 2 9 3\n","output":"646\n"},{"input":"1\n1000000\n1000000\n","output":"757402647\n"},{"input":"2\n1 3\n4 2\n","output":"20\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ETwoArraysAndSumOfFunctions"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::mod_int::ModIntF;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let mut a = input.read_size_vec(n);
    let mut b = input.read_size_vec(n);
    b.sort();

    let mut cnt = n;
    for i in 0..n {
        a[i] *= cnt;
        cnt += n - i - 1;
        cnt -= i + 1;
    }
    a.sort();
    a.reverse();
    // out.print_line(&a);
    // out.print_line(&b);

    let mut ans = ModIntF::new(0);

    for i in 0..n {
        ans += ModIntF::new(a[i] as i64) * ModIntF::new(b[i] as i64);
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
