//{"name":"E. Tree with Small Distances","group":"Codeforces - Codeforces Round 506 (Div. 3)","url":"https://codeforces.com/problemset/problem/1029/E","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n1 2\n2 3\n2 4\n4 5\n4 6\n5 7\n","output":"2\n"},{"input":"7\n1 2\n1 3\n2 4\n2 5\n3 6\n1 7\n","output":"0\n"},{"input":"7\n1 2\n2 3\n3 4\n3 5\n3 6\n3 7\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ETreeWithSmallDistances"}}}

use std::collections::BTreeSet;

use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

// TODO: something is not quite right...
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();

    let mut covered: BTreeSet<usize> = Default::default();
    covered.insert(0);
    let mut g = vec![BTreeSet::<usize>::new(); n];
    let mut edges = vec![];
    for i in 1..n {
        let a = input.read1();
        let b = input.read1();
        if a == 0 {
            covered.insert(b);
        } else if b == 0 {
            covered.insert(a);
        } else {
            g[a].insert(b);
            g[b].insert(a);
            edges.push((a, b));
        }
    }

    let gr = Graph::from_biedges(n, &edges);
    let mut depth = vec![0; n];
    gr.dfs(0, |p, v| {
        if let Some(p) = p {
            depth[v] = depth[p] + 1;
        }
    });

    let mut leaves: BTreeSet<(usize, usize)> = Default::default();
    for i in 0..n {
        if g[i].len() == 1 && !covered.contains(&i) {
            leaves.insert((depth[i], i));
        }
    }

    let mut ans = 0;
    while let Some((d, leaf)) = leaves.pop_first() {
        if covered.contains(&leaf) {
            continue;
        }
        if let Some(&p) = g[leaf].first().clone() {
            if covered.contains(&p) {
                continue;
            }

            for &v in g[p].clone().iter() {
                g[v].remove(&p);
                if g[v].len() == 1 {
                    leaves.insert((depth[v], v));
                }
            }

            g[p].clear();

            covered.insert(p);
            ans += 1;
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
