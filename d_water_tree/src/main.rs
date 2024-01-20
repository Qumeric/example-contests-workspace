//{"name":"D. Water Tree","group":"Codeforces - Codeforces Round 200 (Div. 1)","url":"https://codeforces.com/problemset/problem/343/D","interactive":false,"timeLimit":4000,"tests":[{"input":"5\n1 2\n5 1\n2 3\n4 2\n12\n1 1\n2 3\n3 1\n3 2\n3 3\n3 4\n1 2\n2 4\n3 1\n3 3\n3 4\n3 5\n","output":"0\n0\n0\n1\n0\n1\n0\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DWaterTree"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::specs::ArqSpec;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::path_query_tree::{PathQueryTree, QueryKind};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

pub enum AssignMax {}
impl ArqSpec for AssignMax {
    type S = (i32, i32);
    type F = (i32, i32);
    fn op(&a: &Self::S, &b: &Self::S) -> Self::S {
        (a.0.max(b.0), a.1.max(b.1))
    }
    fn identity() -> Self::S {
        (i32::MIN, i32::MIN)
    }
    fn compose(&f: &Self::F, g: &Self::F) -> Self::F {
        (f.0.max(g.0), f.1.max(g.1))
    }
    fn apply(&f: &Self::F, &g: &Self::S, _: i64) -> Self::S {
        (f.0.max(g.0), f.1.max(g.1))
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();

    let edges = (1..n).map(|_| (input.read1(), input.read1())).collect_vec();
    let g = Graph::from_biedges(n, &edges);

    let mut tree = PathQueryTree::<AssignMax>::new(&g, QueryKind::Vertex);
    drop(g);
    drop(edges);

    let q = input.read_size();
    for i in 1..=q {
        let c = input.read_size();
        let v = input.read1();
        let i = i as i32;
        match c {
            1 => {
                tree.update(v, v, &(i, 0));
            }
            2 => {
                tree.update(0, v, &(0, i));
            }
            3 => {
                let fill_time = tree.query(0, v).0;
                let empty_time = tree.query(v, v).1;

                if fill_time > empty_time {
                    out.print_line(1);
                } else {
                    out.print_line(0);
                }
            }
            _ => unimplemented!(),
        }
        // let after_fill = tree_fill.get_vertexes();
        // out.print_line(after_fill);
        // let after_empty = tree_empty.get_vertexes();
        // out.print_line(after_empty);
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
