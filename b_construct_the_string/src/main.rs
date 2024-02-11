//{"name":"B. Construct the String","group":"Codeforces - Codeforces Round 634 (Div. 3)","url":"https://codeforces.com/contest/1335/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n7 5 3\n6 1 1\n6 6 1\n5 2 2\n","output":"tleelte\nqwerty\nvvvvvv\nabcde\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BConstructTheString"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_size();
    let b = input.read_size();

    let mut v = vec![];
    for i in 0..(a - b) {
        v.push(b'a');
    }
    for i in 0..b {
        v.push(b'a' + i as u8);
    }
    for i in 0..n {
        out.print(v[i % a] as char);
    }
    out.print_line("");

    // b = 3
    // a = 5
    // aaabcaaabc
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
