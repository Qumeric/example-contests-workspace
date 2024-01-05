//{"name":"G. Reducing Delivery Cost","group":"Codeforces - Codeforces Round 677 (Div. 3)","url":"https://codeforces.com/problemset/problem/1433/G","interactive":false,"timeLimit":1000,"tests":[{"input":"6 5 2\n1 2 5\n2 3 7\n2 4 4\n4 5 2\n4 6 8\n1 6\n5 3\n","output":"22\n"},{"input":"5 5 4\n1 2 5\n2 3 4\n1 4 3\n4 3 7\n3 5 2\n1 5\n1 3\n3 3\n1 5\n","output":"13\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GReducingDeliveryCost"}}}

use std::cmp::min;

use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::graph::distances::Distances;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();

    let edges: Vec<_> = (0..m)
        .map(|_| {
            let x = input.read1();
            let y = input.read1();
            let w = input.read_u64();
            (x, y, w)
        })
        .collect();

    let routes: Vec<_> = (0..k)
        .map(|_| {
            let a = input.read1();
            let b = input.read1();
            (a, b)
        })
        .collect();

    let mut ans = u64::MAX;

    let graph = Graph::from_weighted_biedges(n, &edges);
    let mut orig_dist = vec![0; k];
    for (i, &(a, b)) in routes.enumerate() {
        orig_dist[i] = graph.distance(a, b).unwrap().0;
    }

    for i in 0..m {
        let mut new_edges = edges.clone();
        new_edges[i].2 = 0;
        let graph = Graph::from_weighted_biedges(n, &new_edges);
        let (x, y, _) = new_edges[i].clone();
        let distances_from = graph.distances_from(x);
        let distances_to = graph.distances_from(y);

        let mut cans = 0;
        for (i, &(a, b)) in routes.enumerate() {
            cans += min(
                orig_dist[i],
                min(
                    distances_from[a].unwrap().0 + distances_to[b].unwrap().0,
                    distances_from[b].unwrap().0 + distances_to[a].unwrap().0,
                ),
            );
        }
        ans = min(ans, cans);
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
}
//END MAIN
