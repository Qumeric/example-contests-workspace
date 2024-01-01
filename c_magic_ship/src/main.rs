//{"name":"C. Magic Ship","group":"Codeforces - Educational Codeforces Round 60 (Rated for Div. 2)","url":"https://codeforces.com/problemset/problem/1117/C","interactive":false,"timeLimit":2000,"tests":[{"input":"0 0\n4 6\n3\nUUU\n","output":"5\n"},{"input":"0 3\n0 0\n3\nUDD\n","output":"3\n"},{"input":"0 0\n0 1\n1\nL\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CMagicShip"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let x1 = input.read_long();
    let y1 = input.read_long();

    let x2 = input.read_long() - x1;
    let y2 = input.read_long() - y1;

    let n = input.read_size();
    let s = input.read_str();

    let mut x_full = 0;
    let mut y_full = 0;
    for c in s.to_string().chars() {
        let d = match c {
            'U' => (0, 1),
            'D' => (0, -1),
            'L' => (-1, 0),
            'R' => (1, 0),
            _ => unreachable!(),
        };

        x_full -= d.0;
        y_full -= d.1;
    }

    let mut l = 0;
    let mut r = i64::MAX / 3;

    while l + 1 < r {
        let m = (l + r) / 2;

        let circles = m / n as i64;
        let last_swim = m as usize % n;
        let x_circles = x_full * circles;
        let y_circles = y_full * circles;

        let mut x3 = x2 + x_circles;
        let mut y3 = y2 + y_circles;

        for c in s.to_string().chars().take(last_swim) {
            let d = match c {
                'U' => (0, 1),
                'D' => (0, -1),
                'L' => (-1, 0),
                'R' => (1, 0),
                _ => unreachable!(),
            };
            x3 -= d.0;
            y3 -= d.1;
        }

        if m >= x3.abs() + y3.abs() {
            r = m;
        } else {
            l = m;
        }
    }

    if l > i64::MAX / 10 {
        out.print_line(-1);
        return;
    } else {
        out.print_line(r);
        return;
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
    tester::run_tests();
}
//END MAIN
