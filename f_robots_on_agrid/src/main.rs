//{"name":"F. Robots on a Grid","group":"Codeforces - Codeforces Round 634 (Div. 3)","url":"https://codeforces.com/contest/1335/problem/F","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n1 2\n01\nRL\n3 3\n001\n101\n110\nRLL\nDLD\nULL\n3 3\n000\n000\n000\nRRD\nRLD\nULL\n","output":"2 1\n4 3\n2 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FRobotsOnAGrid"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();

    // we have pseudoforest
    // on each pseudotree mark any vertex on cycle
    // now calculate distance from this vector mod (cycle.len())
    // now for each distance say if there is black vertex
    // this is easy but I am lazy to implement
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
