//{"name":"D. Count Good Substrings","group":"Codeforces - Codeforces Round 258 (Div. 2)","url":"https://codeforces.com/problemset/problem/451/D","interactive":false,"timeLimit":2000,"tests":[{"input":"bb\n","output":"1 2\n"},{"input":"baab\n","output":"2 4\n"},{"input":"babb\n","output":"2 5\n"},{"input":"babaa\n","output":"2 7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DCountGoodSubstrings"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::specs::AssignSum;
use algo_lib::collections::static_arq::StaticArq;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve1(s: Vec<u8>) -> (i64, i64) {
    let n = s.len();
    let mut odd_tree = StaticArq::<AssignSum>::new(&vec![0; n]);
    let mut even_tree = StaticArq::<AssignSum>::new(&vec![0; n]);

    for i in 0..n {
        let is_odd = i % 2 == 1;
        let is_a = s[i] == b'a';
        if is_a {
            if is_odd {
                odd_tree.update_point(i, &1);
            } else {
                even_tree.update_point(i, &1);
            }
        }
    }

    let mut a = 0;
    let mut b = 0;
    for i in 0..n {
        let is_a = s[i] == b'a';
        let is_odd = i % 2 == 1;

        if is_a {
            if is_odd {
                a += odd_tree.query(i, n - 1);
                b += even_tree.query(i, n - 1);
            } else {
                b += odd_tree.query(i, n - 1);
                a += even_tree.query(i, n - 1);
            }
        }
    }

    // println!("{a} {b}");
    (a, b)
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let s = input.read_str().into_iter().collect_vec();
    let r = s
        .clone()
        .into_iter()
        .map(|x| if x == b'a' { b'b' } else { b'a' })
        .collect_vec();

    let a1 = solve1(r);
    let a2 = solve1(s);
    out.print_line((a1.1 + a2.1, a1.0 + a2.0));
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
