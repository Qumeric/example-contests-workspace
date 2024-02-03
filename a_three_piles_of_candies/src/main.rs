//{"name":"A. Three Piles of Candies","group":"Codeforces - Codeforces Round 575 (Div. 3)","url":"https://codeforces.com/contest/1196/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n1 3 4\n1 10 100\n10000000000000000 10000000000000000 10000000000000000\n23 34 45\n","output":"4\n55\n15000000000000000\n51\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AThreePilesOfCandies"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let a = input.read_size();
    let b = input.read_size();
    let c = input.read_size();

    let mut v = vec![a, b, c];
    v.sort();
    let diff = v[1] - v[0];
    v[2] -= diff;
    out.print_line(v[1] + v[2] / 2);
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
