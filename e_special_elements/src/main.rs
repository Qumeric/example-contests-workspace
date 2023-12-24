//{"name":"E. Special Elements","group":"Codeforces - Codeforces Round 640 (Div. 4)","url":"https://codeforces.com/problemset/problem/1352/E","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n9\n3 1 4 1 5 9 2 6 5\n3\n1 1 2\n5\n1 1 1 1 1\n8\n8 7 6 5 4 3 2 1\n1\n1\n","output":"5\n1\n0\n4\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ESpecialElements"}}}

use std::collections::HashSet;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n: usize = input.read();
    let a: Vec<i32> = input.read_int_vec(n);

    let mut prefix_sums = vec![0; n + 1];

    for i in 0..n {
        prefix_sums[i + 1] = prefix_sums[i] + a[i];
    }

    let a_set: HashSet<_> = a.clone().into_iter().collect();

    let mut specials: HashSet<_> = Default::default();

    for l in 0..n {
        for r in l + 1..n {
            let sum = prefix_sums[r + 1] - prefix_sums[l];
            if a_set.contains(&sum) {
                specials.insert(sum);
            }
        }
    }

    let ans = a.into_iter().filter(|e| specials.contains(e)).count();

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
