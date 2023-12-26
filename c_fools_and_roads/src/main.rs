//{"name":"C. Fools and Roads","group":"Codeforces - Codeforces Round 121 (Div. 1)","url":"https://codeforces.com/problemset/problem/191/C","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n1 2\n1 3\n2 4\n2 5\n2\n1 4\n3 5\n","output":"2 1 1 1\n"},{"input":"5\n3 4\n4 5\n1 4\n2 4\n3\n2 3\n1 3\n3 5\n","output":"3 1 1 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CFoolsAndRoads"}}}

use algo_lib::collections::specs::PlusSum;
use algo_lib::collections::static_arq::StaticArq;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::hl_decomposition::HLDecomposition;
use algo_lib::graph::lca::{LCATrait, LCA};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n: usize = input.read();

    let edges: Vec<(usize, usize)> = (1..n)
        .map(|_| (input.read::<usize>() - 1, input.read::<usize>() - 1))
        .collect();

    let tree = Graph::from_biedges(n, &edges);

    // It could be solved in easier way with eulerian tour but I am trying HLD to learn to use it
    let lca = tree.lca();
    let (hld_paths, hld_id, hld_pos) = tree.hl_decomposition();

    let mut seg_trees: Vec<_> = hld_paths
        .iter()
        .map(|p| StaticArq::<PlusSum>::new(&vec![0; p.len()]))
        .collect();

    let k: usize = input.read();
    for _ in 0..k {
        let (mut a, mut b) = (input.read::<usize>() - 1, input.read::<usize>() - 1);

        let cur_lca = lca.lca(a, b);
        // out.print_line(("query:", a, b, "lca: ", cur_lca));

        let mut update_path = |x: &mut usize| {
            while hld_id[*x] != hld_id[cur_lca] {
                let path_id = hld_id[*x];
                let cur_pos = hld_pos[*x];

                let path = &mut seg_trees[path_id];
                path.update(0, cur_pos, &1);
                // out.print_line(("updating", path_id, "to", cur_pos));

                *x = lca.parent(hld_paths[path_id][0]).unwrap();
            }
            if *x != cur_lca {
                // Start updating from the node after the LCA
                seg_trees[hld_id[*x]].update(hld_pos[cur_lca] + 1, hld_pos[*x], &1);
            }
        };

        update_path(&mut a);
        update_path(&mut b);
    }

    let vertex_ans: Vec<i64> = (0..n)
        .map(|i| seg_trees[hld_id[i]].query_point(hld_pos[i]))
        .collect();

    // out.print_line(("vertex_ans", &vertex_ans));
    let ans: Vec<i64> = edges
        .into_iter()
        .map(|(u, v)| {
            let child = if lca.level(u) > lca.level(v) { u } else { v };
            vertex_ans[child]
        })
        .collect();

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
