//{"name":"C. Need for Pink Slips","group":"Codeforces - Codeforces Round 730 (Div. 2)","url":"https://codeforces.com/problemset/problem/1543/C","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n0.2 0.2 0.6 0.2\n0.4 0.2 0.4 0.8\n0.4998 0.4998 0.0004 0.1666\n0.3125 0.6561 0.0314 0.2048\n","output":"1.532000000000\n1.860000000000\n5.005050776521\n4.260163673896\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CNeedForPinkSlips"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable5, RecursiveFunction5};
use algo_lib::numbers::real::RealReader;

type PreCalc = ();

const EPS: f64 = 0.000000000001f64;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let a = input.read_real().0;
    let b = input.read_real().0;
    let c = input.read_real().0;
    let v = input.read_real().0;

    // if a > 0.201f64 {
    //     return;
    // }

    // out.print_line(format!("{} {} {} {}", a, b, c, v));

    let mut ans = 0f64;
    let mut fill_map = RecursiveFunction5::new(|f, a, b, c, depth, prob| {
        // out.print_line(format!("depth: {}; prob: {}", depth, prob));
        // out.print_line(format!("{} {} {}", a, b, c));
        ans += depth as f64 * prob * c;
        if depth > 20 {
            return;
        }

        if a > EPS {
            if b > EPS {
                if a <= v {
                    f.call(0f64, b + a / 2f64, c + a / 2f64, depth + 1, prob * a)
                } else {
                    f.call(a - v, b + v / 2f64, c + v / 2f64, depth + 1, prob * a);
                }
            } else {
                let delta = a.min(v);
                f.call(a - delta, b, c + delta, depth + 1, prob * a);
            }
        }
        if b > EPS {
            if a > EPS {
                if b <= v {
                    f.call(a + b / 2f64, 0f64, c + b / 2f64, depth + 1, prob * b)
                } else {
                    f.call(a + v / 2f64, b - v, c + v / 2f64, depth + 1, prob * b);
                }
            } else {
                let delta = b.min(v);
                f.call(a, b - delta, c + delta, depth + 1, prob * b);
            }
        }
    });

    fill_map.call(a, b, c, 1, 1f64);
    // let ans = result.into_iter().fold(0f64, |acc, (steps, val, prob)| {
    //     acc + steps as f64 * val * prob
    // });

    out.print_line(format!("{}", ans));
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

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
}
//END MAIN
