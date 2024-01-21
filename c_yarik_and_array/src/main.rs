//{"name":"C. Yarik and Array","group":"Codeforces - Codeforces Round 909 (Div. 3)","url":"https://codeforces.com/contest/1899/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n5\n1 2 3 4 5\n4\n9 9 8 8\n6\n-1 4 -1 0 5 -4\n4\n-1 2 4 -3\n1\n-1000\n3\n101 -99 101\n20\n-10 5 -8 10 6 -10 7 9 -2 -6 7 2 -4 6 -1 7 -6 -7 4 1\n","output":"15\n17\n8\n4\n-1000\n101\n10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CYarikAndArray"}}}

use std::cmp::max;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn max_subarray(a: &Vec<i64>) -> i64 {
    let mut max_ending_here = 0i64;
    let mut max_so_far = a[0];

    for &x in a.iter() {
        max_ending_here = max_ending_here + x;
        if max_so_far < max_ending_here {
            max_so_far = max_ending_here;
        }
        if max_ending_here < 0 {
            max_ending_here = 0;
        }
    }

    max_so_far
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n);

    let mut i = 0;
    let mut arrays = vec![];

    while i < n {
        let mut parity = a[i] % 2 == 0;
        let mut cur = vec![a[i]];
        i += 1;
        while i < n && (a[i] % 2 == 0) != parity {
            cur.push(a[i]);
            parity = !parity;
            i += 1;
        }
        arrays.push(cur);
    }
    let mut ans = a[0];
    for a in arrays {
        ans = max(ans, max_subarray(&a));
        // out.print_line(&a);
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
