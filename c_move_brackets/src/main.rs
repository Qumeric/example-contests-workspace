//{"name":"C. Move Brackets","group":"Codeforces - Codeforces Round 653 (Div. 3)","url":"https://codeforces.com/problemset/problem/1374/C","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n2\n)(\n4\n()()\n8\n())()()(\n10\n)))((((())\n","output":"1\n0\n1\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CMoveBrackets"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn do_try(s: &Vec<char>, left: usize, mut right: usize) -> bool {
    // just check right?
    let mut cnt = 0;
    for &c in s {
        if c == '(' {
            cnt += 1;
        } else {
            if cnt > 0 {
                cnt -= 1;
            } else if right > 0{
                right -= 1;
            } else {
                return false;
            }
        }
    }
    true
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n: usize = input.read();
    let s: String = input.read_line();
    let s = s.chars().collect();

    for count in 0..=n {
        for left in 0..=count {
            let right = count - left;
            if do_try(&s, left, right) {
                out.print_line(count);
                return;
            }
        }
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
}
//END MAIN
