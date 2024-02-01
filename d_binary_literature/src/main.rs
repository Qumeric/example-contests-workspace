//{"name":"D. Binary Literature","group":"Codeforces - Codeforces Round 715 (Div. 2)","url":"https://codeforces.com/problemset/problem/1509/D","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n1\n00\n11\n01\n3\n011001\n111010\n010001\n","output":"010\n011001010\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DBinaryLiterature"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();

    let a = input.read_str();
    let b = input.read_str();
    let c = input.read_str();

    let mut a_ptr = 0;
    let mut b_ptr = 0;
    let mut c_ptr = 0;

    let mut ans = vec![];
    while a_ptr < a.len() && b_ptr < b.len() && c_ptr < c.len() {
        let mut v = vec![a[a_ptr], b[b_ptr], c[c_ptr]];
        v.sort();
        let v = v[1];

        ans.push(v);

        if a[a_ptr] == v {
            a_ptr += 1;
        }
        if b[b_ptr] == v {
            b_ptr += 1;
        }
        if c[c_ptr] == v {
            c_ptr += 1;
        }
    }

    let mut v = vec![(a_ptr, a), (b_ptr, b), (c_ptr, c)];
    v.sort();
    let v = v[1].clone();
    for i in v.0..(2 * n) {
        ans.push(v.1[i]);
    }

    while ans.len() < 3 * n {
        ans.push(b'0');
    }
    for x in ans {
        out.print(x - b'0');
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
