//{"name":"C. Good String","group":"Codeforces - Codeforces Round 560 (Div. 3)","url":"https://codeforces.com/contest/1165/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"4\ngood\n","output":"0\ngood\n"},{"input":"4\naabc\n","output":"2\nab\n"},{"input":"3\naaa\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CGoodString"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let s = input.read_str();

    let mut cur = s[0];

    let mut ans = vec![];

    let mut i = 1;
    while i < n {
        if s[i] == cur {
            i += 1;
            continue;
        }
        ans.push(cur);
        ans.push(s[i]);
        i += 1;
        if i < n {
            cur = s[i];
        }
    }
    out.print_line(n - ans.len());
    for i in ans {
        out.print(i as char);
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
    // don't forget to set test_type = Single if you do it
    // tester::stress_test(run, tester::check);
}
//END MAIN
