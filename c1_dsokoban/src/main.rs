//{"name":"C. 1D Sokoban","group":"Codeforces - Educational Codeforces Round 105 (Rated for Div. 2)","url":"https://codeforces.com/problemset/problem/1494/C","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n5 6\n-1 1 5 11 15\n-4 -3 -2 6 7 15\n2 2\n-1 1\n-1000000000 1000000000\n2 2\n-1000000000 1000000000\n-1 1\n3 5\n-1 1 2\n-2 -1 1 2 5\n2 1\n1 2\n10\n","output":"4\n2\n0\n3\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"C1DSokoban"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

// TODO: need be a bit careful
fn solve1(a: Vec<i64>, b: Vec<i64>) -> usize {
    if a.is_empty() || b.is_empty() {
        return 0;
    }
    let st = 0;
    let mut ans = 0;
    let fst_match = 0;
    let last_match = 0;
    for i in 0..b.len() {
        if fst_match == a.len() {
            break;
        }
        while fst_match < a.len() && a[fst_match] < b[i] {
            fst_match += 1;
        }
        if fst_match == a.len() {
            break;
        }
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();

    let a = input.read_long_vec(n);
    let b = input.read_long_vec(m);

    let mut aa = vec![];
    let mut bb = vec![];
    let mut ans = 0;
    for i in 0..n {
        if a[i] < 0 {
            aa.push(-a[i]);
        }
    }
    for i in 0..m {
        if b[i] < 0 {
            bb.push(-b[i]);
        }
    }

    aa.sort();
    bb.sort();
    ans += solve1(aa, bb);
    let mut aa = vec![];
    let mut bb = vec![];
    let mut ans = 0;
    for i in 0..n {
        if a[i] > 0 {
            aa.push(a[i]);
        }
    }
    for i in 0..m {
        if b[i] > 0 {
            bb.push(b[i]);
        }
    }
    aa.sort();
    bb.sort();
    ans += solve1(aa, bb);
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
