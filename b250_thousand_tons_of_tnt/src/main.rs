//{"name":"B. 250 Thousand Tons of TNT","group":"Codeforces - Codeforces Round 909 (Div. 3)","url":"https://codeforces.com/contest/1899/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n2\n1 2\n6\n10 2 3 6 1 3\n4\n1000000000 1000000000 1000000000 1000000000\n15\n60978 82265 78961 56708 39846 31071 4913 4769 29092 91348 64119 72421 98405 222 14294\n8\n19957 69913 37531 96991 57838 21008 14207 19198\n","output":"1\n9\n0\n189114\n112141\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"B250ThousandTonsOfTNT"}}}

use std::cmp::max;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn check(k: usize, a: &Vec<u64>) -> u64 {
    let mut v = vec![];
    let trucks = a.len() / k;
    for i in 0..trucks {
        let mut cur = 0;
        for j in 0..k {
            cur += a[i * k + j];
        }
        v.push(cur);
    }
    v.sort();
    v.last().unwrap() - v.first().unwrap()
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_u64_vec(n);

    let mut ans = 0;

    let mut i = 1;
    while i * i <= n {
        if n % i != 0 {
            i += 1;
            continue;
        }

        ans = max(ans, check(i, &a));
        ans = max(ans, check(n / i, &a));

        i += 1;
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
