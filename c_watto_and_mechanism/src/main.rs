//{"name":"C. Watto and Mechanism","group":"Codeforces - Codeforces Round 291 (Div. 2)","url":"https://codeforces.com/problemset/problem/514/C","interactive":false,"timeLimit":3000,"tests":[{"input":"2 3\naaaaa\nacacaca\naabaa\nccacacc\ncaaac\n","output":"YES\nNO\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CWattoAndMechanism"}}}

use std::collections::BTreeSet;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::random::random;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();

    let mut set: Vec<BTreeSet<u128>> = vec![Default::default(); 666_666];

    // Sophie Germain Prime
    let MOD: u128 = 145135534866431u128;

    let r = random();
    let base = (r.gen() % 1000 + 100) as u128;

    for i in 0..n {
        let s = input.read_str();
        let mut h: u128 = 0;

        for c in s.iter() {
            h = (h * base + (c - b'a' + 1) as u128) % MOD;
        }
        let mut m = 1u128;
        // The last letter has 0 degree
        for i in (0..s.len()).rev() {
            let c = if s[i] == b'a' {
                vec![1, 2]
            } else if s[i] == b'b' {
                vec![-1, 1]
            } else {
                vec![-2, -1]
            };

            for e in c {
                let val = if e > 0 {
                    let e = e as u128;
                    (h + 2 * MOD + e * m) % MOD
                } else {
                    let e: i32 = e;
                    let e = e.abs() as u128;
                    (h + 2 * MOD - e * m) % MOD
                };
                set[s.len()].insert(val);
            }
            m = (m * base) % MOD;
        }
    }

    for i in 0..m {
        let s = input.read_str();
        let mut h: u128 = 0;
        for c in s.iter() {
            h = (h * base + (c - b'a' + 1) as u128) % MOD;
        }
        out.print_line(set[s.len()].contains(&h));
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
    // don't forget to set test_type = Single if you do it
    // tester::stress_test(run, tester::check);
}
//END MAIN
