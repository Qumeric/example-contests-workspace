//{"name":"D. Max Median","group":"Codeforces - Codeforces Round 703 (Div. 2)","url":"https://codeforces.com/problemset/problem/1486/D","interactive":false,"timeLimit":2000,"tests":[{"input":"5 3\n1 2 3 2 1\n","output":"2\n"},{"input":"4 2\n1 2 3 4\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DMaxMedian"}}}

use std::cmp::max;

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::treap_map::TreapSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve1(n: usize, k: usize, a: Vec<i64>) -> i64 {
    let mut t = TreapSet::<(i64, usize)>::new();
    let mut i = 0;
    while i < k {
        t.insert((a[i], i));
        i += 1;
    }
    let mut ans = 0;

    while i <= n {
        let m = t.get_at((k - 1) / 2).unwrap();
        ans = max(ans, m.0);
        if i < n {
            t.remove(&(a[i - k], i - k));
            t.insert((a[i], i));
        }
        i += 1;
    }
    ans
}

// TODO: apparetnly it's wrong? Probably it's not enough to check just k, k+1 (and likely even k+2 even though it's TLE)
// Also it doesn't need kth order -- we can just keep heaps
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let a = input.read_long_vec(n);

    let mut a1 = solve1(n, k, a.clone());
    if k < n {
        // hm it worked better when minim was here -- so answer is too large it seems?
        a1.maxim(solve1(n, k + 1, a.clone()));
    }
    if k < n - 1 {
        a1.maxim(solve1(n, k + 2, a));
    }
    out.print_line(a1);
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
