//{"name":"C. The Football Season","group":"Codeforces - Codeforces Round 592 (Div. 2)","url":"https://codeforces.com/problemset/problem/1244/C","interactive":false,"timeLimit":1000,"tests":[{"input":"30 60 3 1\n","output":"17 9 4\n"},{"input":"10 51 5 4\n","output":"-1\n"},{"input":"20 0 15 5\n","output":"0 0 20\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CTheFootballSeason"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::gcd::extended_gcd;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_i128();
    let p = input.read_i128();
    let w = input.read_i128();
    let d = input.read_i128();

    let (g, x, y) = extended_gcd(w, d);
    if p % g != 0 {
        out.print_line(-1);
        return;
    }

    let x0 = x * p / g;
    let y0 = y * p / g;

    // I want min number of draws. so
    // y0 - (w / g) * t >= 0
    // y0 >= (w / g) * t
    // y0 / (w / g) >= t

    let tt = y0 / (w / g);

    for t in vec![tt - 1, tt, tt + 1] {
        // for t in -1000..1000 {
        let xf = x0 + (d / g) * t;
        let yf = y0 - (w / g) * t;

        // out.print_line((xf, yf, n - xf - yf));
        if xf >= 0 && yf >= 0 && xf + yf <= n {
            out.print_line((xf, yf, n - xf - yf));
            return;
        }
    }

    // }
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
