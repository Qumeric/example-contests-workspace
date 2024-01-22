//{"name":"D. Two Strings Swaps","group":"Codeforces - Codeforces Round 498 (Div. 3)","url":"https://codeforces.com/contest/1006/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"7\nabacaba\nbacabaa\n","output":"4\n"},{"input":"5\nzcabd\ndbacz\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DTwoStringsSwaps"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_str();
    let b = input.read_str();
    let half = n / 2;
    let mut fours = vec![];
    for i in 0..half {
        fours.push(vec![a[i], a[n - 1 - i], b[i], b[n - 1 - i]]);
    }
    let mut ans = if n % 2 == 0 {
        0
    } else {
        if a[half] == b[half] {
            0
        } else {
            1
        }
    };

    for four in fours {
        let mut f = four.clone();
        f.sort();
        let mut ff = f.clone();
        ff.dedup();
        // out.print_line(&four);
        // out.print_line(&f);
        let res = match ff.len() {
            1 => 0,
            2 => {
                if f[0] == f[1] && f[2] == f[3] {
                    0
                } else {
                    1
                }
            }
            3 => {
                if four[0] == four[1] {
                    2
                } else {
                    1
                }
            }
            4 => 2,
            _ => panic!(),
        };
        // out.print_line(&res);
        ans += res;
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
