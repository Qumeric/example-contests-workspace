//{"name":"D. Flood Fill","group":"Codeforces - Codeforces Round 538 (Div. 2)","url":"https://codeforces.com/problemset/problem/1114/D","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n5 2 2 1\n","output":"2\n"},{"input":"8\n4 5 2 2 1 3 5 5\n","output":"4\n"},{"input":"1\n4\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DFloodFill"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let mut c = input.read_size_vec(n);

    let mut ans = 0;
    c.dedup();
    loop {
        let mut min_from = 0;
        let mut min_to = c.len();
        for i in 0..c.len() {
            for j in i + 1..c.len() {
                if c[i] == c[j] {
                    if j - i < min_to - min_from {
                        min_to = j;
                        min_from = i;
                    }
                    break;
                }
            }
        }
        if min_from == 0 && min_to == c.len() {
            ans += c.len() - 1;
            break;
        }
        // out.print_line(&c);
        // out.print_line((min_from, min_to));
        ans += min_to - min_from - 1;
        for _ in 0..(min_to - min_from) {
            c.remove(min_from + 1);
        }
    }

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
