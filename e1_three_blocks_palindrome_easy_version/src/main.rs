//{"name":"E1. Three Blocks Palindrome (easy version)","group":"Codeforces - Codeforces Round 634 (Div. 3)","url":"https://codeforces.com/contest/1335/problem/E1","interactive":false,"timeLimit":3000,"tests":[{"input":"6\n8\n1 1 2 2 3 2 1 1\n3\n1 3 3\n4\n1 10 10 1\n1\n26\n2\n2 1\n3\n1 1 1\n","output":"7\n2\n4\n1\n1\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"E1ThreeBlocksPalindromeEasyVersion"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let mut pos = vec![vec![]; 201];
    let mut ptrs = vec![0; 201];

    let mut sums: Vec<Vec<i32>> = vec![vec![0; n]; 201];

    for i in 0..n {
        pos[a[i]].push(i);
        sums[a[i]][i] += 1;
    }

    for x in 1..=200 {
        for i in 1..n {
            sums[x][i] += sums[x][i - 1];
        }
    }

    // inclusive
    let get = |x: usize, from: usize, to: usize| -> i32 {
        sums[x][to] - if from == 0 { 0 } else { sums[x][from - 1] }
    };

    let mut ans = 0;

    // empty first and last blocks
    for x in 1..=200 {
        ans.maxim(sums[x][n - 1]);
    }

    for i in 0..n {
        let e = a[i];
        let start_pos = ptrs[e];
        if pos[e].len() <= start_pos * 2 + 1 {
            continue;
        }
        let end_pos = pos[e].len() - 1 - start_pos;
        let last_possible = pos[e][end_pos] - 1;
        if last_possible != i {
            let mut max_mid = 0;
            for x in 1..=200 {
                max_mid.maxim(get(x, i + 1, last_possible));
            }
            ans.maxim(((start_pos + 1) * 2) as i32 + max_mid);
        }

        ptrs[e] += 1;
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
