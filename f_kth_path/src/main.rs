//{"name":"F. K-th Path","group":"Codeforces - Codeforces Round 575 (Div. 3)","url":"https://codeforces.com/contest/1196/problem/F","interactive":false,"timeLimit":2500,"tests":[{"input":"6 10 5\n2 5 1\n5 3 9\n6 2 2\n1 3 1\n5 1 8\n6 5 10\n1 6 5\n6 4 6\n3 6 2\n3 4 5\n","output":"3\n"},{"input":"7 15 18\n2 6 3\n5 7 4\n6 5 4\n3 6 9\n6 7 7\n1 6 4\n7 1 6\n7 2 1\n4 3 2\n3 2 8\n5 3 6\n2 5 5\n3 7 9\n4 1 8\n2 1 1\n","output":"9\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FKThPath"}}}

use std::collections::BTreeSet;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();

    // given undirected weighted grap, find kth shortest path. k <= 400

    // smallest shortest path is just a smallest edge. At each step we extend one of existing paths
    // size is never > 400, at first take 400 smallest edges

    // maybe one problem is that (i, j) = (j, i) but we can keep 800?

    let mut g = vec![vec![]; n];

    let mut state = vec![];
    for i in 0..m {
        let edge = (input.read1(), input.read1(), input.read_size());
        g[edge.0].push((edge.1, edge.2));
        g[edge.1].push((edge.0, edge.2));
        state.push((edge.2, edge.0, edge.1, false));
        state.push((edge.2, edge.1, edge.0, false));
    }

    let mut result = usize::MAX;
    for _ in 0..k {
        state.sort();
        let mut unique_paths = BTreeSet::new();
        state.retain(|&(weight, from, to, handled)| unique_paths.insert((from, to)));

        while state.len() > k * 2 {
            result = state.pop().unwrap().0;
        }
        let state_len = state.len();
        for i in 0..state_len {
            if state[i].3 {
                continue;
            }
            let tot_w = state[i].0;
            let from = state[i].1;
            let cur = state[i].2;
            for &(to, w) in &g[cur] {
                let new_tot_w = tot_w + w;
                if new_tot_w < result && from != to {
                    state.push((new_tot_w, from, to, false));
                }
            }
            state[i].3 = true;
        }
    }

    // for path in state.iter() {
    //     out.print_line(("(from, to, w)", path.1, path.2, path.0));
    // }

    out.print_line(state[k * 2 - 1].0);
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
