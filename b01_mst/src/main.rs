//{"name":"B. 0-1 MST","group":"Codeforces - Codeforces Round 599 (Div. 1)","url":"https://codeforces.com/problemset/problem/1242/B","interactive":false,"timeLimit":1000,"tests":[{"input":"6 11\n1 3\n1 4\n1 5\n1 6\n2 3\n2 4\n2 5\n2 6\n3 4\n3 5\n3 6\n","output":"2\n"},{"input":"3 0\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"B01MST"}}}

use std::cmp::min;

use algo_lib::collections::dsu::DSU;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();

    let mut g = vec![vec![]; n];

    for i in 0..m {
        let a = input.read1();
        let b = input.read1();
        g[a].push(b);
        g[b].push(a);
    }
    for i in 0..n {
        g[i].sort();
    }
    let mut pairs: Vec<(usize, usize)> = (0..n).map(|i| (g[i].len(), i)).collect();
    pairs.sort();
    pairs.reverse();

    let mut dsu = DSU::new(n);

    for i in 0..min(n, 1000) {
        let v = pairs[i].1;
        let mut ptr = 0;

        // out.print_line(("V", v));
        for u in 0..n {
            if u == v {
                continue;
            }
            if ptr < g[v].len() && u == g[v][ptr] {
                ptr += 1;
                continue;
            }
            // out.print_line(("join", v, u));
            dsu.join(v, u);
        }
    }
    out.print_line(dsu.set_count() - 1);
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
