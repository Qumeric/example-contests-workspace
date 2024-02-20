//{"name":"D. AB-string","group":"Codeforces - Educational Codeforces Round 74 (Rated for Div. 2)","url":"https://codeforces.com/problemset/problem/1238/D","interactive":false,"timeLimit":2000,"tests":[{"input":"5\nAABBB\n","output":"6\n"},{"input":"3\nAAA\n","output":"3\n"},{"input":"7\nAAABABB\n","output":"15\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DABString"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn calc(s: &Vec<u8>) -> usize {
    // ABBB + BAAA - AB

    // number of ABBB -> number of BS except for BS at the beginning

    let a_cnt = s.iter().filter(|&x| *x == b'A').count();
    let b_cnt = s.len() - a_cnt;
    if a_cnt == 0 || b_cnt == 0 {
        return 0;
    }

    let mut i = 0;
    while s[i] == b'B' {
        i += 1;
    }
    let abbb = b_cnt - i;

    let mut i = 0;
    while s[i] == b'A' {
        i += 1;
    }
    let baaa = a_cnt - i;

    let mut ab = 0;
    for i in 0..(s.len() - 1) {
        if s[i] == b'A' && s[i + 1] == b'B' {
            ab += 1;
        }
    }
    abbb + baaa - ab
}

// fuck i was solving kinda wrong thing, missed that letters are A and B only......
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();

    let mut s = input.read_str().into_iter().collect_vec();

    let mut ans = n * (n - 1) / 2;

    ans -= calc(&s);
    s.reverse();
    ans -= calc(&s);
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
    // don't forget to set test_type = Single if you do it
    // tester::stress_test(run, tester::check);
}
//END MAIN
