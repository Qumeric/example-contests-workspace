//{"name":"D - Super Takahashi Bros.","group":"AtCoder - KAJIMA CORPORATION CONTEST 2024（AtCoder Beginner Contest 340）","url":"https://atcoder.jp/contests/abc340/tasks/abc340_d","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n100 200 3\n50 10 1\n100 200 5\n150 1 2\n","output":"350\n"},{"input":"10\n1000 10 9\n1000 10 10\n1000 10 2\n1000 10 3\n1000 10 4\n1000 10 5\n1000 10 6\n1000 10 7\n1000 10 8\n","output":"90\n"},{"input":"6\n1000000000 1000000000 1\n1000000000 1000000000 1\n1000000000 1000000000 1\n1000000000 1000000000 1\n1000000000 1000000000 1\n","output":"5000000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DSuperTakahashiBros"}}}

use algo_lib::graph::distances::Distances;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();

    let mut edges = vec![];
    for stage in 0..(n - 1) {
        let wa = input.read_size();
        let wb = input.read_size();
        let go_to = input.read1();
        edges.push((stage, stage + 1, wa));
        edges.push((stage, go_to, wb));
    }

    let g = Graph::from_weighted_edges(n, &edges);

    let result = g.distance(0, n - 1);
    out.print_line(result.unwrap().0);
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
