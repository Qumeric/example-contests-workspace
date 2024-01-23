//{"name":"C. Classy Numbers","group":"Codeforces - Educational Codeforces Round 50 (Rated for Div. 2)","url":"https://codeforces.com/problemset/problem/1036/C","interactive":false,"timeLimit":3000,"tests":[{"input":"4\n1 1000\n1024 1024\n65536 65536\n999999 1000001\n","output":"1000\n1\n0\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CClassyNumbers"}}}

use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = Vec<u64>;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, data: &PreCalc) {
    let l = input.read_u64();
    let r = input.read_u64();

    let larger_pos = data.upper_bound(&r);
    let smaller_pos = data.lower_bound(&l);
    // out.print_line((larger_pos, smaller_pos));
    let ans = larger_pos - smaller_pos;
    out.print_line(ans);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = vec![];
    pre_calc.push(1_000_000_000_000_000_000u64);
    for i in 0..18 {
        for j in i + 1..18 {
            for k in j + 1..18 {
                for a in 0..=9 {
                    for b in 0..=9 {
                        for c in 0..=9 {
                            let mut str = vec![0u64; 18];
                            str[i] = a;
                            str[j] = b;
                            str[k] = c;
                            let num: u64 =
                                str.iter().rev().fold(0u64, |acc, &digit| acc * 10 + digit);
                            pre_calc.push(num);
                        }
                    }
                }
            }
        }
    }
    pre_calc.sort();
    pre_calc.dedup();

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
