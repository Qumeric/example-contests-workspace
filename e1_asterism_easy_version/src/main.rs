//{"name":"E1. Asterism (Easy Version)","group":"Codeforces - Codeforces Round 654 (Div. 2)","url":"https://codeforces.com/problemset/problem/1371/E1","interactive":false,"timeLimit":1000,"tests":[{"input":"3 2\n3 4 5\n","output":"1\n3\n"},{"input":"4 3\n2 3 5 6\n","output":"2\n3 4\n"},{"input":"4 3\n9 1 1 1\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"E1AsterismEasyVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let p = input.read_size();

    let mut a = input.read_size_vec(n);
    a.sort();

    let mut ans = vec![];
    for x in 1..=2000 {
        let mut to_add = true;
        for i in 0..n {
            let first_possible = a[i].saturating_sub(x);
            let len = (i + 1).saturating_sub(first_possible);

            if len % p == 0 {
                to_add = false;
                break;
            }
        }
        if to_add {
            ans.push(x);
        }
    }

    out.print_line(ans.len());
    out.print_line(ans);
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
