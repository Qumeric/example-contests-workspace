//{"name":"E. Modular Sequence","group":"Codeforces - Codeforces Round 924 (Div. 2)","url":"https://codeforces.com/contest/1928/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n5 8 3 28\n3 5 3 6\n9 1 5 79\n","output":"YES\n8 11 2 2 5\nNO\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EModularSequence"}}}

use std::cmp::min;

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let mut x = input.read_size();
    let y = input.read_size();
    let mut s = input.read_size();

    let c = x % y;
    if s < c * (n - 1) + x {
        out.print_line("NO");
        return;
    }
    s -= c * n;
    x -= c;

    if s % y != 0 {
        out.print_line("NO");
        return;
    }
    s /= y;
    x /= y;

    for len in 1..min(100, n) {
        let mut sum = 0;
        let mut cur = x;
        let mut ans = vec![];
        for i in 0..len {
            ans.push(cur);
            sum += cur;
            cur += 1;
        }
        cur -= 1;

        for _ in len..n {
            if sum + (cur + 1) <= s {
                cur += 1;
                sum += cur;
            } else {
                cur = 0;
            }
            ans.push(cur);
        }

        if sum == s {
            out.print_line("YES");
            // out.print_line(&ans);
            out.print_line(ans.map(|e| e * y + c).collect_vec());
            return;
        }
    }
    out.print_line("NO");
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
