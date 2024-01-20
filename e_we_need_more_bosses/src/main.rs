//{"name":"E. We Need More Bosses","group":"Codeforces - Educational Codeforces Round 46 (Rated for Div. 2)","url":"https://codeforces.com/contest/1000/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"5 5\n1 2\n2 3\n3 1\n4 1\n5 2\n","output":"2\n"},{"input":"4 3\n1 2\n4 3\n3 2\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EWeNeedMoreBosses"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::graph::bridges::BridgeSearch;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::strongly_connected_components::StronglyConnectedComponents;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let edges = (0..m).map(|_| (input.read1(), input.read1())).collect_vec();
    let g = Graph::from_biedges(n, &edges);

    let bridges = g.biconnected_tree();

    let diameter = bridges.find_diameter_path();

    out.print_line(diameter.len() - 1);
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
