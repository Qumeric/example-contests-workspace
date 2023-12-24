//{"name":"C. Watering an Array","group":"Codeforces - Codeforces Round 917 (Div. 2)","url":"https://codeforces.com/contest/1917/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n3 4 4\n1 2 3\n1 3 2 3\n6 2 3\n6 1 2 4 1 5\n6 6\n5 1 1\n0 5 0 5 0\n5\n1 1 1\n1\n1\n3 4 6\n1 2 3\n1 3 2 3\n","output":"4\n3\n0\n1\n5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CWateringAnArray"}}}

use std::cmp::max;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

// TODO: almost correct but some limit is too low so fst
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n: usize = input.read(); // <= 2000
    let k: usize = input.read(); // <= 1e5
    let d: i64 = input.read(); // <= 1e9

    let mut a: Vec<i64> = input.read_vec(n);
    let mut v: Vec<i64> = input.read_vec(k);

    let mut ans = 0;

    for i in 0..n {
        if a[i] == (i + 1) as i64 {
            ans += 1;
        }
    }
    ans += (d - 1) / 2;

    v.pop();

    for (day, &b) in v.iter().enumerate() {
        let mut cans = 0;
        for i in 0..b {
            a[i as usize] += 1;
        }

        for i in 0..n {
            if a[i] == (i + 1) as i64 {
                cans += 1;
            }
        }
        cans += (d - 2 - day as i64) / 2;

        ans = max(ans, cans);
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
