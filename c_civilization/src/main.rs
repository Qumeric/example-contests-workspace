//{"name":"C. Civilization","group":"Codeforces - Codeforces Round 260 (Div. 1)","url":"https://codeforces.com/problemset/problem/455/C","interactive":false,"timeLimit":1000,"tests":[{"input":"6 0 6\n2 1 2\n2 3 4\n2 5 6\n2 3 2\n2 5 3\n1 1\n","output":"4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CCivilization"}}}

use std::cmp::max;

use algo_lib::collections::dsu::DSU;
use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{
    Callable, Callable3, RecursiveFunction, RecursiveFunction2, RecursiveFunction3,
};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let q = input.read_size();

    let mut dsu = DSU::new(n);
    let edges = (0..m).map(|_| (input.read1(), input.read1())).collect_vec();

    for &(a, b) in &edges {
        dsu.join(a, b);
    }

    let g = Graph::from_biedges(n, &edges);

    let mut dfs = RecursiveFunction3::new(|f, parent, node, depth| {
        let mut val = (depth, node);
        for e in g[node].iter() {
            if e.to() != parent {
                val = max(val, f.call(node, e.to(), depth + 1));
            }
        }

        val
    });

    let mut find_diam = RecursiveFunction::new(|f, node| {
        let deep1 = dfs.call(node, node, 0);
        let deep2 = dfs.call(deep1.1, deep1.1, 0);

        deep2.0
    });

    let mut colors = vec![0; n];
    for i in 0..n {
        let c = dsu.get(i);
        if colors[c] == 0 {
            colors[c] = find_diam.call(i);
            // out.print_line(("COLOR", c, colors[c]));
        }
    }

    for i in 0..q {
        let t = input.read_size();
        if t == 1 {
            let x = input.read1();
            out.print_line(colors[dsu.get(x)]);
            continue;
        }

        let a = dsu.get(input.read1());
        let b = dsu.get(input.read1());

        let merged = dsu.join(a, b);
        if merged {
            let v = dsu.get(a);
            let old_max = max(colors[a], colors[b]);
            colors[v] = max(old_max, (colors[a] + 1) / 2 + (colors[b] + 1) / 2 + 1);

            // out.print_line(("AFTER JOIN", a, b, "DIAM", colors[v]));
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
    // don't forget to set test_type = Single if you do it
    // tester::stress_test(run, tester::check);
}
//END MAIN
