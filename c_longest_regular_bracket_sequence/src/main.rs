//{"name":"C. Longest Regular Bracket Sequence","group":"Codeforces - Codeforces Beta Round 5","url":"https://codeforces.com/problemset/problem/5/C","interactive":false,"timeLimit":2000,"tests":[{"input":")((())))(()())\n","output":"6 2\n"},{"input":"))(\n","output":"0 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CLongestRegularBracketSequence"}}}

use std::collections::BTreeMap;

use algo_lib::collections::specs::AssignMin;
use algo_lib::collections::static_arq::StaticArq;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

// TODO: this is not quite correct but close.
// The problem is that it considers sequences with too many '(' as correct ones.
// Could be modified to binary search only among correct endings.
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let s = input.read_str();
    let n = s.len();
    let mut segtree = StaticArq::<AssignMin>::new(&vec![0; n]);
    let mut cur = 0;

    for i in 0..n {
        if s[i] == b'(' {
            cur += 1;
        } else {
            cur -= 1;
        }
        segtree.update_point(i, &cur);
    }

    let mut ans: BTreeMap<usize, usize> = Default::default();

    for i in 0..n {
        if s[i] != b'(' {
            continue;
        }
        let first = segtree.query_point(i);

        let postfix_min = segtree.query(i, n - 1);
        if postfix_min >= first {
            // it never closes
            continue;
        }

        let mut l = 0;
        let mut r = (n - i) + 1;

        while l + 1 < r {
            let mid = (l + r) / 2;

            let min = segtree.query(i, i + mid - 1);

            if min < first - 1 {
                r = mid;
            } else {
                l = mid;
            }
        }

        if r > 1 {
            out.print_line((i, l));
        }
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
}
//END MAIN
