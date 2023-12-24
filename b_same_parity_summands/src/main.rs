//{"name":"B. Same Parity Summands","group":"Codeforces - Codeforces Round 640 (Div. 4)","url":"https://codeforces.com/problemset/problem/1352/B","interactive":false,"timeLimit":1000,"tests":[{"input":"8\n10 3\n100 4\n8 7\n97 2\n8 8\n3 10\n5 3\n1000000000 9\n","output":"YES\n4 2 4\nYES\n55 5 5 35\nNO\nNO\nYES\n1 1 1 1 1 1 1 1\nNO\nYES\n3 1 1\nYES\n111111110 111111110 111111110 111111110 111111110 111111110 111111110 111111110 111111120\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BSameParitySummands"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n: i64 = input.read();
    let k: i64 = input.read();
    if k > n {
        out.print_line("NO");
        return;
    }

    if n % 2 == k % 2 {
        out.print_line(true);
        let mut ans: Vec<i64> = (0..k).map(|e| 1).collect();
        ans[(k - 1) as usize] = n - (k - 1);
        out.print_line(ans);
        return;
    }
    if n % 2 == 0 {
        // k % 2 == 1
        if n < k * 2 {
            out.print_line(false);
            return;
        }
        out.print_line(true);
        let mut ans: Vec<i64> = (0..k).map(|e| 2).collect();
        ans[(k - 1) as usize] = n - (k - 1) * 2;
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
}
//END MAIN
