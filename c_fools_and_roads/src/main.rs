//{"name":"C. Fools and Roads","group":"Codeforces - Codeforces Round 121 (Div. 1)","url":"https://codeforces.com/problemset/problem/191/C","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n1 2\n1 3\n2 4\n2 5\n2\n1 4\n3 5\n","output":"2 1 1 1\n"},{"input":"5\n3 4\n4 5\n1 4\n2 4\n3\n2 3\n1 3\n3 5\n","output":"3 1 1 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CFoolsAndRoads"}}}

use algo_lib::collections::specs::PlusSum;
use algo_lib::collections::static_arq::StaticArq;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::hl_decomposition::HLDecomposition;
use algo_lib::graph::lca::{LCATrait, LCA};
use algo_lib::graph::magic_tree::{MagicTree, QueryKind};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n: usize = input.read();

    let edges: Vec<(usize, usize)> = (1..n)
        .map(|_| (input.read::<usize>() - 1, input.read::<usize>() - 1))
        .collect();

    let tree = Graph::from_biedges(n, &edges);

    let mut magic_tree: MagicTree<PlusSum> = MagicTree::new(&tree, QueryKind::Edge);

    let k: usize = input.read();
    for _ in 0..k {
        let (a, b) = (input.read::<usize>() - 1, input.read::<usize>() - 1);

        magic_tree.update(a, b, &1);
    }

    out.print_line(magic_tree.get_edges(&edges));
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
