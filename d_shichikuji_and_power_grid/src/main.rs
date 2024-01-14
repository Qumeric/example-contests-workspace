//{"name":"D. Shichikuji and Power Grid","group":"Codeforces - Codeforces Round 597 (Div. 2)","url":"https://codeforces.com/problemset/problem/1245/D","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n2 3\n1 1\n3 2\n3 2 3\n3 2 3\n","output":"8\n3\n1 2 3\n0\n"},{"input":"3\n2 1\n1 2\n3 3\n23 2 23\n3 2 3\n","output":"27\n1\n2\n2\n1 2\n2 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DShichikujiAndPowerGrid"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::minimal_spanning_tree::MinimalSpanningTree;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();

    let cities = (0..n)
        .map(|_| (input.read_size(), input.read_size()))
        .collect_vec();
    let costs = input.read_size_vec(n);
    let ks = input.read_size_vec(n);

    let mut edges = vec![];

    for i in 0..n {
        for j in (i + 1)..n {
            let dist = cities[i].0.abs_diff(cities[j].0) + cities[i].1.abs_diff(cities[j].1);
            edges.push((i + 1, j + 1, dist * (ks[i] + ks[j])));
        }
    }
    for i in 0..n {
        edges.push((0, i + 1, costs[i]));
    }

    let graph = Graph::from_weighted_biedges(n + 1, &edges);

    let mst = graph.minimal_spanning_tree().into_egdes();
    let mut ans = 0;

    let mut ps = vec![];
    let mut cs = vec![];

    for (from, v) in mst.into_iter().enumerate() {
        for e in v.into_iter() {
            let to = e.to();
            if to <= from {
                continue;
            }
            if from == 0 {
                ps.push(to);
            } else {
                cs.push((from, to));
            }
            ans += e.weight();
        }
    }

    out.print_line(ans);
    out.print_line(ps.len());
    out.print_line(ps);
    out.print_line(cs.len());
    for c in cs {
        out.print_line(c);
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
