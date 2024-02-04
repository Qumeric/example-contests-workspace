//{"name":"E. New Year Parties","group":"Codeforces - Codeforces Round 611 (Div. 3)","url":"https://codeforces.com/contest/1283/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1 2 4 4\n","output":"2 4\n"},{"input":"9\n1 1 8 8 8 4 4 4 4\n","output":"3 8\n"},{"input":"7\n4 3 7 1 4 3 3\n","output":"3 6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ENewYearParties"}}}

use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let x = input.read_size_vec(n);
    let mut c = vec![0; n + 2];
    for i in x {
        c[i] += 1;
    }

    let mut max_occupy = vec![0; n + 2];
    for i in 1..=n {
        let mut cur = c[i].clone();
        if max_occupy[i - 1] == 0 && cur > 0 {
            max_occupy[i - 1] = 1;
            cur -= 1;
        }
        if max_occupy[i] == 0 && cur > 0 {
            max_occupy[i] = 1;
            cur -= 1;
        }
        if max_occupy[i + 1] == 0 && cur > 0 {
            max_occupy[i + 1] = 1;
        }
    }

    let mut min_occupy = 0;
    let mut i = 0;
    while i <= n {
        if c[i] == 0 {
            i += 1;
        } else {
            min_occupy += 1;
            i += 3;
        }
    }

    out.print_line((min_occupy, max_occupy.into_iter().sum::<usize>()));
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
