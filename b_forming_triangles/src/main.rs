//{"name":"B. Forming Triangles","group":"Codeforces - Educational Codeforces Round 161 (Rated for Div. 2)","url":"https://codeforces.com/contest/1922/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n7\n1 1 1 1 1 1 1\n4\n3 2 1 3\n3\n1 2 3\n1\n1\n","output":"35\n2\n0\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BFormingTriangles"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let mut a = input.read_size_vec(n);
    a.sort();
    if n <= 2 {
        out.print_line(0);
        return;
    }
    let mut pairs = Vec::new();
    let mut count = 1i64;
    for i in 1..a.len() {
        if a[i] == a[i - 1] {
            count += 1;
        } else {
            pairs.push((a[i - 1], count));
            count = 1;
        }
    }
    pairs.push((a[a.len() - 1], count));
    let mut ans = 0;

    let mut sum = 0;
    for (_, amt) in pairs {
        if amt >= 2 {
            ans += amt * (amt - 1) / 2 * sum;
        }
        if amt >= 3 {
            ans += amt * (amt - 1) * (amt - 2) / 6;
        }
        sum += amt;
    }
    // non_degenerate a+b > c

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
