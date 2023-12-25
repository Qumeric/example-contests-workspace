//{"name":"D. Distance in Tree","group":"Codeforces - VK Cup 2012 Round 1","url":"https://codeforces.com/problemset/problem/161/D","interactive":false,"timeLimit":3000,"tests":[{"input":"5 2\n1 2\n2 3\n3 4\n2 5\n","output":"4\n"},{"input":"5 3\n1 2\n2 3\n3 4\n4 5\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DDistanceInTree"}}}

use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

// TODO: not done at all, only input
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n: usize = input.read(); // <= 50k
    let k: usize = input.read(); // <= 500

    let mut tree = Graph::<BiEdge<()>>::new(n);

    for _ in 1..n {
        let a: usize = input.read();
        let b: usize = input.read();

        tree.add_edge(BiEdge::new(a - 1, b - 1));
    }

    assert!(tree.is_tree());

    // You are given a tree with n vertices and a positive number k.
    // Find the number of distinct pairs of the vertices which have a distance of exactly k between them.
    // Note that pairs (v, u) and (u, v) are considered to be the same pair.
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
