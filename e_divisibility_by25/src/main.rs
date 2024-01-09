//{"name":"E. Divisibility by 25","group":"Codeforces - Codeforces Round 486 (Div. 3)","url":"https://codeforces.com/contest/988/problem/E","interactive":false,"timeLimit":1000,"tests":[{"input":"5071\n","output":"4\n"},{"input":"705\n","output":"1\n"},{"input":"1241367\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EDivisibilityBy25"}}}

use std::cmp::min;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve2(s: Str<'_>) -> usize {
    let n = s.len();
    let positions_of_zero: Vec<usize> = s
        .iter()
        .enumerate()
        .filter_map(|(idx, ch)| if ch == b'0' { Some(idx) } else { None })
        .collect();

    let position_of_two: Option<usize> = s
        .iter()
        .enumerate()
        .filter_map(|(idx, ch)| if ch == b'2' { Some(idx) } else { None })
        .last();

    let position_of_five: Option<usize> = s
        .iter()
        .enumerate()
        .filter_map(|(idx, ch)| if ch == b'5' { Some(idx) } else { None })
        .last();

    let num_zeros = positions_of_zero.len();
    let mut ans = 1_000_000_000usize;
    if num_zeros >= 2 {
        ans = min(
            ans,
            (n - 1).abs_diff(positions_of_zero[num_zeros - 1])
                + (n - 2).abs_diff(positions_of_zero[num_zeros - 2]),
        )
    }

    if let (Some(two), Some(five)) = (position_of_two, position_of_five) {
        // out.print_line(("pos 2", two, "pos 5", five));
        if num_zeros == 0 || !(five == 0 && positions_of_zero[0] == 1) {
            if two == n - 1 {
                ans = min(ans, (n - 1).abs_diff(five) + (n - 2).abs_diff(two) - 1);
            } else if five < two {
                ans = min(ans, (n - 1).abs_diff(five) + (n - 2).abs_diff(two) + 1);
            } else {
                ans = min(ans, (n - 1).abs_diff(five) + (n - 2).abs_diff(two));
            }
        }
    }

    if let Some(five) = position_of_five {
        if num_zeros > 0 {
            let zero = positions_of_zero[num_zeros - 1];
            if five == n - 1 {
                ans = min(ans, (n - 1).abs_diff(zero) + (n - 2).abs_diff(five) - 1)
            } else if zero < five {
                ans = min(ans, (n - 1).abs_diff(zero) + (n - 2).abs_diff(five) + 1)
            } else {
                ans = min(ans, (n - 1).abs_diff(zero) + (n - 2).abs_diff(five))
            }
        }
    }

    return ans;
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let mut s = input.read_str();
    let n = s.len();
    for i in 0..n {
        if s[i] == b'7' {
            s[i] = b'2';
        }
    }

    let mut ans = solve2(s.clone());

    if n > 2 {
        if s[1] == b'0' {
            for i in 2..n {
                if s[i] != b'0' {
                    s[1] = s[i];
                    s[i] = b'0';
                    ans = min(ans, solve2(s.clone()) + (i - 1));
                    break;
                }
            }
        }
    }

    if ans == 1_000_000_000usize {
        out.print_line(-1);
    } else {
        out.print_line(ans);
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
    // tester::run_tests();
    // don't forget to set test_type = Single if you do it
    tester::stress_test(run, tester::check);
}
//END MAIN
