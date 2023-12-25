//{"name":"D. Roads not only in Berland","group":"Codeforces - Codeforces Beta Round 25 (Div. 2 Only)","url":"https://codeforces.com/problemset/problem/25/D","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n1 2\n","output":"0\n"},{"input":"7\n1 2\n2 3\n3 1\n4 5\n5 6\n6 7\n","output":"1\n3 1 3 7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DRoadsNotOnlyInBerland"}}}

use algo_lib::collections::dsu::DSU;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n: usize = input.read();

    let edges: Vec<(usize, usize)> = (1..n)
        .map(|_| (input.read::<usize>() - 1, input.read::<usize>() - 1))
        .collect();

    let mut dsu = DSU::new(n);
    let mut to_close: Vec<(usize, usize)> = Default::default();

    for (a, b) in edges {
        let a_root = dsu.get(a);
        let b_root = dsu.get(b);

        if a_root != b_root {
            dsu.join(a, b);
        } else {
            to_close.push((a, b));
        }
    }
    let mut to_close = to_close.into_iter();

    out.print_line(dsu.set_count() - 1);

    for (c1, c2) in dsu.iter().zip(dsu.iter().skip(1)) {
        if let Some((e1, e2)) = to_close.next() {
            out.print_line((e1 + 1, e2 + 1, c1 + 1, c2 + 1));
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
