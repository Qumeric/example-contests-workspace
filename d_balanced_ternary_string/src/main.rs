//{"name":"D. Balanced Ternary String","group":"Codeforces - Codeforces Round 531 (Div. 3)","url":"https://codeforces.com/contest/1102/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n121\n","output":"021\n"},{"input":"6\n000000\n","output":"001122\n"},{"input":"6\n211200\n","output":"211200\n"},{"input":"6\n120110\n","output":"120120\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DBalancedTernaryString"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let mut s = input.read_str();

    let a = s.iter().filter(|&c| c == b'0').count();
    let b = s.iter().filter(|&c| c == b'1').count();
    let c = s.iter().filter(|&c| c == b'2').count();

    let need = (n / 3) as i64;
    let need_a = need - a as i64;
    let need_b = need - b as i64;
    let need_c = need - c as i64;

    let mut vec = vec![need_a, need_b, need_c];

    // out.print_line(&vec);

    for i in 0..n {
        if s[i] == b'2' {
            if vec[2] < 0 {
                vec[2] += 1;
                if vec[0] > 0 {
                    s[i] = b'0';
                    vec[0] -= 1;
                } else if vec[1] > 0 {
                    s[i] = b'1';
                    vec[1] -= 1;
                }
            }
            continue;
        }

        if s[i] == b'1' {
            if vec[1] < 0 {
                if vec[0] > 0 {
                    vec[1] += 1;
                    s[i] = b'0';
                    vec[0] -= 1;
                }
            }
        }
    }

    // for i in 0..n {
    //     if s[i] == b'1' {
    //         if vec[1] < 0 {
    //             vec[1] += 1;
    //             if vec[0] > 0 {
    //                 s[i] = b'0';
    //                 vec[0] -= 1;
    //             } else {
    //                 s[i] = b'2';
    //                 vec[2] -= 1;
    //             }
    //         }
    //     }
    // }

    // out.print_line(&vec);
    // out.print_line(&s);
    for i in (0..n).rev() {
        if s[i] == b'0' {
            if vec[0] < 0 {
                vec[0] += 1;
                if vec[2] > 0 {
                    s[i] = b'2';
                    vec[2] -= 1;
                } else if vec[1] > 0 {
                    s[i] = b'1';
                    vec[1] -= 1;
                }
            }
            continue;
        }

        if s[i] == b'1' {
            if vec[1] < 0 {
                if vec[2] > 0 {
                    vec[1] += 1;
                    s[i] = b'2';
                    vec[2] -= 1;
                }
            }
        }
    }

    out.print_line(s);
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
