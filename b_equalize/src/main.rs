//{"name":"B. Equalize","group":"Codeforces - Codeforces Round 924 (Div. 2)","url":"https://codeforces.com/contest/1928/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n2\n1 2\n4\n7 1 4 1\n3\n103 102 104\n5\n1 101 1 100 1\n5\n1 10 100 1000 1\n2\n3 1\n3\n1000000000 999999997 999999999\n","output":"2\n2\n3\n2\n1\n1\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BEqualize"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let mut a = input.read_size_vec(n);
    a.sort();
    a.dedup();

    let mut ptr = 0;
    let mut ans = 0;
    for i in 0..a.len() {
        while ptr < a.len() && a[ptr] - a[i] < n {
            ptr += 1;
        }
        ans.maxim(ptr - i);
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
