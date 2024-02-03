//{"name":"E. Connected Component on a Chessboard","group":"Codeforces - Codeforces Round 575 (Div. 3)","url":"https://codeforces.com/contest/1196/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n1 1\n1 4\n2 5\n","output":"YES\n2 2\n1 2\nYES\n2 3\n1 3\n3 3\n2 2\n2 4\nYES\n2 3\n2 4\n2 5\n1 3\n1 5\n3 3\n3 5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EConnectedComponentOnAChessboard"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let mut b = input.read_size();
    let mut w = input.read_size();
    let mut swapped = false;

    if b > w {
        let tmp = b;
        b = w;
        w = tmp;
        swapped = true;
    }

    let start = (1_000_000, 1_000_000);

    let mut squares = vec![];

    let mut cur = (start.0, start.1);
    let mut is_black = false;

    while b > 0 || w > 0 {
        if w > b {
            if is_black {
                if b == 0 {
                    out.print_line(false);
                    return;
                }
                squares.push(cur);
                squares.push((cur.0, cur.1 - 1));
                squares.push((cur.0, cur.1 + 1));
                w -= 2;
                b -= 1;
            } else {
                squares.push(cur);
                w -= 1;
            }
        } else {
            squares.push(cur);
            squares.push((cur.0, cur.1 + 1));
            w -= 1;
            b -= 1;
        }
        cur = (cur.0 + 1, cur.1);
        is_black = !is_black;
    }

    if w + b == 0 {
        out.print_line(true);
        if swapped {
            for i in 0..squares.len() {
                squares[i].0 += 1;
            }
        }
        for s in squares {
            out.print_line(s);
        }
    } else {
        out.print_line(false);
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
