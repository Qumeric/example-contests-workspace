//{"name":"C. Need for Pink Slips","group":"Codeforces - Codeforces Round 730 (Div. 2)","url":"https://codeforces.com/problemset/problem/1543/C","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n0.2 0.2 0.6 0.2\n0.4 0.2 0.4 0.8\n0.4998 0.4998 0.0004 0.1666\n0.3125 0.6561 0.0314 0.2048\n","output":"1.532000000000\n1.860000000000\n5.005050776521\n4.260163673896\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CNeedForPinkSlips"}}}

use std::cmp::min;

use algo_lib::io::input::{Input, Readable};
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{self, RecursiveFunction2};

type PreCalc = ();

const EPS: f64 = 0.000000000001f64;

// TODO: unfinihsed: there is some bug apparently?
fn fill_map(
    result: &mut Vec<(usize, f64, f64)>,
    a: Option<f64>,
    b: Option<f64>,
    c: f64,
    v: f64,
    step_n: usize,
    prob_got_here: f64,
) {
    result.push((step_n, c, prob_got_here));

    let a1 = if let Some(a) = a {
        if a >= v {
            Some(a - v)
        } else {
            None
        }
    } else {
        None
    };

    let b1 = if let Some(b) = b {
        if b >= v {
            Some(b - v)
        } else {
            None
        }
    } else {
        None
    };

    match (a, b) {
        (None, None) => {
            return;
        }
        (None, Some(b)) => fill_map(
            result,
            None,
            b1,
            c + v.min(b),
            v,
            step_n + 1,
            prob_got_here * b,
        ),
        (Some(a), None) => fill_map(
            result,
            a1,
            None,
            c + v.min(a),
            v,
            step_n + 1,
            prob_got_here * a,
        ),
        (Some(a), Some(b)) => {
            // let a1 = if a >= v / 2 { Some(a - v / 2) } else { None };
            // let b1 = if b >= v / 2 { Some(b - v / 2) } else { None };

            let val = v.min(a) / 2f64;

            fill_map(
                result,
                a1,
                Some(b + val),
                c + val,
                v,
                step_n + 1,
                prob_got_here * a,
            );
            fill_map(
                result,
                Some(a + val),
                b1,
                c + val,
                v,
                step_n + 1,
                prob_got_here * b,
            );
        }
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let s = input.read_line();
    let s: Vec<&str> = s.split(' ').collect();
    let a: f64 = s[0].parse().unwrap();
    let b: f64 = s[1].parse().unwrap();
    let c: f64 = s[2].parse().unwrap();
    let v: f64 = s[3].parse().unwrap();

    if a > 0.201f64 {
        return;
    }

    out.print_line(format!("{} {} {} {}", a, b, c, v));
    let mut result = vec![];

    fill_map(&mut result, Some(a), Some(b), c, v, 1, 1f64);

    for (steps, val, prob) in result.iter() {
        out.print_line(format!("{} {} {}", steps, val, prob));
    }

    let ans = result.into_iter().fold(0f64, |acc, (steps, val, prob)| {
        acc + steps as f64 * val * prob
    });

    out.print_line(format!("{}", ans));
    out.print_line("");
    out.print_line("");
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
