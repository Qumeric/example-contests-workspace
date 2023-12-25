//{"name":"E. Tree Queries","group":"Codeforces - Codeforces Round 629 (Div. 3)","url":"https://codeforces.com/problemset/problem/1328/E","interactive":false,"timeLimit":2000,"tests":[{"input":"10 6\n1 2\n1 3\n1 4\n2 5\n2 6\n3 7\n7 8\n7 9\n9 10\n4 3 8 9 10\n3 2 4 6\n3 2 1 5\n3 4 8 2\n2 6 10\n3 5 4 7\n","output":"YES\nYES\nYES\nYES\nNO\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ETreeQueries"}}}

use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::lca::LCATrait;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n: usize = input.read();
    let m: usize = input.read();

    let mut tree = Graph::<BiEdge<()>>::new(n);

    for _ in 1..n {
        let (a, b): (usize, usize) = input.read();
        tree.add_edge(BiEdge::new(a - 1, b - 1));
    }

    assert!(tree.is_tree());

    let lca = tree.lca_with_root(0);

    for i in 0..m {
        let k: usize = input.read();
        let ks: Vec<usize> = (0..k)
            .into_iter()
            .map(|_| input.read::<usize>() - 1)
            .collect();

        let lowest = *ks
            .iter()
            .max_by(|&a, &b| lca.level(*a).cmp(&lca.level(*b)))
            .unwrap();

        let mut found = false;
        for k in ks {
            let cur_lca = lca.lca(lowest, k);
            if lca.level(cur_lca) + 1 < lca.level(k) {
                out.print_line(false);
                found = true;
                break;
            }
        }
        if !found {
            out.print_line(true);
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
