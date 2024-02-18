//{"name":"D1. Sum over all Substrings (Easy Version)","group":"Codeforces - think-cell Round 1","url":"https://codeforces.com/contest/1930/problem/D1","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1\n1\n2\n10\n5\n00000\n20\n11110110000000111111\n","output":"1\n2\n0\n346\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"D1SumOverAllSubstringsEasyVersion"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let s = input
        .read_str()
        .iter()
        .map(|e| if e == b'1' { 1 } else { 0 })
        .collect_vec();

    let calc = |a, b| {
        let mut ans = 0;
        // Don't forget the last
        let mut i = a;
        while i <= b {
            if s[i] == 1 {
                ans += 1;
                i += 3;
            } else {
                i += 1;
            }
        }
        ans
    };

    // last can cover itself or cover with +1
    //

    let mut dp = Arr2d::new(n, 3);

    // maybe can consider only even strings?
    let mut ans = 0;
    for i in 0..n {
        for j in i..n {
            ans += calc(i, j);
        }
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
