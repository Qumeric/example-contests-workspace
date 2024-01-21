//{"name":"D. Yarik and Musical Notes","group":"Codeforces - Codeforces Round 909 (Div. 3)","url":"https://codeforces.com/contest/1899/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n1\n2\n4\n3 1 3 2\n2\n1000 1000\n3\n1 1 1\n19\n2 4 1 6 2 8 5 4 2 10 5 10 8 7 4 3 2 6 10\n","output":"0\n2\n1\n3\n19\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DYarikAndMusicalNotes"}}}

use std::collections::BTreeMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let mut counters: BTreeMap<usize, usize> = Default::default();

    let mut ans = 0;
    for i in 0..n {
        ans += *counters.entry(a[i]).or_default();
        if a[i] == 2 {
            ans += *counters.entry(1).or_default();
        }
        if a[i] == 1 {
            ans += *counters.entry(2).or_default();
        }
        counters.entry(a[i]).and_modify(|x| *x += 1);
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
