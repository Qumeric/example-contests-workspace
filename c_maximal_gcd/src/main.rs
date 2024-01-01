//{"name":"C. Maximal GCD","group":"Codeforces - Educational Codeforces Round 20","url":"https://codeforces.com/problemset/problem/803/C","interactive":false,"timeLimit":1000,"tests":[{"input":"6 3\n","output":"1 2 3\n"},{"input":"8 2\n","output":"2 6\n"},{"input":"5 3\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CMaximalGCD"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::integer_sqrt::IntegerSqrt;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let k = input.read_size();

    if k > 500_000 {
        out.print_line(-1);
        return;
    }

    if k == 1 {
        out.print_line(n);
        return;
    }

    let min = (1 + k) * k / 2;

    if min > n {
        out.print_line(-1);
        return;
    }

    let max_d = (n as i64).upper_sqrt() as usize;

    for i in 1..=max_d {
        if n % i != 0 {
            continue;
        }
        let d = n / i;
        if min * d <= n {
            let mut ans: Vec<usize> = vec![];
            for j in 1..k {
                ans.push(j * d);
            }
            ans.push(n - k * (k - 1) / 2 * d);
            out.print_line(ans);
            return;
        }
    }

    for d in (1..max_d).rev() {
        if n % d != 0 {
            continue;
        }

        if min * d <= n {
            let mut ans: Vec<usize> = vec![];
            for j in 1..k {
                ans.push(j * d);
            }
            ans.push(n - k * (k - 1) / 2 * d);
            out.print_line(ans);
            return;
        }
    }

    out.print_line(-1);
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
}
//END MAIN
