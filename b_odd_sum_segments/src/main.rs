//{"name":"B. Odd Sum Segments","group":"Codeforces - Codeforces Round 575 (Div. 3)","url":"https://codeforces.com/contest/1196/problem/B","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n5 3\n7 18 3 14 1\n5 4\n1 2 3 4 5\n6 2\n1 2 8 4 10 2\n","output":"YES\n1 3 5\nNO\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BOddSumSegments"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let mut k = input.read_size();
    let a = input.read_size_vec(n);

    let mut ans = vec![];
    let mut sum = 0;
    for i in 0..n {
        if k == 1 {
            break;
        }
        sum += a[i];
        if sum % 2 == 1 {
            ans.push(i + 1);
            sum = 0;
            k -= 1;
        }
    }
    if k > 1 {
        out.print_line(false);
        return;
    }

    let mut sum = 0;
    let last = *ans.last().unwrap_or_else(|| &0);
    for i in last..n {
        sum += a[i];
    }
    if sum % 2 == 1 {
        ans.push(n);
        out.print_line(true);
        out.print_line(ans);
    } else {
        out.print_line(false);
    }
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
