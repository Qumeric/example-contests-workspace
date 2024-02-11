//{"name":"F - S = 1","group":"AtCoder - KAJIMA CORPORATION CONTEST 2024（AtCoder Beginner Contest 340）","url":"https://atcoder.jp/contests/abc340/tasks/abc340_f","interactive":false,"timeLimit":2000,"tests":[{"input":"3 5\n","output":"1 1\n"},{"input":"-2 0\n","output":"0 1\n"},{"input":"8752654402832944 -6857065241301125\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FS1"}}}

use std::i128;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::gcd::extended_gcd;

type PreCalc = ();

fn solve1(x: i128, y: i128) -> Option<(i128, i128)> {
    assert_ne!(x, 0);
    // b*x == 2
    if y == 0 {
        // |b*x| = 2
        return if x.abs() == 1 {
            Some((0, 2))
        } else if x.abs() == 2 {
            Some((0, 1))
        } else {
            None
        };
    }
    // b = (a*y-2)/x | b = (a*y+2)/x
    let (gcd, inv, other) = extended_gcd(y, x);
    // out.print_line((gcd, inv, other));
    if gcd == 1 {
        let a = (2 * inv + x.abs()) % x;
        let b1 = (a * y - 2) / x;
        let b2 = (a * y + 2) / x;
        if b1 * x == a * y - 2 {
            return Some((a, b1));
        }
        if b2 * x == a * y + 2 {
            return Some((a, b2));
        }
    } else if gcd == 2 {
        // TODO
    }
    None
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let x = input.read_i128();
    let y = input.read_i128();

    // shall be 2
    // let area = (a * y - b * x).abs_diff();

    if x != 0 {
        if let Some((a, b)) = solve1(x, y) {
            out.print_line((a, b));
            return;
        }
    } else {
        if let Some((b, a)) = solve1(y, x) {
            out.print_line((a, b));
            return;
        }
    }
    out.print_line(-1);
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
