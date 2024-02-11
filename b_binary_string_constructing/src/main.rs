//{"name":"B. Binary String Constructing","group":"Codeforces - Codeforces Round 494 (Div. 3)","url":"https://codeforces.com/contest/1003/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"2 2 1\n","output":"1100\n"},{"input":"3 3 3\n","output":"101100\n"},{"input":"5 3 6\n","output":"01010100\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BBinaryStringConstructing"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let mut a = input.read_size();
    let mut b = input.read_size();
    let x = input.read_size();

    let mut cur = if a > b { 0 } else { 1 };
    let mut ans = vec![cur];
    if cur == 0 {
        a -= 1;
    } else {
        b -= 1;
    }
    for i in 0..x {
        cur = 1 - cur;
        ans.push(cur);
        if cur == 0 {
            a -= 1;
        } else {
            b -= 1;
        }
    }

    for e in ans {
        if e == 0 {
            while a > 0 {
                out.print(0);
                a -= 1;
            }
        } else {
            while b > 0 {
                out.print(1);
                b -= 1;
            }
        }
        out.print(e)
    }
    out.print_line("");
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
