//{"name":"E. XOR Guessing","group":"Codeforces - Educational Codeforces Round 71 (Rated for Div. 2)","url":"https://codeforces.com/problemset/problem/1207/E","interactive":true,"timeLimit":1000,"tests":[{"input":"0\n32\n","output":"? 3 5 6\n? 32 24 37\n! 5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EXORGuessing"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    out.print("? ");
    for i in 1..=100 {
        out.print(i);
        out.print(" ");
    }
    out.print_line("");
    let small = input.read_size();
    out.print("? ");
    for i in 1..=100 {
        out.print(i << 7);
        out.print(" ");
    }
    out.print_line("");
    let big = input.read_size();

    let small_mask = (1 << 7) - 1;
    let big_mask = small_mask << 7;

    let result = (small & big_mask) + (big & small_mask);
    out.print_line(("!", result))
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
