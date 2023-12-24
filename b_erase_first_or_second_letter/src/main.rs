//{"name":"B. Erase First or Second Letter","group":"Codeforces - Codeforces Round 917 (Div. 2)","url":"https://codeforces.com/contest/1917/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n5\naaaaa\n1\nz\n5\nababa\n14\nbcdaaaabcdaaaa\n20\nabcdefghijklmnopqrst\n","output":"5\n1\n9\n50\n210\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BEraseFirstOrSecondLetter"}}}

use std::collections::HashSet;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n: usize = input.read();
    let s: String = input.read_line();
    let s: Vec<char> = s.chars().collect();

    let mut ans = 1;

    let mut unique: HashSet<char> = Default::default();
    unique.insert(s[0]);

    for i in 1..n {
        unique.insert(s[i]);
        ans += unique.len();
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
