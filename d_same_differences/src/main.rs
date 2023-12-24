//{"name":"D. Same Differences","group":"Codeforces - Codeforces Round 719 (Div. 3)","url":"https://codeforces.com/problemset/problem/1520/D","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n6\n3 5 1 4 6 6\n3\n1 2 3\n4\n1 3 3 4\n6\n1 6 3 4 5 6\n","output":"1\n3\n3\n10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DSameDifferences"}}}

use std::collections::HashMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n: usize = input.read();
    let mut a: Vec<i64> = input.read_vec(n);

    for i in 0..n {
        a[i] -= i as i64;
    }

    let mut s: HashMap<i64, i64> = Default::default();

    for e in a {
        s.insert(e, *s.get(&e).unwrap_or(&0) + 1);
    }

    let mut ans = 0;
    for v in s.values() {
        ans += (v-1)*v/2;
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
}
//END MAIN
