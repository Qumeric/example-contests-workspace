//{"name":"C. Did We Get Everything Covered?","group":"Codeforces - Codeforces Round 921 (Div. 2)","url":"https://codeforces.com/contest/1925/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n2 2 4\nabba\n2 2 3\nabb\n3 3 10\naabbccabab\n","output":"YES\nNO\naa\nNO\nccc\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CDidWeGetEverythingCovered"}}}

use std::collections::BTreeSet;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let m = input.read_size();

    let s = input.read_str();

    // find string of lenght n using first k letters which doesn't occur as subseq of s

    let mut chars: BTreeSet<u8> = Default::default();

    let mut str = vec![];

    for i in 0..m {
        chars.insert(s[i]);
        if chars.len() == k {
            str.push(s[i] as char);
            chars.clear();
        }
    }

    if str.len() >= n {
        out.print_line("YES");
    } else {
        out.print_line("NO");
        let mut cnt = 0;
        for x in str {
            out.print(x);
            cnt += 1;
        }
        for i in b'a'..=b'z' {
            if !chars.contains(&i) {
                while cnt < n {
                    out.print(i as char);
                    cnt += 1;
                }
                out.print_line("");
                return;
            }
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
