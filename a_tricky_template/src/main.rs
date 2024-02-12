//{"name":"A. Tricky Template","group":"Codeforces - Educational Codeforces Round 161 (Rated for Div. 2)","url":"https://codeforces.com/contest/1922/problem/A","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1\na\nb\nc\n2\naa\nbb\naa\n10\nmathforces\nluckforces\nadhoccoder\n3\nacc\nabd\nabc\n","output":"YES\nNO\nYES\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ATrickyTemplate"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_str();
    let b = input.read_str();
    let c = input.read_str();

    let mut not_matched = false;
    for i in 0..n {
        let x = a[i];
        let y = b[i];
        let z = c[i];

        if z != x && z != y {
            not_matched = true;
        }
    }
    out.print_line(not_matched);
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
