//{"name":"E. Necklace Assembly","group":"Codeforces - Codeforces Round 650 (Div. 3)","url":"https://codeforces.com/problemset/problem/1367/E","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n6 3\nabcbac\n3 6\naaa\n7 1000\nabczgyo\n5 4\nababa\n20 10\naaebdbabdbbddaadaadc\n20 5\necbedececacbcbccbdec\n","output":"6\n3\n5\n4\n15\n10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ENecklaceAssembly"}}}

use std::cmp::min;

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let k = input.read_size();

    let s = input.read_str();

    let mut f = vec![0; 26];
    for c in s {
        f[(c - b'a') as usize] += 1;
    }

    f.sort();

    let mut divs = vec![];
    let mut i = 0;
    while i * i <= k {
        i += 1;
        if k % i != 0 {
            continue;
        }
        let j = k / i;
        divs.push(i);
        divs.push(j);
    }

    divs.sort();
    divs.dedup();

    let mut ans = 0;
    for len in divs.into_iter() {
        for cnt in 1..=2005 {
            let mut filled = 0;
            for c in f.iter() {
                filled += c / cnt;
            }
            if filled >= len {
                ans.maxim(len * cnt);
            }
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
