//{"name":"B. Digits Permutations","group":"Codeforces - Codeforces Beta Round 99 (Div. 1)","url":"https://codeforces.com/contest/138/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"198\n","output":"981\n819\n"},{"input":"500\n","output":"500\n500\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BDigitsPermutations"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_str();
    let mut cnts = vec![0; 10];

    for i in n {
        let c = i - b'0';
        cnts[c as usize] += 1;
    }

    let mut ans1 = vec![];
    let mut ans2 = vec![];

    while cnts[0] > 0 {
        cnts[0] -= 1;
        ans1.push(0);
        ans2.push(0);
    }
    while cnts[5] > 0 {
        cnts[5] -= 1;
        ans1.push(5);
        ans2.push(5);
    }
    for i in 1..5 {
        if cnts[i] > 0 && cnts[10 - i] > 0 {
            cnts[i] -= 1;
            cnts[10 - i] -= 1;
            ans1.push(i);
            ans2.push(10 - i);
            ans1.push(i);
            ans2.push(10 - i);
        }
    }
    for i in 0..9 {
        while cnts[i] > 0 {
            ans1.push(i);
            ans2.push(i);
            cnts[i] -= 1;
        }
    }
    ans1.reverse();
    ans2.reverse();
    for i in ans1 {
        out.print(i);
    }
    out.print_line("");
    for i in ans2 {
        out.print(i);
    }
    out.print_line("");
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
